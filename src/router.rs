use axum::{
	extract::Path,
	http::StatusCode,
	response::IntoResponse,
	Router,
	routing::get
};
use tower_http::trace;

pub fn build_router() -> Router {
	Router::new()
		.route("/", get(root_endpoint))
		.route("/-1/error", get(day_minusone))
		.route("/1/*serial", get(day_one))
		.layer(trace::TraceLayer::new_for_http()
			.make_span_with(trace::DefaultMakeSpan::new()
				.level(tracing::Level::INFO))
			.on_response(trace::DefaultOnResponse::new()
				.level(tracing::Level::INFO)))
}

pub async fn root_endpoint() -> impl IntoResponse {
	(
		StatusCode::OK,
		"Merry Christmas to all, and to all a good hunt!",
	)
		.into_response()
}

pub async fn day_minusone() -> impl IntoResponse {
	(StatusCode::INTERNAL_SERVER_ERROR, "B-B-B-B-BONUS!!!").into_response()
}

pub async fn day_one(Path(serial): Path<String>) -> impl IntoResponse {
	let v: Vec<&str> = serial.split_terminator('/').collect();
	let mut result: i32 = 0;
	for i in v.iter() {
		result = result ^ i.parse::<i32>().unwrap_or(0);
	}
	(StatusCode::OK, result.pow(3).to_string()).into_response()
}
