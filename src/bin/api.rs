use std::{env, net::SocketAddr};

use anyhow::Result;
use stockin_api::{App, State};

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let port = env::var("PORT").unwrap_or("3000".to_string()).parse()?;
    let token = env::var("BEARER_TOKEN")?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let state = State::from_url(&database_url).await?;
    let app = App::new(state, &token).await?;
    app.start(listener).await?;

    Ok(())
}
