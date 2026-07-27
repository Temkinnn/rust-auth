use actix_web::{App, HttpServer};
use tracing::info;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use rust_auth::env::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let env = Env::init();

    let server = HttpServer::new(|| {
        let (app, openapi) = App::new()
            .into_utoipa_app()
            .service(utoipa_actix_web::scope("/api/v1"))
            .split_for_parts();

        app.service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind((env.host, env.port))?;

    info!("Server has started!");

    server.run().await
}
