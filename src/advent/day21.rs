use axum::{extract::Path, response::IntoResponse};
use dms_coordinates::DMS;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/coords/:bin", axum::routing::get(coords))
}

fn get_latlon(id: u64) -> (f64, f64) {
	let cell_id = s2::cellid::CellID(id);
	let cell = s2::cell::Cell::from(cell_id);

	(
		cell.center().latitude().deg(),
		cell.center().longitude().deg()
	)
}

async fn coords(Path(bin): Path<String>) -> impl IntoResponse {
	let id = u64::from_str_radix(bin.as_str(), 2).expect("could not parse input");

	let (la, lo) = get_latlon(id);

	let la = DMS::from_decimal_degrees(la, true);
	let lo = DMS::from_decimal_degrees(lo, false);
	
	let lat = format!("{}°{}'{:.3}''{}", la.degrees, la.minutes, la.seconds, la.bearing);
	let lon = format!("{}°{}'{:.3}''{}", lo.degrees, lo.minutes, lo.seconds, lo.bearing);

	(
		axum::http::StatusCode::OK,
		format!("{lat} {lon}")
	)
}