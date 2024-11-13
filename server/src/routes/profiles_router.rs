use crate::handlers::profiles::{
    add_profile, delete_profile, get_profile, get_profiles, update_profile,
};
use axum::{
    routing::get,
    Router,
};
use developer_blog_business::path::Path;

use super::router::Database;

pub fn build_profiles_router() -> Router {
    let database = Database::default();

    Router::new()
        .route(
            Path::Profiles.to_string().as_str(),
            get(get_profiles).post(add_profile),
        )
        .route(
            Path::ProfileId.to_string().as_str(),
            get(get_profile)
                .delete(delete_profile)
                .patch(update_profile),
        )
        // TODO add persistence
        // .route("/todos/persist", post(persist))
        .with_state(database)
}
