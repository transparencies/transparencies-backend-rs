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
use stable_eyre::eyre::Result;
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
pub async fn process_match_info_request(
    par: MatchInfoRequest,
    client: reqwest::Client,
    root: Url,
    in_memory_db: Arc<Mutex<InMemoryDb>>,
    export_path: Option<PathBuf>,
) -> Result<MatchInfoResult, ProcessingError> {
    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Querying for data from APIs...");

    let responses = MatchDataResponses::new_with_match_data(
        par.clone(),
        client,
        in_memory_db,
        export_path,
        root,
    )
    .instrument(query_span)
    .await;

    #[allow(unused_assignments)]
    let mut result = MatchInfoResult::new();

    match responses {
        Err(err) => match err {
            ResponderError::LastMatchNotFound => {
                error!("Failed with {:?}", err);
                result = MatchInfoResult::builder()
                    .error_message(ErrorMessageToFrontend::LastMatchNotFound)
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
            result = MatchInfoProcessor::new_with_response(response)
                .process()?
                .assemble()?;
        }
    }

    Ok(result)
}
