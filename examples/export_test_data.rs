#[tokio::main]
async fn main() {
    get_resources_for_tests().await;
}

async fn get_resources_for_tests() {
    use std::sync::Arc;
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

    let mut guard = in_memory_db.lock().await.clone();

    guard.github_file_content.index().expect("Indexing failed.");

    process_match_info_request(
        match_info_request,
        api_clients.aoe2net.clone(),
        in_memory_db.clone(),
        export_flag,
    )
    .await
    .expect("Matchinfo processing failed.");
}
