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

mod db_common;

pub async fn router(pool: sqlx::PgPool) -> Result<axum::Router, shuttle_runtime::CustomError> {
	let state = db_common::DBState::init(pool).await?;

	Ok(axum::Router::new()
		.nest("/", day0::router())
		.nest("/1", day1::router())
		// Days 2 and 3 are omitted due to being weekends
		.nest("/4", day4::router())
		.nest("/", day5::router())
		.nest("/6", day6::router())
		.nest("/7", day7::router())
		.nest("/8", day8::router())
		// Days 9 and 10 are omitted due to being weekends
		.nest("/11", day11::router())
		.nest("/12", day12::router())
		.nest("/13", day13::router(&state))
		.nest("/14", day14::router())
		.nest("/15", day15::router())
		// Days 16 and 17 are omitted due to being weekends
		.nest("/18", day18::router(&state))
		.nest("/19", day19::router())
		.nest("/20", day20::router())
		.nest("/21", day21::router())
		.nest("/22", day22::router()))
}
