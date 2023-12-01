pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/18", axum::routing::get(axum::http::StatusCode::OK))
}