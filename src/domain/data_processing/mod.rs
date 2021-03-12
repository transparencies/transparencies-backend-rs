//! Starting point of our data processing stage

mod match_data_responder;
pub mod match_info_processor;

use crate::domain::{
    data_processing::match_info_processor::MatchInfoProcessor,
    types::{
        api::{
            ErrorMessageToFrontend,
            MatchInfoRequest,
            MatchInfoResult,
        },
        error::ResponderError,
        MatchDataResponses,
    },
};

use tracing::debug;

use std::sync::Arc;
use tokio::{
    self,
    sync::Mutex,
};

use crate::domain::types::{
    error::ProcessingError,
    InMemoryDb,
};

use url::Url;

use stable_eyre::eyre::Result;

/// Entry point for processing part of `matchinfo` endpoint
///
/// # Errors
/// Results get bubbled up and are handled by the caller
pub async fn process_match_info_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    root: Url,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    export_path: Option<&str>,
) -> Result<MatchInfoResult, ProcessingError> {
    debug!(
        "MatchInfoRequest for Game {:?}: {:?} with {:?} in Language {:?}",
        par.game, par.id_type, par.id_number, par.language
    );

    let aoe2net_folder = export_path.map_or("", |path| path);

    let responses = MatchDataResponses::new_with_match_data(
        par,
        client,
        in_memory_db,
        aoe2net_folder,
        root,
    )
    .await;

    #[allow(unused_assignments)]
    let mut result = MatchInfoResult::new();

    match responses {
        Err(err) => match err {
            ResponderError::DerankedPlayerDetected => {
                result = MatchInfoResult::builder()
                    .error_message(
                        ErrorMessageToFrontend::DerankedPlayerDetected,
                    )
                    .build()
            }
            _ => {
                result = MatchInfoResult::builder()
                    .error_message(
                        ErrorMessageToFrontend::GenericResponderError(format!(
                            "{}",
                            err
                        )),
                    )
                    .build()
            }
        },
        Ok(response) => {
            result = MatchInfoProcessor::new_with_response(response)
                .process()?
                .assemble()?
        }
    }

    Ok(result)
}
