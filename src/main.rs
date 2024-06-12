use std::sync::Arc;

use tokio::sync::RwLock;

mod config;
mod controller;
mod service;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    config::log::init();

    service::signal_svc::handle();

    let config = config::config::get_config();
    tracing::debug!("Read Config: {:#?}", &config);

    let app_state = config::state::AppState {
        config: config.clone(),
        token: Arc::new(RwLock::new(None)),
        token_exp: Arc::new(RwLock::new(None)),
    };

    let router = config::route::init(app_state).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5234").await?;
    tracing::info!("listening on {:?}", listener);
    axum::serve(listener, router).await
        .unwrap_or_else(|e| {
            panic!("start service fail {:#?}", e)
        });

    anyhow::Ok(())
}
