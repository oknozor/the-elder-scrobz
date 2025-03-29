use clap::Parser;
use elder_scrobz_api::api::ApiDoc;
use std::fs::File;
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
        .nest("/api/v1", elder_scrobz_api::api::router())
        .split_for_parts();

    let openapi = openapi.to_pretty_json()?;

    match cli.out {
        Some(out) => {
            let file = File::open(&out)?;
            serde_json::to_writer_pretty(file, &openapi)?;
        }
        None => {
            println!("{}", openapi);
        }
    }

    Ok(())
}
