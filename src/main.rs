use actix_web::{web, App, HttpServer};
use crate::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::connection::get_pool().await;  // Use the get_pool fn

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) //Clone the pool for each thread
            .service(
                web::resource("/coa")
                    .route(web::get().to(get_coa_route)) // Example endpoint
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}