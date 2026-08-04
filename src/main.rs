use actix_web::{App, HttpServer, web};
use tracing::info;

use rust_auth::{
    config::{app::AppConfig, jwt::JwtConfig, repositories::Repositories, services::Services},
    handlers::{auth::auth_router, user::users_router},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init().await;
    let jwt_config = JwtConfig::init(&config.env);

    let repos = Repositories::init(config.db, config.redis);
    let services = Services::init(repos, jwt_config);

    let services_data = web::Data::new(services);

    let server = HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .app_data(services_data.clone())
                .configure(auth_router)
                .configure(users_router),
        )
    })
    .bind((config.env.host, config.env.port))?;

    info!("Server has started!");

    server.run().await
}
