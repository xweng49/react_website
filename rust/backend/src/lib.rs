use serde_json::{Value};
use actix_web::{get, web, App, HttpServer, Responder};

use actix_web::{
    error, middleware, Error, HttpRequest, HttpResponse,
};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use json::{JsonValue};
use yahoo_finance_api as yahoo;
use yahoo::{Quote, YResponse, YahooError};
use std::time::{Duration, UNIX_EPOCH};
use chrono::{Utc,TimeZone};
use log::{info, warn};
use tokio_test;

/// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
pub fn test_function() -> i32 {
    10
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

///takes the quotes request, calls a method to process the dates into month, day, year in two tuples
async fn get_quotes(request: JsonValue) -> Result<Vec<Quote>, YahooError>{
    let provider = yahoo::YahooConnector::new();
    let name = request["name"].clone().to_string();
    info!("Name: {:?}", name);
    let date_start = request["date_start"].clone();
    let date_end = request["date_end"].clone();
    let (year_start, month_start, day_start) = process_date_ymd(date_start);
    let (year_end, month_end, day_end) = process_date_ymd(date_end);

    //TODO: edge-case of same day fails currently 
    let start = Utc.ymd(year_start, month_start, day_start).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(year_end, month_end, day_end).and_hms_milli(23, 59, 59, 999);
    // let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    // let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);
    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history(&name, start, end));
    let quotes = match resp {
        Ok(i) => i.quotes(),
        Err(e) => {
            return Err(e)
        }
    };
    
    let quote_vec = match quotes {
        Ok(string) => string,
        Err(e) => return Err(e),
    };
    Ok(quote_vec)
}


/// This handler manually load request payload and parses json-rust
pub async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let request: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    //TODO: replace the unwrap maybe.
    let mut flag = false;
    let mut error: Option<String> = None;
    
    let mut quotes: Result<Vec<Quote>, YahooError>;
    quotes = match get_quotes(request.clone()).await {
        Ok(i) => {
            Ok(i)
        }
        Err(e) => { 
            info!("Our Error: {}", e);
            error = Some(e.to_string());
            let my_vec = vec![];
            info!("Running again");
            flag = true;
            Ok(my_vec)
        }
    };
    
    
    // let json_response = json::from(r#"{ response: "Success" }"#);
    //TODO: process quotes struct so that we can post the response.
    if flag {
        let mut failure_response = json::JsonValue::new_object();
        failure_response["response"] = "Failure".into();
        match error {
            None => (),
            Some(a) => failure_response["errorMessage"] = a.into(),
        };
        //send json error message
        return Ok(HttpResponse::Ok()
        .content_type("application/json")
    .body(failure_response.dump()));
    }
    //send average between open and close. if there is an error, send back a small error in json form.
    let processed_quotes = process_quotes(quotes.unwrap());
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(processed_quotes.dump()))
}


#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
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


#[cfg(test)]
mod tests {
    use log::{info};
    use super::start_server;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    ///Should test for failed inputs and give a useful error message.
    fn test_error_input() {

    }

    ///tests server start
    #[test]
    fn test_start_up() {
        let result = match start_server() {
            Ok(i) => "SUCCESS".to_string(),
            Err(e) => format!("FAILURE: server didn't start. Error msg: {}", e) ,   
        };
        info!("result: {}", result );
    }
}
