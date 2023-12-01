pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/20", axum::routing::get(axum::http::StatusCode::OK))
}