use axum::{
	response::{Html, IntoResponse},
	Json,
};
use serde_json::Value;
use tracing::info;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/unsafe", axum::routing::post(day_14_unsafe))
		.route("/safe", axum::routing::post(day_14_safe))
}

static HEAD: &str = "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    ";
static TAIL: &str = "
  </body>
</html>";

async fn day_14_unsafe(Json(body): Json<Value>) -> impl IntoResponse {
	info!("Request body: {body}");
	let content: String =
		serde_json::from_value(body.get("content").expect("Could not parse object").clone())
			.expect("Could not deserialize input");
	(
		axum::http::StatusCode::OK,
		Html(format!("{HEAD}{content}{TAIL}")),
	)
}

async fn day_14_safe(Json(body): Json<Value>) -> impl IntoResponse {
	info!("Request body: {body}");
	let uns_content: String =
		serde_json::from_value(body.get("content").expect("Could not parse object").clone())
			.expect("Could not deserialize input");

	let content = uns_content
		.as_str()
		.replace('<',"&lt;")
		.as_str()
		.replace('>',"&gt;")
		.as_str()
		.replace('"',"&quot;");

	info!(content);

	(
		axum::http::StatusCode::OK,
		Html(format!("{HEAD}{content}{TAIL}")),
	)
}
