use backend::*;
use serde_json::{Value};
use actix_web::{get, web, App, HttpServer, Responder};

use actix_web::{
    error, middleware, Error, HttpRequest, HttpResponse,
};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use json::{JsonValue};
use yahoo_finance_api as yahoo;
use yahoo::{YahooError, Quote};
use std::time::{Duration, UNIX_EPOCH};
use chrono::{Utc,TimeZone};
use tokio_test;



// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }
#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

/// This handler uses json extractor with limit
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0) // <- send json response
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

/// This handler manually load request payload and parse json object
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(actix_web::error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

/// This handler manually load request payload and parses json-rust
async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    //TODO: replace the unwrap maybe.
    //TODO: using injson to get quotes:
    let quotes = get_quotes().await.unwrap();

    // let json_response = json::from(r#"{ response: "Success" }"#);
    //TODO: process quotes struct so that we can post the response.
    let processed_quotes = process_quotes(quotes);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(processed_quotes.dump()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
//    pull_from_postman().unwrap();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/mjsonrust").route(web::post().to(index_mjsonrust)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

    
}

fn pull_from_postman() -> Result<(), Error> {
   //listens for postman request, reads the response.

   let response = r#"
   {
      "name": "company",
      "date_start": "1-1-2020",
      "date_end": "12-30-2020"
  }"#;

  let value: Value = serde_json::from_str(response)?;

  println!("Name: {}, date_start: {}, date_end: {}", value["name"], value["date_start"], value["date_end"]);
  Ok(())
}

//Todo : user input
async fn get_quotes() -> Result<Vec<Quote>, YahooError>{
    let provider = yahoo::YahooConnector::new();
    //TODO: get the quotes using the json info.
    let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2020,1,31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end));
    let quotes = resp.unwrap().quotes();
    let quote_vec = match quotes {
        Ok(string) => string,
        Err(e) => return Err(e),
    };
    println!("Apple's quotes of the last month: {:?}", quote_vec);
    Ok(quote_vec)
}

fn process_quotes(quotes: Vec<Quote>) -> JsonValue {
    let mut num = 0;
    let mut jsonQuotes = json::JsonValue::new_array();
    for  quote in quotes {
        let mut data = json::JsonValue::new_object();
        data["timestamp"] = quote.timestamp.into();
        data["open"] = quote.open.into();
        data["high"] = quote.high.into();
        data["low"] = quote.low.into();
        data["volume"] = quote.volume.into();
        data["close"] = quote.close.into();
        data["adjclose"] = quote.adjclose.into();
        //pushes quote data with quote key into jsonquotes
        jsonQuotes[format!("Quote {}: ", num)] = data.into();
        num +=1;
    }
    jsonQuotes
}