//! Executable for managing aoe-reference-data files.
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// TODO: Temporary, remove later
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

// Error handling
#[macro_use]
extern crate log;
use eyre::Error;
use human_panic::setup_panic;
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use simple_log::LogConfigBuilder;
use std::{
    env,
    process,
};
use warp::{
    http::StatusCode,
    Filter,
};

// CLI
use structopt::StructOpt;

// Internal Configuration
use transparencies_backend_rs::{
    domain::api_handler::client::ApiRequest,
    routes::health_check,
    setup::{
        cli::CommandLineSettings,
        configuration::get_configuration,
    },
};

#[tokio::main]
async fn main() {
    // Install the panic and error report handlers
    // stable_eyre::install();

    // Webserver logging
    if env::var_os("RUST_LOG").is_none() {
        // TODO Deactivate Debug logs
        env::set_var("RUST_LOG", "transparencies=debug");
        // Access logs
        // env::set_var("RUST_LOG", "transparencies=info");
    }

    // Human Panic. Only enabled when *not* debugging.
    #[cfg(not(debug_assertions))]
    {
        setup_panic!(Metadata {
            name: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: "the transparencies authors".into(),
            homepage: "https://github.com/transparencies/transparencies-backend-rs/issues".into(),
        });
    }

    // Setting up configuration
    let _configuration =
        get_configuration().expect("Failed to read configuration.");

    // Calling the command line parsing logic with the argument values
    let cli_args = CommandLineSettings::from_args();

    // If `debug` flag is set, we use a logfile
    if cli_args.debug {
        // Setting up logfile
        let log_setup = LogConfigBuilder::builder()
            .path(&cli_args.log_file_path)
            .size(1 * 100)
            .roll_count(10)
            .level(&cli_args.log_level)
            .output_file()
            .output_console()
            .build();

        simple_log::new(log_setup.clone()).expect("Log setup failed!");
        debug!("Log config: {:?}", &log_setup);
        trace!("Logs were set up.");
    }

    let _db = models::blank_db();

    let api = filters::transparencies();

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("transparencies"));

    warp::serve(routes)
        // Activate after certificates have been received from Let's Encrypt
        // .tls()
        // .cert_path("examples/tls/cert.pem")
        // .key_path("examples/tls/key.rsa")
        .run(([127, 0, 0, 1], 8000))
        .await;
}

mod filters {
    use super::{
        handlers,
        models::{
            Db,
            ListOptions,
            Todo,
        },
    };
    use warp::Filter;

    pub fn transparencies(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        health_check()
        // .or(matchinfo())
    }

    /// GET /health_check
    pub fn health_check(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("health_check")
            .and(warp::get())
            .and_then(handlers::return_health_check)
    }

    // /// GET /matchinfo?idtype=steamid&idnumber=12318931981421
    // pub fn matchinfo() -> impl Filter<Extract = impl warp::Reply, Error =
    // warp::Rejection> + Clone {     warp::path!("todos")
    //         .and(warp::get())
    //         .and(warp::query::<ListOptions>())
    //         .and(with_db(db))
    //         .and_then(handlers::list_todos)
    // }
}

/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
mod handlers {
    use super::models::{
        Db,
        MatchInfoRequest,
        Todo,
    };
    use std::convert::Infallible;
    use transparencies_backend_rs::domain::api_handler::{
        client::*,
        response::aoe2net::last_match::PlayerLastMatch,
    };
    use warp::http::StatusCode;

    pub async fn return_health_check() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply())
    }

    pub async fn return_matchinfo(
        opts: MatchInfoRequest
    ) -> Result<impl warp::Reply, Infallible> {
        let request: ApiRequest = ApiRequestBuilder::default()
            .root("https://aoe2.net/api/")
            .endpoint("player/lastmatch")
            .query(vec![
                ("game".to_string(), "aoe2de".to_string()),
                (
                    opts.id_type.unwrap().to_string(),
                    opts.id_number.unwrap().to_string(),
                ),
            ])
            .build()
            .unwrap();

        let response = request.execute::<PlayerLastMatch>().await.unwrap();

        // Just return a JSON array of todos, applying the limit and offset.
        // let todos = db.lock().await;
        // let todos: Vec<Todo> = todos
        //     .clone()
        //     .into_iter()
        //     .skip(opts.offset.unwrap_or(0))
        //     .take(opts.limit.unwrap_or(std::usize::MAX))
        //     .collect();
        Ok(warp::reply::json(&response))
    }

    // pub async fn create_todo(create: Todo, db: Db) -> Result<impl
    // warp::Reply, Infallible> {     log::debug!("create_todo: {:?}",
    // create);

    //     let mut vec = db.lock().await;

    //     for todo in vec.iter() {
    //         if todo.id == create.id {
    //             log::debug!("    -> id already exists: {}", create.id);
    //             // Todo with id already exists, return `400 BadRequest`.
    //             return Ok(StatusCode::BAD_REQUEST);
    //         }
    //     }

    //     // No existing Todo with id, so insert and return `201 Created`.
    //     vec.push(create);

    //     Ok(StatusCode::CREATED)
    // }

    // pub async fn update_todo(
    //     id: u64,
    //     update: Todo,
    //     db: Db,
    // ) -> Result<impl warp::Reply, Infallible> {
    //     log::debug!("update_todo: id={}, todo={:?}", id, update);
    //     let mut vec = db.lock().await;

    //     // Look for the specified Todo...
    //     for todo in vec.iter_mut() {
    //         if todo.id == id {
    //             *todo = update;
    //             return Ok(StatusCode::OK);
    //         }
    //     }

    //     log::debug!("    -> todo id not found!");

    //     // If the for loop didn't return OK, then the ID doesn't exist...
    //     Ok(StatusCode::NOT_FOUND)
    // }

    // pub async fn delete_todo(id: u64, db: Db) -> Result<impl warp::Reply,
    // Infallible> {     log::debug!("delete_todo: id={}", id);

    //     let mut vec = db.lock().await;

    //     let len = vec.len();
    //     vec.retain(|todo| {
    //         // Retain all Todos that aren't this id...
    //         // In other words, remove all that *are* this id...
    //         todo.id != id
    //     });

    //     // If the vec is smaller, we found and deleted a Todo!
    //     let deleted = vec.len() != len;

    //     if deleted {
    //         // respond with a `204 No Content`, which means successful,
    //         // yet no body expected...
    //         Ok(StatusCode::NO_CONTENT)
    //     } else {
    //         log::debug!("    -> todo id not found!");
    //         Ok(StatusCode::NOT_FOUND)
    //     }
    // }
}

mod models {
    use serde::{
        Deserialize,
        Serialize,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

    /// So we don't have to tackle how different database work, we'll just use
    /// a simple in-memory DB, a vector synchronized by a mutex.
    pub type Db = Arc<Mutex<Vec<Todo>>>;

    pub fn blank_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Todo {
        pub id: u64,
        pub text: String,
        pub completed: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct MatchInfoRequest {
        pub id_type: Option<String>,
        pub id_number: Option<String>,
    }

    // The query parameters for list_todos.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub offset: Option<usize>,
        pub limit: Option<usize>,
    }
}

#[cfg(test)]
mod tests {
    use warp::{
        http::StatusCode,
        test::request,
    };

    use super::{
        filters,
        models::{
            self,
            Todo,
        },
    };

    #[tokio::test]
    async fn test_post() {
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&Todo {
                id: 1,
                text: "test 1".into(),
                completed: false,
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_post_conflict() {
        let db = models::blank_db();
        db.lock().await.push(todo1());
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_put_unknown() {
        let _ = pretty_env_logger::try_init();
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("PUT")
            .path("/todos/1")
            .header("authorization", "Bearer admin")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    fn todo1() -> Todo {
        Todo {
            id: 1,
            text: "test 1".into(),
            completed: false,
        }
    }
}
