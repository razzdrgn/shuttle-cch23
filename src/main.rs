use tower_http::trace;

mod advent;

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn init() -> Result<CCHService, shuttle_runtime::Error> {
	let router = advent::router()
		.layer(trace::TraceLayer::new_for_http()
			.make_span_with(trace::DefaultMakeSpan::new()
				.level(tracing::Level::INFO))
			.on_response(trace::DefaultOnResponse::new()
				.level(tracing::Level::INFO)));

	Ok(CCHService { router })
}

struct CCHService {
	router: axum::Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CCHService {
	async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
		let router = self.router;
		let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

		tokio::select!(() = async {axum::serve(listener, router).await.unwrap()} => {});

		Ok(())
	}
}
