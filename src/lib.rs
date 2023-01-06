use worker::*;

mod utils;
mod ergonomic;
mod raw;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

pub async fn sys_fetch(_: Request, _: RouteContext<()>) -> Result<Response> {
    todo!()
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/ergonomic/by-url", ergonomic::fetch_by_url)
        .get_async("/ergonomic/by-request", ergonomic::fetch_by_request)
        .get_async("/ergonomic/by-request-with-init", ergonomic::fetch_by_request_with_init)
        .get_async("/raw/by-url", raw::fetch_by_url)
        .get_async("/raw/by-request", raw::fetch_by_request)
        .get_async("/raw/by-request-with-init", raw::fetch_by_request_with_init)
        .run(req, env)
        .await
}
