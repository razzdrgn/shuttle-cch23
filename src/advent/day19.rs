use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use axum::{
	extract::{
		ws::{Message, WebSocket},
		Path, State, WebSocketUpgrade,
	},
	response::IntoResponse,
};

use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast;
use futures_util::{stream::StreamExt, SinkExt};

pub fn router() -> axum::Router {
	let ping = axum::Router::new()
		.route("/ws/ping", axum::routing::get(day_19_ping_setup))
		.with_state(GameState { started: false });

	let tweet = axum::Router::new()
		.route(
			"/ws/room/:id/user/:name",
			axum::routing::get(day_19_tweet_setup),
		)
		.route("/reset", axum::routing::post(day_19_reset))
		.route("/views", axum::routing::get(day_19_views))
		.with_state(Arc::new(RwLock::new(TweetState::new())));

	axum::Router::new().nest("/", tweet).nest("/", ping)
}

#[derive(Clone)]
struct GameState {
	started: bool,
}

#[derive(Clone)]
struct TweetState {
	count: u32,
	rooms: HashMap<i32, broadcast::Sender<String>>,
}

impl TweetState {
	fn new() -> TweetState {
		TweetState {
			count: 0,
			rooms: HashMap::new(),
		}
	}

	fn reset(&mut self) {
		self.count = 0;
		self.rooms = HashMap::new();
	}

	fn inc(&mut self) {
		self.count += 1;
	}
}

#[derive(Clone)]
struct FeedState {
	tweets: Arc<RwLock<TweetState>>,
	conn: TweetParams,
}

#[derive(Deserialize, Clone)]
struct TweetParams {
	room: i32,
	name: String,
}

async fn day_19_ping_setup(
	ws: WebSocketUpgrade,
	State(state): State<GameState>,
) -> impl IntoResponse {
	ws.on_upgrade(|socket| day_19_ping_socket(socket, state))
}

async fn day_19_ping_socket(mut socket: WebSocket, mut state: GameState) {
	while let Some(msg) = socket.recv().await {
		let msg = if let Ok(msg) = msg {
			if let Ok(msg) = msg.into_text() {
				msg
			} else {
				String::new()
			}
		} else {
			state.started = false;
			String::new()
		};

		if msg.as_str() == "serve" {
			state.started = true;
		}

		if msg.as_str() == "ping" && state.started {
			let _ = socket.send(Message::Text("pong".to_string())).await;
		}
	}
}

async fn day_19_reset(State(state): State<Arc<RwLock<TweetState>>>) -> impl IntoResponse {
	state.write().expect("could not write lock state").reset();
	axum::http::StatusCode::OK
}

async fn day_19_views(State(state): State<Arc<RwLock<TweetState>>>) -> impl IntoResponse {
	(
		axum::http::StatusCode::OK,
		state
			.read()
			.expect("could not read lock state")
			.count
			.to_string(),
	)
}

async fn day_19_tweet_setup(
	ws: WebSocketUpgrade,
	State(state): State<Arc<RwLock<TweetState>>>,
	Path((id, name)): Path<(i32, String)>,
) -> impl IntoResponse {
	ws.on_upgrade(move |socket| {
		day_19_tweet_socket(
			socket,
			FeedState {
				tweets: state,
				conn: TweetParams { room: id, name },
			},
		)
	})
}

async fn day_19_tweet_socket(socket: WebSocket, state: FeedState) {
	#[derive(Deserialize)]
	struct Msg {
		message: String,
	}

	let tweets = state.tweets;

	let (mut send, mut recv) = socket.split();

	let out = tweets
		.write()
		.expect("could not establish write lock on data")
		.rooms
		.entry(state.conn.room)
		.or_insert(broadcast::channel(100).0)
		.clone();

	let mut sub = out.subscribe();

	let mut send_tweet = tokio::spawn(async move {
		while let Ok(msg) = sub.recv().await {
			tweets
				.write()
				.expect("could not establish write lock on data")
				.inc();
			if send.send(Message::Text(msg)).await.is_err() {
				break;
			}
		}
	});

	let mut recv_tweet = {
		tokio::spawn(async move {
			while let Some(Ok(Message::Text(msg))) = recv.next().await {
				let msg: Msg = serde_json::from_str(msg.as_str())
					.expect("could not deserialize input message");
				if msg.message.chars().count() > 128 {
					continue;
				}
				let msg = json!({"user":state.conn.name.clone(),"message":msg.message}).to_string();
				let _ = out.send(msg);
			}
		})
	};

	tokio::select! {
		_ = (&mut send_tweet) => recv_tweet.abort(),
		_ = (&mut recv_tweet) => send_tweet.abort(),
	};
}
