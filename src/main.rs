mod app;
mod hello;
mod user;

#[actix_web::main]
async fn main() {
    match app::WarframeTeamApp::run(6767).await {
        Ok(_) => println!("Server started"),
        Err(e) => println!("Server failed to start: {}", e),
    }
}
