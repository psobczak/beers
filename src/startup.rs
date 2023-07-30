use std::net::TcpListener;

use axum::{
    routing::{get, post},
    Router, ServiceExt,
};
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

use crate::{beer::post_beer, settings::AppConfig};

pub struct Application;

impl Application {
    pub async fn build(config: &AppConfig) -> Result<(), std::io::Error> {
        let address = config.application.get_address();
        let listener = TcpListener::bind(address)?;
        let pool = config
            .database
            .connect()
            .expect("Failed to create db connection");

        let redis_manager = config.redis.get_redis_connection().await.unwrap();

        run(listener, redis_manager, pool).await;

        Ok(())
    }
}

async fn run(listener: TcpListener, redis_manager: ConnectionManager, pool: PgPool) {
    let router = Router::new()
        .route("/beer", post(post_beer))
        .with_state(pool)
        .with_state(redis_manager);

    let service = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid::default())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        )
        .propagate_x_request_id()
        .service(router);

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(service.into_make_service())
        .await
        .unwrap();
}
