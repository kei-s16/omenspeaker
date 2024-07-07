use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Contents {
    flavor: String,
    cardname: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/", |_, ctx| async move {
            let d1 = ctx.env.d1("DB").expect("couldn't get d1 bindings.");
            let statement = d1.prepare("SELECT * FROM contents ORDER BY random() LIMIT 1");
            let result = statement.all().await?;
            Response::from_json(&result.results::<Contents>().unwrap())
        })
        .run(req, env)
        .await
}
