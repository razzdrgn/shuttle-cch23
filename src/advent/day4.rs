pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/4", axum::routing::get(axum::http::StatusCode::OK))
}