use crate::routes::{health_check, matchinfo};
use actix_web::{dev::Server, middleware, web, web::Data, App, HttpServer};
use log::{debug, error, info, trace, warn};

use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Entrypoint for the library part of the Executable's main function

pub fn run(
    listener: TcpListener, /*  */

                           /* config: &cli::Args */
) -> eyre::Result<Server, std::io::Error> {
    // debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    // Create server with Endpoints
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            // enable logger - always register actix-web Logger middleware last
            // TODO: Enable Logging
            // .wrap(middleware::Logger::default())
            .service(health_check)
            .service(matchinfo)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
