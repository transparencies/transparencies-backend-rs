use crate::routes::{
    health_check,
    matchinfo,
};
use actix_web::{
    dev::Server,
    middleware,
    web,
    web::Data,
    App,
    HttpServer,
};
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};

use crate::{
    domain::api_handler::client::*,
    setup::cli::CommandLineSettings,
};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Entrypoint for the library part of the Executable's main function

pub fn run_server(
    listener: TcpListener,
    config: &CommandLineSettings,
    api_client: ApiRequest,
) -> eyre::Result<Server, std::io::Error> {
    debug!("CLI config: {:#?}", config);
    trace!("We are inside the run-function!");

    // Share api client among all App instances
    let api_client = Data::new(api_client);

    // Create server with Endpoints
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger)
            // enable logger - always register actix-web Logger middleware last
            // TODO: Enable Logging
            // .wrap(middleware::Logger::default())
            .service(health_check)
            .service(matchinfo)
            .app_data(api_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
