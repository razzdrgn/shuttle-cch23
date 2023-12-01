pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/6", axum::routing::get(axum::http::StatusCode::OK))
}