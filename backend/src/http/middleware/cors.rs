use std::time::Duration;

use tower_http::cors::{AllowHeaders, Any, CorsLayer};

pub fn middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(AllowHeaders::mirror_request())
        .max_age(Duration::from_secs(600))
}
