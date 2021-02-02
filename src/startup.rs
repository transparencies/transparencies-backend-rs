use crate::routes::{health_check, matchinfo};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use log::{debug, error, info, trace, warn};

use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Entrypoint for the library part of the Executable's main function
pub fn run(
    listener: TcpListener,
    /*config: &cli::Args*/
) -> eyre::Result<Server, std::io::Error> {
    // debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    // needed endpoints
    // rating/?steam_id=<Steam-ID>
    // rating/?profile_id=<ageofempires.com-Profile-ID>

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            .route("/health_check", web::get().to(health_check))
            .route("/matchinfo", web::get().to(matchinfo))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
