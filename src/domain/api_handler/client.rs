//! Core client logic of the application

use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use stable_eyre::eyre::{
    eyre,
    Report,
    Result,
    WrapErr,
};

use ::serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
use std::{
    collections::HashMap,
    time::Duration,
};

use crate::domain::api_handler::response::{
    aoc_ref::{
        platforms,
        players,
        teams,
    },
    aoe2net::last_match::PlayerLastMatch,
};

use std::fmt;

// App-Name as USERAGENT
static APP_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Debug, Clone)]
pub enum FileFormat {
    Toml,
    Json,
    Yaml,
    Ron,
    Xml,
    Url,
    Uninitialized,
}

impl Default for FileFormat {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}

#[derive(Builder, Debug, Clone)]
pub struct File {
    name: String,
    ext: FileFormat,
}

impl Default for File {
    fn default() -> Self {
        Self {
            name: String::new(),
            ext: FileFormat::default(),
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}.{}", self.name, self.ext)
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub aoe2net: reqwest::Client,
}

impl Default for ApiClient {
    fn default() -> Self {
        // Duration for timeouts
        let request_timeout: Duration = Duration::new(5, 0);
        let connection_timeout: Duration = Duration::new(600, 0);

        Self {
            aoe2net: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(request_timeout)
                .connect_timeout(connection_timeout)
                .use_rustls_tls()
                .https_only(true)
                .build()
                .unwrap(),
        }
    }
}

// root: https://raw.githubusercontent.com
// user: SiegeEngineers
// repo: aoc-reference-data
// uri: master/data
// file: File {
//         name: players
//         ext: FileFormat::Yaml
// }
// https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml
// &format!("{}/{}/{}/{}/{}", &self.root, &self.user, &self.repo, &self.uri,
// &self.file)
#[derive(Builder, Debug)]
#[builder(public, setter(into))]
pub struct GithubFileRequest {
    #[builder(setter(skip))]
    client: reqwest::Client,
    root: String,
    user: String,
    repo: String,
    uri: String,
    file: File,
}

impl Default for GithubFileRequest {
    fn default() -> Self {
        // Duration for timeouts
        let request_timeout: Duration = Duration::new(5, 0);
        let connection_timeout: Duration = Duration::new(5, 0);

        Self {
            client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .timeout(request_timeout)
                .connect_timeout(connection_timeout)
                .use_rustls_tls()
                .https_only(true)
                .build()
                .unwrap(),
            root: String::new(),
            user: String::new(),
            repo: String::new(),
            uri: String::new(),
            file: File::default(),
        }
    }
}

impl GithubFileRequest {
    pub async fn execute<R>(&self) -> Result<Response<R>>
    where R: for<'de> serde::Deserialize<'de> {
        todo!();
        // TODO: Create Response from Request
        // Deserialize depending on `FileFormat` into Response
        // Ok(Response {
        //     response: self
        //         .client
        //         .get(&format!(
        //             "{}/{}/{}/{}/{}",
        //             &self.root, &self.user, &self.repo, &self.uri, &self.file
        //         ))
        //         .send()
        //         .await
        //         .unwrap(),
        // })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response<T> {
    pub response: T,
}

#[derive(Builder, Debug)]
#[builder(public, setter(into))]
pub struct ApiRequest {
    // #[builder(setter(skip))]
    client: reqwest::Client,
    root: String,
    endpoint: String,
    query: Vec<(String, String)>,
}

impl Default for ApiRequest {
    fn default() -> Self {
        Self {
            client: reqwest::Client::default(),
            root: String::new(),
            endpoint: String::new(),
            query: Vec::new(),
        }
    }
}

impl ApiRequest {
    pub async fn execute<R>(&self) -> Result<R>
    where R: for<'de> serde::Deserialize<'de> {
        Ok(self
            .client
            .get(&format!("{}/{}", &self.root, &self.endpoint))
            .query(&self.query)
            .send()
            .await?
            .json::<R>()
            .await?)
    }
}

// TODO: Pull file(s) from github
// e.g. https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml

//     /// Fills members of a DataLists object with data deserialised from
//     /// file paths that were given via the command line arguments
//     /// players_input_path, teams_input_path, platforms_input_path
//     pub fn new_from_cli_config(config: cli::Args) -> Result<DataLists,
// Report> {         // TODO: Error handling
//         let mut data = DataLists::new();

//         // Deserializing Player file
//         match config.players_input_path {
//             Some(k) => {
//                 debug!("Player file given: {:?}", &k);
//                 data.player_list =
//                     DataLists::deserialize_list_to_vec_from_file::<
//                         players::Players,
//                     >(&k)
//                     .expect("Parsing of players file failed.");
//             }
//             None => {}
//         };

//         // Deserializing Teams file
//         match config.teams_input_path {
//             Some(k) => {
//                 debug!("Teams file given: {:?}", &k);
//                 data.team_list =
// DataLists::deserialize_list_to_vec_from_file::<teams::Teams>(&k)
// .expect("Parsing of teams file failed.");             }
//             None => {}
//         };

//         // Deserializing Platforms file
//         match config.platforms_input_path {
//             Some(k) => {
//                 debug!("Platforms file given: {:?}", &k);
//                 data.platform_list =
//                     DataLists::deserialize_list_to_vec_from_file::<
//                         platforms::Platforms,
//                     >(&k)
//                     .expect("Parsing of platforms file failed.");
//             }
//             None => {}
//         }

//         // Returning DataLists Struct
//         Ok(data)
//     }

//     /// Generic function to deserialize a given file with `serde_any`
//     /// from a path into a vector that contains a special datatype T
//     fn deserialize_list_to_vec_from_file<T>(
//         path: &dyn AsRef<Path>
//     ) -> Result<Vec<T>, Report>
//     where T: DeserializeOwned {
//         // Open the file in read-only mode with buffer.
//         // let file = File::open(&path)?;
//         // let reader = BufReader::new(file);

//         // Read the list contents of the file as an instance of `Vec<T>`.
//         let list: Vec<T> = serde_any::from_file(path)
//             .expect("Parsing of the data file failed.");
//         Ok(list)
//     }

// /// Matches the extension of a given filename and
// /// returns the datatype for serde
// pub fn get_file_type_from_extension(
//     ext: &str
// ) -> Option<serde_any::Format> {
//     match ext {
//         "yaml" | "yml" => Some(serde_any::Format::Yaml),
//         "json" => Some(serde_any::Format::Json),
//         "toml" => Some(serde_any::Format::Toml),
//         _ => None,
//     }
// }
