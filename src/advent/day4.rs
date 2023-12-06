use axum::{http::StatusCode, response::IntoResponse, extract::Json};
use serde::{Serialize, Deserialize};
use tracing::info;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/4", axum::routing::get(axum::http::StatusCode::OK))
		.route("/4/strength", axum::routing::post(day_four_strength))
		.route("/4/contest", axum::routing::post(day_four_contest))
}

#[derive(Deserialize)]
struct Reindeer {
	name: String,
	strength: isize,
	speed: Option<f32>,
	height: Option<isize>,
	antler_width: Option<isize>,
	snow_magic_power: Option<isize>,
	favorite_food: Option<String>,
	candies_eaten_yesterday: Option<isize>,
}

#[derive(Serialize)]
struct Results {
	fastest: String,
	tallest: String,
	magician: String,
	consumer: String,
}

impl Results {
	fn new(deer: &[Reindeer]) -> Results {
		let fastest_deer = deer.iter().reduce(|x,y| if x.speed > y.speed { x } else { y }).unwrap();
		let tallest_deer = deer.iter().reduce(|x,y| if x.height > y.height { x } else { y }).unwrap();
		let magician_deer = deer.iter().reduce(|x,y| if x.snow_magic_power > y.snow_magic_power { x } else { y }).unwrap();
		let consumer_deer = deer.iter().reduce(|x,y| if x.candies_eaten_yesterday > y.candies_eaten_yesterday { x } else { y }).unwrap();
		let fastest = format!("Speeding past the finish line with a strength of {} is {}", fastest_deer.strength, fastest_deer.name);
		let tallest = format!("{} is standing tall with his {} cm wide antlers", tallest_deer.name, tallest_deer.antler_width.unwrap());
		let magician = format!("{} could blast you away with a snow magic power of {}", magician_deer.name, magician_deer.snow_magic_power.unwrap());
		let consumer = format!("{} ate lots of candies, but also some {}", consumer_deer.name, consumer_deer.favorite_food.clone().unwrap());
		Results { fastest, tallest, magician, consumer }
	}
}

async fn day_four_strength(Json(reindeer): Json<String>) -> impl IntoResponse {
	info!("Request Body: {}", &reindeer);
	let deer: Vec<Reindeer> = serde_json::from_str(&reindeer).expect("Broke when running from_str");
	(StatusCode::OK, deer.iter().map(|x| x.strength).reduce(|x,y| x+y).unwrap_or_default().to_string())
}

async fn day_four_contest(Json(reindeer): Json<String>) -> impl IntoResponse {
	info!("Request Body: {}", &reindeer);
	let deer: Vec<Reindeer> = serde_json::from_str(&reindeer).expect("Broke when running from_str");
	(StatusCode::OK, serde_json::to_string(&Results::new(&deer)).expect("Broke when serializing results"))
}