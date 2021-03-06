#[tokio::main]
async fn main() {
    get_resources_for_tests().await;
}

async fn get_resources_for_tests() {
    use std::{
        path::PathBuf,
        str::FromStr,
        sync::Arc,
    };
    use tokio::sync::Mutex;
    use transparencies_backend_rs::domain::{
        data_processing::process_match_info_request,
        in_memory_db::data_preloading::preload_data,
        types::{
            api::MatchInfoRequest,
            requests::ApiClient,
            InMemoryDb,
        },
    };

    let export_flag = true;

    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let api_clients = ApiClient::default();
    let match_info_request = MatchInfoRequest {
        language: Some("en".to_string()),
        game: Some("aoe2de".to_string()),
        id_type: "profile_id".to_string(),
        id_number: "196240".to_string(),
    };

    preload_data(
        api_clients.github.clone(),
        api_clients.aoe2net.clone(),
        in_memory_db.clone(),
        export_flag,
    )
    .await
    .expect("Preloading data failed.");

    let result = process_match_info_request(
        match_info_request,
        api_clients.aoe2net.clone(),
        in_memory_db.clone(),
        export_flag,
    )
    .await
    .expect("Matchinfo processing failed.");

    if export_flag {
        result.export_data_to_file(
            PathBuf::from_str("tests/integration/resources").unwrap(),
        );
    }
}
