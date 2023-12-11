pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/11", axum::routing::get(axum::http::StatusCode::OK))
		.nest_service("/11/assets", tower_http::services::ServeDir::new("assets"))
}
