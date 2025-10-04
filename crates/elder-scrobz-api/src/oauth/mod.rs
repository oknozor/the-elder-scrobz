use crate::error::AppError;
use crate::oauth::client::Oauth2Client;
use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderMap;
use elder_scrobz_db::user::{CreateUser, User};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use oauth2::basic::BasicTokenIntrospectionResponse;
use oauth2::TokenIntrospectionResponse;
use serde::{Deserialize, Serialize};

pub mod client;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    scrobz_role: Option<String>,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    User,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub name: String,
    pub role: UserRole,
}

impl AuthenticatedUser {
    fn from_introspection(value: BasicTokenIntrospectionResponse) -> Option<AuthenticatedUser> {
        value.active().then(|| {
            let name = value
                .sub()
                .expect("no sub claim in token introspection")
                .to_string();

            println!("{:?}", value.extra_fields());
            // Default role to User - JWT role extraction happens in from_request_parts
            let role = UserRole::User;

            AuthenticatedUser { name, role }
        })
    }

    fn from_jwt_token(token: &str) -> Option<AuthenticatedUser> {
        // For JWT validation, we need to be lenient since we can't verify signature
        // without the public key from the OIDC provider
        let mut validation = Validation::new(Algorithm::RS256);
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false; // We rely on introspection for expiry

        // Try to decode without signature validation to extract claims
        match decode::<JwtClaims>(token, &DecodingKey::from_secret(&[]), &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;
                println!("{:?}", claims);
                let role = match claims.scrobz_role.as_deref() {
                    Some("admin") => UserRole::Admin,
                    _ => UserRole::User,
                };

                Some(AuthenticatedUser {
                    name: claims.sub,
                    role,
                })
            }
            Err(err) => {
                println!("Error decoding JWT token: {:?}", err);
                None
            }
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn has_role(&self, role: &UserRole) -> bool {
        &self.role == role
    }
}

#[derive(Debug, Clone)]
pub struct AdminUser {
    pub user: AuthenticatedUser,
}

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = AuthenticatedUser::from_request_parts(parts, state).await?;

        if !user.is_admin() {
            return Err(AppError::Unauthorized("Admin role required".to_string()));
        }

        Ok(AdminUser { user })
    }
}

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let oauth2_client = parts
            .extensions
            .get::<Oauth2Client>()
            .expect("Missing Oauth2Client extension");

        let auth_value = bearer_from_headers(&parts.headers).or_else(|| match parts.uri.query() {
            Some(query) => bearer_from_query_params(query),
            None => None,
        });

        match auth_value {
            Some(token) => {
                // First validate token via introspection
                match oauth2_client.introspect(token.clone()).await {
                    Ok(response) => {
                        if !response.active() {
                            return Err(AppError::Unauthorized(
                                "Token is invalid or inactive".to_string(),
                            ));
                        }

                        // Try to extract role from JWT directly
                        let user = if let Some(jwt_user) = AuthenticatedUser::from_jwt_token(&token)
                        {
                            jwt_user
                        } else {
                            // Fallback to introspection result
                            AuthenticatedUser::from_introspection(response).ok_or_else(|| {
                                AppError::Unauthorized("Token is invalid or inactive".to_string())
                            })?
                        };

                        // Ensure user exists in database
                        let existing_user = User::get_by_username(&state.db, &user.name).await?;
                        if existing_user.is_none() {
                            CreateUser {
                                username: user.name.clone(),
                            }
                            .insert(&state.db)
                            .await?;
                        }
                        Ok(user)
                    }
                    Err(err) => Err(AppError::Unauthorized(err.to_string())),
                }
            }
            None => Err(AppError::Unauthorized("Missing Bearer Token".to_string())),
        }
    }
}

fn bearer_from_query_params(query: &str) -> Option<String> {
    let params: std::collections::HashMap<String, String> =
        url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();

    params.get("token").cloned()
}

fn bearer_from_headers(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("authorization")?;
    let bytes = header.as_bytes();
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value
            .strip_prefix("Bearer ")
            .map(|token: &str| token.to_string()),
        Err(_) => None,
    }
}
