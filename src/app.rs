use crate::hello;
use crate::user;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

pub struct WarframeTeamApp {}

impl WarframeTeamApp {
    pub async fn run(port: u16) -> std::io::Result<()> {
        parse_logger();
        HttpServer::new(|| {
            App::new()
                .wrap(parse_cors())
                .wrap(Logger::default())
                .service(hello::hello)
                .service(hello::echo)
                .route("/hey", web::get().to(hello::manual_hello))
                .configure(user::config)
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    }
}

fn parse_cors() -> Cors {
    let cors = Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        .max_age(3600);
    cors
}

fn parse_logger() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}
