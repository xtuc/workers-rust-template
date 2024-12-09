use std::sync::Arc;
use worker::*;
use worker_router::{path, Router};

struct ServerState {}

#[event(start)]
fn init() {
    // Setup a panic hook that prints errors to the JavScript console
    console_error_panic_hook::set_once();
}

async fn get_hello(_req: Request, _state: Arc<ServerState>) -> Result<Response> {
    ResponseBuilder::new().ok("hello")
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let state = Arc::new(ServerState {});

    let router = Router::new_with_state(state).get(path("/hello")?, get_hello);

    router.run(req).await
}
