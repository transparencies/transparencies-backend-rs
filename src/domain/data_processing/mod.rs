//! Starting point of our data processing stage

mod match_data_responder;
pub mod match_info_processor;

use crate::domain::{
    data_processing::match_info_processor::MatchInfoProcessor,
    types::{
        api::{
            MatchInfoRequest,
            MatchInfoResult,
        },
        MatchDataResponses,
    },
};

use tracing::debug;

use std::sync::Arc;
use tokio::{
    self,
    sync::Mutex,
};

use crate::domain::{
    in_memory_db::data_preloading::preload_data,
    types::{
        error::ProcessingError,
        InMemoryDb,
    },
};

use stable_eyre::eyre::Result;

use super::types::error::ApiRequestError;

/// Download static files (Github files, language strings) continously every 10
/// minutes inside a thread
///
/// # Errors
/// Errors get bubble up to the caller
pub async fn get_static_data_inside_thread(
    git_client_clone: reqwest::Client,
    api_client_clone: reqwest::Client,
    in_memory_db_clone: Arc<Mutex<InMemoryDb>>,
) -> Result<(), ApiRequestError> {
    Ok(preload_data(
        api_client_clone.clone(),
        git_client_clone.clone(),
        in_memory_db_clone.clone(),
    )
    .await?)
}

/// Entry point for processing part of `matchinfo` endpoint
///
/// # Errors
/// Results get bubbled up and are handled by the caller
pub async fn process_match_info_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
) -> Result<MatchInfoResult, ProcessingError> {
    debug!(
        "MatchInfoRequest for Game {:?}: {:?} with {:?} in Language {:?}",
        par.game, par.id_type, par.id_number, par.language
    );

    let responses =
        MatchDataResponses::new_with_match_data(par, client, in_memory_db)
            .await?;

    // Debugging responses
    // responses.export_data_to_file();

    let result = MatchInfoProcessor::new_with_response(responses)
        .process()?
        .assemble()?;

    debug!("MatchInfoResult: {:#?}", result);

    // Debugging result
    result.export_data_to_file();

    Ok(result)
}
