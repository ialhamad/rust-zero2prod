use crate::routes::{health_check::health_check, subscriptions::subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;
pub fn run(listerner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listerner)?
    .run();
    Ok(server)
}
