use axum::response::IntoResponse;
use pathfinding::directed::bfs::bfs;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/integers", axum::routing::post(integers))
		.route("/rocket", axum::routing::post(rocket))
}

async fn integers(body: String) -> impl IntoResponse {
	let mut p: Vec<u64> = body
		.split_terminator('\n')
		.map(|n| n.parse::<u64>().expect("parse_error"))
		.collect();
	p.sort_unstable();
	let q: &u64 = p
		.group_by(|a, b| a == b)
		.filter(|s| s.len() == 1)
		.flatten()
		.next()
		.expect("all_dups");
	(
		axum::http::StatusCode::OK,
		(0..*q).map(|_| "🎁").collect::<String>(),
	)
}

async fn rocket(body: String) -> impl IntoResponse {
	let mut data: Vec<&str> = body.split_terminator('\n').collect();
	data.reverse();
	let starcount: u64 = data
		.pop()
		.expect("empty")
		.parse::<u64>()
		.expect("parse_err");
	let mut starcoords: Vec<Vec<i32>> = Vec::new();
	for _ in 0..starcount {
		let c: Vec<i32> = data
			.pop()
			.expect("empty")
			.split_terminator(' ')
			.map(|n| n.parse::<i32>().expect("parse_err"))
			.collect();
		starcoords.push(c);
	}
	let portcount: u64 = data
		.pop()
		.expect("empty")
		.parse::<u64>()
		.expect("parse_err");
	let mut portcoords: Vec<Vec<u64>> = Vec::new();
	for _ in 0..portcount {
		let c: Vec<u64> = data
			.pop()
			.expect("empty")
			.split_terminator(' ')
			.map(|n| n.parse::<u64>().expect("parse_err"))
			.collect();
		portcoords.push(c);
	}

	let path = bfs(
		&0,
		|p| {
			portcoords
				.iter()
				.filter(|c| c.first().expect("bad portal") == p)
				.map(|c| *c.get(1).expect("bad portal"))
				.collect::<Vec<u64>>()
		},
		|p| p == &(starcount - 1),
	).expect("bad path");

	let dist = path.windows(2).fold(0.0, |a: f32, p| {
		a + distance(
			starcoords
				.get(usize::try_from(*p.first().expect("bad portal")).expect("upgrade to 64 bit"))
				.expect("bad star"),
			starcoords
				.get(usize::try_from(*p.get(1).expect("bad portal")).expect("upgrade to 64 bit"))
				.expect("bad star"),
		)
	});

	(
		axum::http::StatusCode::OK,
		format!("{} {:.3}", path.len() - 1, dist),
	)
}

#[allow(clippy::cast_precision_loss)]
fn distance(a: &Vec<i32>, b: &Vec<i32>) -> f32 {
	if a.len() != b.len() {
		return 0.0;
	}

	let mut sum = 0;

	for i in 0..a.len() {
		sum += (b.get(i).expect("bad coord") - a.get(i).expect("bad coord")).pow(2);
	}

	(sum as f32).sqrt()
}
