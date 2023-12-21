use axum::{body::Bytes, http::HeaderMap, response::IntoResponse};
use bytes::Buf;
use git2::Repository;
use tar::Archive;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/archive_files", axum::routing::post(filecount))
		.route("/archive_files_size", axum::routing::post(filesize))
		.route("/cookie", axum::routing::post(cookie))
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

async fn cookie(headers: HeaderMap, body: Bytes) -> impl IntoResponse {
	if headers
		.get("Content-Type")
		.unwrap()
		.to_str()
		.unwrap_or_default()
		!= "application/x-tar"
	{
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	}

	let dir = tempfile::tempdir().unwrap();

	assert!(
		Archive::new(body.reader()).unpack(dir.path()).is_ok(),
		"unable to extract archive"
	);

	let r = Repository::open(dir.path()).expect("could not open git repo");

	let mut commit = r
		.find_branch("christmas", git2::BranchType::Local)
		.expect("branch does not exist")
		.get()
		.peel_to_commit()
		.expect("could not extract commit");

	loop {
		let tree = commit.tree().unwrap();
		let objects: Vec<git2::Object> = tree
			.iter()
			.filter(|e| e.name().unwrap_or_default() == "santa.txt")
			.flat_map(|e| e.to_object(&r))
			.collect();
		let strings: Vec<&str> = objects
			.iter()
			.filter_map(|o| o.as_blob())
			.map(git2::Blob::content)
			.flat_map(|c| std::str::from_utf8(c))
			.filter(|s| s.contains("COOKIE"))
			.collect();

		if !strings.is_empty() || commit.parents().next().is_none() {
			break;
		}
		commit = commit.parents().next().unwrap();
	}

	(
		axum::http::StatusCode::OK,
		format!(
			"{} {}",
			commit.author().name().unwrap_or_default(),
			commit.id()
		),
	)
		.into_response()
}
