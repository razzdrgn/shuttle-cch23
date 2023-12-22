use axum::{extract::Path, response::IntoResponse};
use capitalize::Capitalize;
use celes::LookupTable;
use dms_coordinates::DMS;

pub fn router() -> axum::Router {
	axum::Router::new()
		.route("/coords/:bin", axum::routing::get(coords))
		.route("/country/:bin", axum::routing::get(country))
}

fn get_latlon(id: u64) -> (f64, f64) {
	let cell_id = s2::cellid::CellID(id);
	let cell = s2::cell::Cell::from(cell_id);

	(
		cell.center().latitude().deg(),
		cell.center().longitude().deg(),
	)
}

async fn coords(Path(bin): Path<String>) -> impl IntoResponse {
	let id = u64::from_str_radix(bin.as_str(), 2).expect("could not parse input");

	let (la, lo) = get_latlon(id);

	let la = DMS::from_decimal_degrees(la, true);
	let lo = DMS::from_decimal_degrees(lo, false);

	let lat = format!(
		"{}°{}'{:.3}''{}",
		la.degrees, la.minutes, la.seconds, la.bearing
	);
	let lon = format!(
		"{}°{}'{:.3}''{}",
		lo.degrees, lo.minutes, lo.seconds, lo.bearing
	);

	(axum::http::StatusCode::OK, format!("{lat} {lon}"))
}

async fn country(Path(bin): Path<String>) -> impl IntoResponse {
	let id = u64::from_str_radix(bin.as_str(), 2).expect("could not parse input");

	let lalo = get_latlon(id);

	let country = celes::Country::from_alpha2(
		country_boundaries::CountryBoundaries::from_reader(
			country_boundaries::BOUNDARIES_ODBL_360X180,
		)
		.expect("could not load country data")
		.ids(country_boundaries::LatLon::new(lalo.0, lalo.1).expect("bad latlon"))
		.first()
		.expect("no country code")
		.get(..2)
		.expect("could not slice"),
	)
	.expect("could not parse country code");

	(
		axum::http::StatusCode::OK,
		country
			.aliases
			.iter()
			.next()
			.unwrap_or(&country.long_name)
			.capitalize(),
	)
}
