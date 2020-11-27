use backend::*;
use yahoo_finance::history;
use serde_json::{Value};
use actix_web::{get, web, App, HttpServer, Responder};

use actix_web::{
    error, middleware, Error, HttpRequest, HttpResponse,
};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use json::{JsonValue};


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

/// This handler manually load request payload and parse json-rust
async fn index_mjsonrust(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize json-rust
    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string() },
    };

    let json_response = json::from("{ response: \"Success\" }");
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response.dump()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   pull_from_postman().unwrap();
   // grab_apple_info();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            // .service(web::resource("/extractor").route(web::post().to(index)))
            // .service(
            //     web::resource("/extractor2")
            //         .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
            //         .route(web::post().to(extract_item)),
            // )
            // .service(web::resource("/manual").route(web::post().to(index_manual)))
            .service(web::resource("/mjsonrust").route(web::post().to(index_mjsonrust)))
            // .service(web::resource("/").route(web::post().to(index)))
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

fn untyped_example() -> Result<(),Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}


//TODO: Either find out how to work around their code or fix it myself or find a new api for finance in rust?
async fn grab_apple_info() {
    let number = test_function();
    println!("Hello world: {}", number);

    // retrieve 6 months worth of data for Apple
   let result_data = history::retrieve("AAPL").await;
   let data = result_data.unwrap();
   // print the date and closing price for each day we have data
   for bar in &data {
      // println!("On {} Apple closed at ${:.2}", bar.datetime(), bar.close)
   }

}