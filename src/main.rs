use actix_web::{App, HttpServer, web};
use tracing::info;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use rust_auth::config::{
    app::AppConfig, jwt::JwtConfig, repositories::Repositories, services::Services,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::init().await;
    let jwt_config = JwtConfig::init(&config.env);

    let repos = Repositories::init(config.db, config.redis);
    let services = Services::init(repos, jwt_config);

    let server = HttpServer::new(|| {
        let (app, openapi) = App::new()
            .into_utoipa_app()
            .service(utoipa_actix_web::scope("/api/v1").app_data(web::Data::new(services)))
            .split_for_parts();

        app.service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind((config.env.host, config.env.port))?;

    info!("Server has started!");

    server.run().await
}
