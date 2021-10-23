use chrono::prelude::Utc;
use serde::{Deserialize, Serialize};
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[derive(Deserialize, Serialize, Debug)]
struct Jmeniny {
    name: String,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
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
        .get_async("/today", |_req, ctx| async move {
            let jmeniny = ctx.kv("KV_JMENINY")?;
            let key = Utc::now().format("%d%m").to_string();
            return match jmeniny.get(&key).await? {
                Some(name) => Response::from_json(&name.as_json::<Jmeniny>()?),
                None => Response::error("Not found", 404),
            };
        })
        .run(req, env)
        .await
}
