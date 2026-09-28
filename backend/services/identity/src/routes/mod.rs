mod v1;

use axum::{Router};

use crate::routes::v1::router_v1;

pub fn router() -> Router {
  Router::new()
    .nest("/api/v1", router_v1())
}