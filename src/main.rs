use sea_orm::prelude::*;
mod entity;
use entity::bit_test;


#[tokio::main]
async fn main() {
    let conn = sea_orm::Database::connect(std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let query = bit_test::Entity::find();
    let results = query.clone().all(&conn).await.unwrap();
    let results_json = query.into_json().all(&conn).await.unwrap();

    dbg!(&results);
    dbg!(&results_json);
    assert!(!results.is_empty());
    for result in results_json {
        assert!(result.get("field").is_some())
    }
}
