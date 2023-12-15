use axum::Json;
use fancy_regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
pub(super) enum GameResult {
	#[serde(rename(serialize = "8 chars"))]
	Length,
	#[serde(rename(serialize = "more types of chars"))]
	CharType,
	#[serde(rename(serialize = "55555"))]
	Digits,
	#[serde(rename(serialize = "math is hard"))]
	Sum,
	#[serde(rename(serialize = "not joyful enough"))]
	Joy,
	#[serde(rename(serialize = "illegal: no sandwich"))]
	Sandwich,
	#[serde(rename(serialize = "outranged"))]
	Symbols,
	#[serde(rename(serialize = "😳"))]
	Emoji,
	#[serde(rename(serialize = "not a coffee brewer"))]
	Sha,
	#[serde(rename(serialize = "that's a nice password"))]
	Pass,
}

#[derive(Serialize)]
pub(super) struct PasswordGame {
	result: String,
	reason: GameResult,
}

impl PasswordGame {
	pub fn new(pass: &str) -> PasswordGame {
		let reason = Self::test(pass);
		let result = match reason {
			GameResult::Pass => "nice",
			_ => "naughty",
		};

		PasswordGame {
			result: result.to_string(),
			reason,
		}
	}

	pub fn create_response(self) -> (axum::http::StatusCode, Json<Self>) {
		match self.reason {
			GameResult::Pass => (axum::http::StatusCode::OK, Json(self)),
			GameResult::Joy => (axum::http::StatusCode::NOT_ACCEPTABLE, Json(self)),
			GameResult::Sandwich => (
				axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
				Json(self),
			),
			GameResult::Symbols => (axum::http::StatusCode::RANGE_NOT_SATISFIABLE, Json(self)),
			GameResult::Emoji => (axum::http::StatusCode::UPGRADE_REQUIRED, Json(self)),
			GameResult::Sha => (axum::http::StatusCode::IM_A_TEAPOT, Json(self)),
			_ => (axum::http::StatusCode::BAD_REQUEST, Json(self)),
		}
	}

	fn test(pass: &str) -> GameResult {
		if Self::test_one(pass) {
			GameResult::Length
		} else if Self::test_two(pass) {
			GameResult::CharType
		} else if Self::test_three(pass) {
			GameResult::Digits
		} else if Self::test_four(pass) {
			GameResult::Sum
		} else if Self::test_five(pass) {
			GameResult::Joy
		} else if Self::test_six(pass) {
			GameResult::Sandwich
		} else if Self::test_seven(pass) {
			GameResult::Symbols
		} else if Self::test_eight(pass) {
			GameResult::Emoji
		} else if Self::test_nine(pass) {
			GameResult::Sha
		} else {
			GameResult::Pass
		}
	}

	fn test_one(pass: &str) -> bool {
		pass.len() < 8
	}

	fn test_two(pass: &str) -> bool {
		Regex::new(r"(?=.*[A-Z])(?=.*[a-z])(?=.*\d).*")
			.unwrap()
			.find(pass)
			.unwrap()
			.is_none()
	}

	fn test_three(pass: &str) -> bool {
		Regex::new(r"\d").unwrap().find_iter(pass).count() < 5
	}

	fn test_four(pass: &str) -> bool {
		Regex::new(r"\d+")
			.unwrap()
			.find_iter(pass)
			.flatten()
			.flat_map(|m| m.as_str().parse::<i32>())
			.sum::<i32>()
			!= 2023
	}

	fn test_five(pass: &str) -> bool {
		!Regex::new(r"[^JjOoYy]*[Jj][^JjOoYy]*[Oo][^JjOoYy]*[Yy][^JjOoYy]*")
			.unwrap()
			.find(pass)
			.unwrap()
			.is_some_and(|r| r.as_str() == pass)
	}

	fn test_six(pass: &str) -> bool {
		Regex::new(r"([^\d\s])[^\d\s]\1")
			.unwrap()
			.find(pass)
			.unwrap()
			.is_none()
	}

	fn test_seven(pass: &str) -> bool {
		Regex::new(r"[⦀-⯿]").unwrap().find(pass).unwrap().is_none()
	}

	fn test_eight(pass: &str) -> bool {
		Regex::new(r"\p{Extended_Pictographic}")
			.unwrap()
			.find(pass)
			.unwrap()
			.is_none()
	}

	fn test_nine(pass: &str) -> bool {
		!sha256::digest(pass)
			.char_indices()
			.nth_back(0)
			.is_some_and(|c| c.1 == 'a')
	}
}
