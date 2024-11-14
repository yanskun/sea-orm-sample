use axum::{response::Json, routing::get, Router};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, EntityTrait, QuerySelect};
use serde_json::json;
use std::env;
use std::net::SocketAddr;

mod entity; // Adjusted to match your folder structure

use entity::users::Entity as Users;

async fn get_users(db: DatabaseConnection) -> Json<serde_json::Value> {
    let users = Users::find()
        .select_only()
        .column(entity::users::Column::Name)
        .into_tuple::<String>()
        .all(&db)
        .await
        .unwrap();
    Json(json!(users))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await.unwrap();

    // Clone the database connection for each handler
    let app = Router::new().route("/users", get(move || get_users(db.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
