use std::net::TcpListener;
use dotenv::dotenv;
use routes::run::run;

mod middleware;
mod db;
mod handlers;
mod models;
mod routes;

fn main() {
    dotenv().ok();
    let listener = TcpListener::bind("0.0.0.0:8000").expect("Failed to bind to address");
    println!("Server is running on port 8000");
    run(listener).expect("Failed to run server");
}