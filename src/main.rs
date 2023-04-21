use sea_orm::prelude::*;
mod entity;


#[tokio::main]
async fn main() {
    let conn = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap()).await.unwrap();
}
