use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use sha2::{Sha256};
use hmac::{Hmac, Mac};


async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let secret = std::env::var("TELEGRAM_BOT_TOKEN").unwrap();

    let query_string = event.query_string_parameters();
    let query_id = query_string.first("query_id").unwrap();
    let user = query_string.first("user").unwrap();
    let auth_date = query_string.first("auth_date").unwrap();
    let hash = query_string.first("hash").unwrap();

    // 'auth_date=<auth_date>\nquery_id=<query_id>\nuser=<user>'
    let data = format!("auth_date={}\nquery_id={}\nuser={}", auth_date, query_id, user);

    // data = ...
    //     secret_key = HMAC_SHA256(<bot_token>, "WebAppData")
    // if (hex(HMAC_SHA256(data_check_string, secret_key)) == hash) {
    //     // data is from Telegram
    // }
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice("WebAppData".as_bytes()).unwrap();
    mac.update(secret.as_bytes());
    let secret_key = mac.finalize().into_bytes();

    let mut mac = HmacSha256::new_from_slice(&secret_key).unwrap();
    mac.update(data.as_bytes());
    let hash_check = mac.finalize().into_bytes();
    let hash_check = hex::encode(hash_check);

    if hash_check == hash {
        Ok(Response::builder()
            .status(200)
            .body(Body::from(user.to_string()))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(403)
            .body(Body::from("Forbidden"))
            .unwrap())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
