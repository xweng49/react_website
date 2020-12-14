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
use log::{info, warn};
use tokio_test;



#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}



/// This handler manually load request payload and parses json-rust
async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let request: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    //TODO: replace the unwrap maybe.
    //TODO: using injson to get quotes:
    let quotes = get_quotes(request).await.unwrap();

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
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("the answer was: {}", "asdofjd");
    warn!("did this read out");
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


//Todo : user input
///takes the quotes request, calls a method to process the dates into month, day, year in two tuples
async fn get_quotes(request: JsonValue) -> Result<Vec<Quote>, YahooError>{
    let provider = yahoo::YahooConnector::new();
    //TODO: get the quotes using the json info.
    let name = request["name"].clone().to_string();
    info!("Name: {:?}", name);
    let date_start = request["date_start"].clone();
    let date_end = request["date_end"].clone();
    let (year_start, month_start, day_start) = process_date_ymd(date_start);
    let (year_end, month_end, day_end) = process_date_ymd(date_end);

    //edge-case of same day.
    let start = Utc.ymd(year_start, month_start, day_start).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(year_end, month_end, day_end).and_hms_milli(23, 59, 59, 999);
    // let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    // let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history(&name, start, end));
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
///processes jsonvalue date.
fn process_date_ymd(date: JsonValue) -> (i32, u32, u32) {
    let date_string = date.to_string();
    let vec_myd: Vec<&str> = date_string.split("-").collect();
    let month = vec_myd[0].parse::<u32>().unwrap();
    let year = vec_myd[2].parse::<i32>().unwrap();
    let day = vec_myd[1].parse::<u32>().unwrap();
    println!("why doesn't this work?");
    info!("Month: {:?} Year: {:?} Day: {:?}", month, year, day);
    (year, month, day)
}