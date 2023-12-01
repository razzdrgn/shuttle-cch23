pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/12", axum::routing::get(axum::http::StatusCode::OK))
}