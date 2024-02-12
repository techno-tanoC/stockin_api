use std::{env, net::SocketAddr};

use anyhow::Result;
use stockin_api::App;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let port = env::var("PORT").unwrap_or("3000".to_string()).parse()?;
    let token = env::var("BEARER_TOKEN")?;
    let app = App::new_app(&database_url, &token).await?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
