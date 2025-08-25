use clap::Parser;
use elder_scrobz_api::api::ApiDoc;
use elder_scrobz_db::build_pg_pool;
use elder_scrobz_settings::Settings;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[clap(long, short)]
    out: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let settings = Settings::get()?;
    let settings = Arc::new(settings);
    let pool = build_pg_pool(&settings.database_url).await;
    let (_, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", elder_scrobz_api::api::router(true, pool))
        .split_for_parts();

    let openapi = openapi.to_pretty_json()?;

    match cli.out {
        Some(out) => {
            fs::write(&out, &openapi)?;
        }
        None => {
            println!("{openapi}");
        }
    }

    Ok(())
}
