use std::{convert::Infallible, sync::Arc};

use db::DB;
use log::error;
use rest_client::RestClient;
use warp::hyper::{Body, Response};

use crate::{
    models::{ApiConfiguration, Path},
    services::proxy::ProxyService,
};

pub async fn proxy_stream(
    path: Path,
    config: ApiConfiguration,
    db: Arc<DB>,
    client: Arc<RestClient>,
) -> Result<Response<Body>, Infallible> {
    let mut proxy_service = ProxyService::new();
    proxy_service.initialize(db, client);

    let res = match proxy_service.proxy_stream(path.clone(), config).await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to proxy stream with id {}, error: {}", path.id, err);
            Response::builder()
                .status(500)
                .body(Body::from(format!("Error on stream proxy {}", path.id)))
                .unwrap_or_default()
        }
    };
    Ok(res)
}

pub async fn proxy_attr(
    id: u64,
    db: Arc<DB>,
    client: Arc<RestClient>,
) -> Result<Response<Body>, Infallible> {
    let mut proxy_service = ProxyService::new();
    proxy_service.initialize(db, client);

    let res = match proxy_service.proxy_attribute(id).await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to proxy stream with id {}, error: {}", id, err);
            Response::builder()
                .status(500)
                .body(Body::from(format!("Error on attribute proxy {}", id)))
                .unwrap_or_default()
        }
    };

    Ok(res)
}
