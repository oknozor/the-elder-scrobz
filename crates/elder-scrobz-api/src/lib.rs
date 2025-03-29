pub mod api;
pub mod error;
pub mod oauth;
pub mod settings;

#[cfg(test)]
mod test_helper;

#[cfg(test)]
mod tests {
    use crate::api::router;
    use crate::test_helper::{scrobble_fixture, start_postgres};
    use axum::response::Response;
    use axum::{http::Request, http::StatusCode};
    use elder_scrobz_db::user::{CreateUser, User};
    use http_body_util::BodyExt;
    use speculoos::prelude::*;
    use tower::ServiceExt;

    #[tokio::test]
    async fn submit_listens() -> anyhow::Result<()> {
        let _container = start_postgres().await?;
        let (app, _) = router().split_for_parts();

        let body = serde_json::to_string(&CreateUser {
            username: "oknozor".to_string(),
            email: "paul.delafosse@protonmail.com".to_string(),
        })?;

        let request = Request::builder()
            .method("POST")
            .uri("/users")
            .header("Content-Type", "application/json")
            .body(body)?;

        let response = ServiceExt::oneshot(app.clone(), request).await?;
        assert_that!(response.status()).is_equal_to(StatusCode::OK);

        let body = body_to_string(response).await?;
        let user: User = serde_json::from_str(&body)?;

        let request = Request::builder()
            .method("POST")
            .uri(format!("/users/{}/api-key/create", user.username))
            .header("Content-Type", "application/json")
            .body(axum::body::Body::empty())?;

        let response = ServiceExt::oneshot(app.clone(), request).await?;
        assert_that!(response.status()).is_equal_to(StatusCode::OK);

        let body = body_to_string(response).await?;
        #[derive(serde::Deserialize)]
        struct ApiKeyCreated {
            api_key: String,
        }

        let api_key: ApiKeyCreated = serde_json::from_str(&body)?;

        let scrobble = scrobble_fixture()?;

        let request = Request::builder()
            .method("POST")
            .uri("/1/submit-listens")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Token {}", api_key.api_key))
            .body(scrobble)?;

        let response = ServiceExt::oneshot(app.clone(), request).await?;
        assert_that!(response.status()).is_equal_to(StatusCode::OK);

        Ok(())
    }

    async fn body_to_string(response: Response) -> anyhow::Result<String> {
        let body = response.into_body();
        let bytes = body.collect().await?.to_bytes();
        let body = String::from_utf8(bytes.to_vec())?;
        Ok(body)
    }
}
