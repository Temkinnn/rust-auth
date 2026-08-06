use actix_web::{App, HttpServer, web};
use tracing::info;
use utoipa::Modify;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use rust_auth::{
    config::{app::AppConfig, jwt::JwtConfig, repositories::Repositories, services::Services},
    docs::SecurityAddon,
    handlers::{auth::auth_router, user::users_router},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init().await;
    let jwt_config = JwtConfig::init(&config.env);

    let repos = Repositories::init(config.db, config.cache);
    let services = Services::init(repos, jwt_config);

    let services_data = web::Data::new(services);

    let server = HttpServer::new(move || {
        let (app, mut openapi) = App::new()
            .into_utoipa_app()
            .service(
                utoipa_actix_web::scope("/api/v1")
                    .app_data(services_data.clone())
                    .configure(auth_router)
                    .configure(users_router),
            )
            .split_for_parts();

        SecurityAddon.modify(&mut openapi);
        // Connect Swagger
        app.service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind((config.env.host, config.env.port))?;

    info!("Server has started!");

    server.run().await
}
