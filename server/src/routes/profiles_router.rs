use axum::{routing::get, Router};

use crate::controllers::profiles::get_profile;

pub fn build_profiles_router() -> Router {
    Router::new()
        .route("/profiles", get(get_profile))
}
