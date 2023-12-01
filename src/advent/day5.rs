pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/5", axum::routing::get(axum::http::StatusCode::OK))
}