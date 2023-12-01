pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/21", axum::routing::get(axum::http::StatusCode::OK))
}