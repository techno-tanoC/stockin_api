use std::{env, net::SocketAddr};

use anyhow::Result;
use stockin_api::App;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    let stackdriver = tracing_stackdriver::layer();
    tracing_subscriber::registry()
        .with(stackdriver)
        .with(EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL")?;
    let port = env::var("PORT").unwrap_or("3000".to_string()).parse()?;
    let token = env::var("BEARER_TOKEN")?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = App::new_app(&database_url, &token)
        .await?
        .layer(axum::middleware::from_fn(
            stockin_api::handler::access_log_middleware,
        ));

    axum::serve(listener, app).await?;
    Ok(())
}
