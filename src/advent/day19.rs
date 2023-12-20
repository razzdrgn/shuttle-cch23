use axum::{
	extract::{
		ws::{Message, WebSocket},
		WebSocketUpgrade, State,
	},
	response::IntoResponse,
};

pub fn router() -> axum::Router {
	let ping = axum::Router::new()
		.route("/ping", axum::routing::get(day_19_ping_setup))
		.with_state(GameState { started: false });

	axum::Router::new().nest("/ws", ping)
}

#[derive(Clone)]
struct GameState {
	started: bool,
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
