use servlin::{
    Error,
    HttpServerBuilder,
    Request,
    Response,
};
use servlin::log::log_request_and_response;
use servlin::reexport::{safina_executor, safina_timer};
use std::sync::Arc;
use temp_dir::TempDir;
use applin::{applin_response, back_button, nav_page, pop, text, user_error};

struct State {}

// fn index(_state: Arc<State>, req: Request) -> Result<Response, Error> {
//     #[derive(Deserialize)]
//     struct Input {
//         name: String,
//     }
//     let input: Input = req.json()?;
//     Ok(Response::json(200, json!({"message": format!("Hello, {}!", input.name)}))
//         .unwrap())
// }

fn index(_state: Arc<State>, _req: Request) -> Result<Response, Error> {
    Ok(applin_response(
        nav_page(
            "Applin Rust Demo",
            text("Hello!"),
        ).with_start(back_button([pop()]))
    ).unwrap())
}

fn handle_req(state: Arc<State>, req: Request) -> Result<Response, Error> {
    match (req.method(), req.url().path()) {
        ("GET", "/healthz") => Ok(Response::text(200, "success")),
        ("GET", "/ok") => Ok(Response::new(200)),
        ("GET", "/error") => Ok(user_error("example error")),
        ("GET", "/") => index(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

fn main() {
    let state = Arc::new(State {});
    let request_handler = move |req: Request| {
        log_request_and_response(req, |req| handle_req(state, req)).unwrap()
    };
    let cache_dir = TempDir::new().unwrap();
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::new(1, 4).unwrap();
    executor.block_on(
        HttpServerBuilder::new()
            .listen_addr(servlin::socket_addr_127_0_0_1(8000))
            .max_conns(10)
            .small_body_len(64 * 1024)
            .receive_large_bodies(cache_dir.path())
            .spawn_and_join(request_handler)
    ).unwrap();
}
