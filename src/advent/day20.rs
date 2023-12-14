pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/", axum::routing::get(axum::http::StatusCode::OK))
}