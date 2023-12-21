use axum::{body::Bytes, http::HeaderMap, response::IntoResponse};
use bytes::Buf;
use git2::{Object, Repository};
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
		.is_some_and(|v| v.to_str().unwrap_or_default() != "application/x-tar")
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
		.is_some_and(|v| v.to_str().unwrap_or_default() != "application/x-tar")
	{
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	};

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
		.is_some_and(|v| v.to_str().unwrap_or_default() != "application/x-tar")
	{
		return axum::http::StatusCode::BAD_REQUEST.into_response();
	};

	let dir = tempfile::tempdir().unwrap();

	assert!(
		Archive::new(body.reader()).unpack(dir.path()).is_ok(),
		"unable to extract archive"
	);

	let r = Repository::open(dir.path()).expect("could not open git repo");

	let commit = r
		.find_branch("christmas", git2::BranchType::Local)
		.expect("branch does not exist")
		.get()
		.peel_to_commit()
		.expect("could not extract commit");

	let cookie = cookiefinder(0, &commit, &r).1.unwrap();

	(
		axum::http::StatusCode::OK,
		format!(
			"{} {}",
			cookie.author().name().unwrap_or_default(),
			cookie.id()
		),
	)
		.into_response()
}

fn cookiefinder<'a>(
	count: u32,
	commit: &git2::Commit<'a>,
	r: &git2::Repository,
) -> (u32, Option<git2::Commit<'a>>) {
	let tree = commit.tree().unwrap();
	let mut santa: Vec<Object> = Vec::new();
	tree.walk(git2::TreeWalkMode::PreOrder, |_, e| {
		if e.name() == Some("santa.txt") {
			santa.push(e.clone().to_object(r).expect("Couldn't push object"));
		}
		git2::TreeWalkResult::Ok
	})
	.unwrap();
	let strings: Vec<&str> = santa
		.iter()
		.filter_map(|o| o.as_blob())
		.map(git2::Blob::content)
		.flat_map(|c| std::str::from_utf8(c))
		.filter(|s| s.contains("COOKIE"))
		.collect();

	if !strings.is_empty() {
		return (count, Some(commit.clone()));
	}

	commit
		.parents()
		.map(|p| cookiefinder(count + 1, &p, r))
		.min_by_key(|c| c.0)
		.unwrap_or((0, None))
}
