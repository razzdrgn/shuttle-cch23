use axum::{body::Bytes, http::HeaderMap, response::IntoResponse};
use bytes::Buf;
use tar::Archive;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/archive_files", axum::routing::post(filecount))
		.route("/archive_files_size", axum::routing::post(filesize))
}

async fn filecount(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
	if headers
		.get("Content-Type")
		.unwrap()
		.to_str()
		.unwrap_or_default()
		!= "application/x-tar"
	{
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	};

	(
		axum::http::StatusCode::OK,
		Archive::new(body.reader())
			.entries()
			.expect("could not parse archive")
			.count()
			.to_string(),
	)
		.into_response()
}

async fn filesize(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
	if headers
		.get("Content-Type")
		.unwrap()
		.to_str()
		.unwrap_or_default()
		!= "application/x-tar"
	{
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	}

	(
		axum::http::StatusCode::OK,
		Archive::new(body.reader())
			.entries()
			.unwrap()
			.flatten()
			.map(|f| f.size())
			.sum::<u64>()
			.to_string(),
	)
		.into_response()
}
