mod cli;
mod db;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    sqlx::migrate!();
}
