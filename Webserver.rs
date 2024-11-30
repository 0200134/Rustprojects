use actix_web::{web, middleware, App, HttpServer};
use futures::Future;
use jsonwebtoken::{encode, decode, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, verify};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaim {
    user_id: i32,
    exp: i64,
}

async fn authenticate(mut req: HttpRequest, client: web::Data<Client>) -> Result<HttpResponse, Error> {
    let login_data: LoginData = web::Json::extract(&mut req).await?;

    let rows = client.query("SELECT * FROM users WHERE username = $1", &[&login_data.username]).await?;
    let user: Option<User> = rows.iter().map(|row| User {
        id: row.get(0),
        username: row.get(1),
        password_hash: row.get(2),
    }).next();

    if let Some(user) = user {
        if verify(&login_data.password, &user.password_hash).is_ok() {
            let claim = JwtClaim {
                user_id: user.id,
                exp: (std::time::SystemTime::now() + std::time::Duration::from_secs(3600)).duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            };

            let token = encode(&Header::default(), &claim, &EncodingKey::from_secret("your_secret_key".as_bytes())).unwrap();

            Ok(HttpResponse::Ok().json(json!({"token": token})))
        } else {
            Ok(HttpResponse::Unauthorized().finish())
        }
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

async fn protected_route(req: HttpRequest) -> Result<HttpResponse, Error> {
    let token = req.headers().get("Authorization").unwrap().to_str().unwrap().replace("Bearer ", "");
    let token_data = decode::<JwtClaim>(&token, &Validation::default(Algorithm::HS256), &DecodingKey::from_secret("your_secret_key".as_bytes())).unwrap();

    // Access protected resources using token_data.user_id
    Ok(HttpResponse::Ok().json(json!({"message": "Protected resource accessed successfully"})))
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
            .wrap(middleware::Logger::default())
            .app_data(client.clone())
            .service(web::resource("/authenticate").route(web::post().to(authenticate)))
            .service(web::resource("/protected").route(web::get().to(protected_route)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
