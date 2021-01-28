use backend::*;
use serde_json::{Value};
use actix_web::{get, web, App, HttpServer, Responder};

use actix_web::{
    middleware
};
use serde::{Serialize, Deserialize};

use log::{info, warn};




#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}


// #[actix_web::main]
fn main() -> std::io::Result<()> {
    backend::start_server()   
}

