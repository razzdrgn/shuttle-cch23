pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/13", axum::routing::get(axum::http::StatusCode::OK))
}