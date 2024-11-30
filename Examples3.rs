use actix_web::{web, App, HttpServer};
use tokio_postgres::{Client, Error};
use dotenv::dotenv;
use std::env;

async fn get_users(client: web::Data<Client>) -> Result<impl actix_web::Responder, Error> {
    let rows = client.query("SELECT * FROM users", &[]).await?;
    let users: Vec<User> = rows.iter().map(|row| User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
    }).collect();
    Ok(web::Json(users))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let client = Client::connect(&database_url, tokio_postgres::NoTls)
        .await
        .expect("Failed to connect to Postgres.");

    let client = web::Data::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::resource("/users").to(get_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
