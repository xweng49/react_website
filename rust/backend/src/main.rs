use backend::*;
use yahoo_finance::history;
use serde_json::{Result, Value};
use actix_web::{get, web, App, HttpServer, Responder};





#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   pull_from_postman().unwrap();
   // grab_apple_info();
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn pull_from_postman() -> Result<()> {
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
fn untyped_example() -> Result<()> {
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