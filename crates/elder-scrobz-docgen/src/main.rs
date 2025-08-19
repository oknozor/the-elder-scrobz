use clap::Parser;
use elder_scrobz_api::api::ApiDoc;
use std::fs;
use std::path::PathBuf;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[clap(long, short)]
    out: Option<PathBuf>,
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (_, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", elder_scrobz_api::api::router(true))
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
