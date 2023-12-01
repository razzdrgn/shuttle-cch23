pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/8", axum::routing::get(axum::http::StatusCode::OK))
}