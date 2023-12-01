mod day0;
mod day1;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

pub fn router() -> axum::Router {
	axum::Router::new()
		.nest("/", day0::router())
		.nest("/", day1::router())
		// Days 2 and 3 are omitted due to being weekends
		.nest("/", day4::router())
		.nest("/", day5::router())
		.nest("/", day6::router())
		.nest("/",  day7::router())
		.nest("/",  day8::router())
		// Days 9 and 10 are omitted due to being weekends
		.nest("/",  day11::router())
		.nest("/",  day12::router())
		.nest("/",  day13::router())
		.nest("/",  day14::router())
		.nest("/",  day15::router())
		// Days 16 and 17 are omitted due to being weekends
		.nest("/",  day18::router())
		.nest("/",  day19::router())
		.nest("/",  day20::router())
		.nest("/",  day21::router())
		.nest("/",  day22::router())
}