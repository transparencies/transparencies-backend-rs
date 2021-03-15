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
        error::{
            ErrorMessageToFrontend,
            ResponderError,
        },
        MatchDataResponses,
    },
};

use tracing::error;
use tracing_futures::Instrument;

use std::{
    path::PathBuf,
    sync::Arc,
};
use tokio::{
    self,
    sync::Mutex,
};

use crate::domain::types::{
    error::ProcessingError,
    InMemoryDb,
};

use url::Url;
use uuid::Uuid;

/// Entry point for processing part of `matchinfo` endpoint
///
/// # Errors
/// Results get bubbled up and are handled by the caller
#[tracing::instrument(
name = "Processing MatchInfoRequest",
skip(client, root, in_memory_db, export_path),
fields(
request_id = %Uuid::new_v4(),
id_type = %par.id_type,
id_number = %par.id_number
)
)]
pub async fn build_result(
    par: MatchInfoRequest,
    client: reqwest::Client,
    root: Url,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    export_path: Option<PathBuf>,
) -> MatchInfoResult {
    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Querying for data from APIs...");

    let responses = MatchDataResponses::with_match_data(
        par.clone(),
        client,
        in_memory_db,
        export_path,
        root,
    )
    .instrument(query_span)
    .await;

    #[allow(unused_assignments)]
    let mut result = MatchInfoResult::default();

    match responses {
        Err(err) => match err {
            ResponderError::LastMatchNotFound => {
                error!("Failed with {:?}", err);
                result = MatchInfoResult::builder()
                    .error_message(ErrorMessageToFrontend::HardFail(format!(
                        "MatchInfo processing failed: {}",
                        err.to_string()
                    )))
                    .build();
            }
            _ => {
                error!("Failed with {:?}", err);
                result = MatchInfoResult::builder()
                    .error_message(
                        ErrorMessageToFrontend::GenericResponderError(format!(
                            "{}",
                            err
                        )),
                    )
                    .build();
            }
        },
        Ok(response) => {
            // Process the Responses
            let processed_result =
                MatchInfoProcessor::with_response(response).process();

            // Handle all the errors and make sure, we always return a
            // `MatchInfoResult`
            if let Err(err) = processed_result {
                match err {
                    ProcessingError::NotRankedLeaderboard(_) => {
                        error!("Failed with {:?}", err);
                        result = MatchInfoResult::builder()
                            .error_message(ErrorMessageToFrontend::HardFail(
                                format!(
                                    "MatchInfo processing failed: {}",
                                    err.to_string()
                                ),
                            ))
                            .build();
                    }
                    _ => {
                        error!("Failed with {:?}", err);
                        result = MatchInfoResult::builder()
                            .error_message(ErrorMessageToFrontend::HardFail(format!(
                                "MatchInfo processing failed for {:?}:{:?} with {}",
                                par.id_type,
                                par.id_number,
                                err.to_string()
                            )))
                            .build();
                    }
                }
            }
            else {
                result = processed_result
                    .unwrap()
                    .assemble()
                    .expect("MatchInfoResult assembly failed.");
            }
        }
    }
    result
}
