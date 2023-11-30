mod router;
use router::build_router;

#[shuttle_runtime::main]
async fn init() -> Result<CCHService, shuttle_runtime::Error> {
	let router = build_router();

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

		tokio::select!(_ = async {axum::serve(listener, router).await.unwrap()} => {});

		Ok(())
	}
}
