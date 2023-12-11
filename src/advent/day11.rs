use axum::{body::Bytes, response::IntoResponse};

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/11", axum::routing::get(axum::http::StatusCode::OK))
		.route("/11/red_pixels", axum::routing::post(day_eleven))
		.nest_service("/11/assets", tower_http::services::ServeDir::new("assets"))
}

async fn day_eleven(
	headers: axum::http::HeaderMap,
	mut body: axum::extract::Multipart,
) -> impl IntoResponse {
	if !headers.contains_key(axum::http::header::CONTENT_TYPE)
		|| !headers
			.get(axum::http::header::CONTENT_TYPE)
			.is_some_and(|h| {
				h.to_str()
					.expect("Could not decode content type")
					.contains("multipart/form-data")
			}) {
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	}

	let mut img_data: Bytes = Bytes::new();

	while let Some(field) = body.next_field().await.expect("Could not unwrap field") {
		if field.name().unwrap().to_string().contains("image") {
			img_data = field.bytes().await.unwrap();
		}
	}

	let img = image::io::Reader::new(std::io::Cursor::new(img_data))
		.with_guessed_format()
		.expect("Could not infer image format")
		.decode()
		.expect("Could not decode image");

	let red_pix = img
		.as_rgb8()
		.expect("Could not interperet image data")
		.pixels()
		.map(image::Pixel::channels)
		.map(Vec::from)
		.map(|i| i.into_iter().map(u16::from).collect::<Vec<u16>>())
		.map(|i| {
			i.first()
				.unwrap_or(&0)
				.cmp(&(i.get(1).unwrap_or(&0) + i.get(2).unwrap_or(&0)))
				.is_gt()
		})
		.map(usize::from)
		.sum::<usize>();

	(axum::http::StatusCode::OK, red_pix.to_string()).into_response()
}
