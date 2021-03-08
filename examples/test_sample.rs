use std::{
    collections::HashMap,
    fs,
    io::BufReader,
    path::{
        Path,
        PathBuf,
    },
    sync::Arc,
};

use reqwest::{
    get,
    Url,
};
use tokio::sync::Mutex;
use transparencies_backend_rs::domain::{
    data_processing::process_match_info_request,
    in_memory_db::data_preloading::{
        create_github_file_list,
        preload_data,
    },
    types::{
        api::{
            MatchInfoRequest,
            MatchInfoResult,
        },
        ApiClient,
        InMemoryDb,
    },
    util,
};
use wiremock::{
    matchers::method,
    Mock,
    MockServer,
    ResponseTemplate,
};

#[tokio::main]
async fn main() {
    // Start a background HTTP server on a random local port
    let mock_server = MockServer::start().await;

    let current_dir = std::env::current_dir().unwrap();
    let resources_root_dir = "/tests/matchinfo-integration/resources";

    let mut profile_ids: Vec<String> = Vec::with_capacity(8);

    let mut language_mock_responses: HashMap<String, serde_json::Value> =
        HashMap::with_capacity(18);

    let mut aoe2net_mock_responses: HashMap<String, serde_json::Value> =
        HashMap::with_capacity(18);

    let mut github_mock_responses: HashMap<String, serde_json::Value> =
        HashMap::with_capacity(3);

    let mut last_match: serde_json::Value = serde_json::Value::Null;

    let mut ron_result = MatchInfoResult::default();

    load_responses_from_fs(
        current_dir,
        resources_root_dir,
        &mut profile_ids,
        &mut aoe2net_mock_responses,
        &mut last_match,
        &mut language_mock_responses,
        &mut ron_result,
        &mut github_mock_responses,
    );

    let aoe2net_api_roots: Vec<&str> = vec![
        "/api/strings",
        "/api/player/lastmatch",
        "/api/leaderboard",
        "/api/player/ratinghistory",
        "/SiegeEngineers/aoc-reference-data/master/data/",
    ];

    mount_mocks(
        aoe2net_api_roots,
        profile_ids,
        last_match,
        &mock_server,
        aoe2net_mock_responses,
        language_mock_responses,
        github_mock_responses,
    )
    .await;

    // URL
    let missing_link_url =
        Url::parse(&format!("{}/missing", &mock_server.uri())).unwrap();

    // If the request doesn't match any `Mock` mounted on our `MockServer` a 404
    // is returned.
    let status = get(missing_link_url).await.unwrap().status();
    assert_eq!(status.as_u16(), 404);

    // REAL TESTING
    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let in_memory_db_clone = in_memory_db.clone();

    let api_clients = ApiClient::new_with_https(false);

    let match_info_request = MatchInfoRequest {
        language: Some("en".to_string()),
        game: Some("aoe2de".to_string()),
        id_type: "profile_id".to_string(),
        id_number: "196240".to_string(),
    };

    preload_data(
        Some(api_clients.github.clone()),
        Some(api_clients.aoe2net.clone()),
        in_memory_db_clone.clone(),
        &format!("{}", &mock_server.uri()),
        &format!("{}", &mock_server.uri()),
        None,
        true,
    )
    .await
    .expect("Preloading data failed.");

    let result = process_match_info_request(
        match_info_request,
        api_clients.aoe2net.clone(),
        in_memory_db_clone.clone(),
        None,
        &format!("{}", &mock_server.uri()),
    )
    .await
    .expect("Matchinfo processing failed.");

    assert_eq!(ron_result, result);
}

fn load_responses_from_fs(
    current_dir: PathBuf,
    resources_root_dir: &str,
    profile_ids: &mut Vec<String>,
    aoe2net_mock_responses: &mut HashMap<String, serde_json::Value>,
    last_match: &mut serde_json::Value,
    language_mock_responses: &mut HashMap<String, serde_json::Value>,
    ron_result: &mut MatchInfoResult,
    github_mock_responses: &mut HashMap<String, serde_json::Value>,
) {
    for entry in fs::read_dir(
        Path::new(&format!("{}{}", current_dir.display(), resources_root_dir))
            .as_os_str(),
    )
    .unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();

        let file_name = path.file_name().unwrap();

        match file_name.to_str().unwrap() {
            "aoe2net" => {
                path.clone().push("/aoe2net/");

                for new_entry in fs::read_dir(path).unwrap() {
                    let new_entry = new_entry.unwrap();
                    let new_path = new_entry.path();

                    match new_path.file_name().unwrap().to_str().unwrap() {
                        "rating_history" | "leaderboard" => {
                            for very_new_entry in
                                fs::read_dir(new_path.clone()).unwrap()
                            {
                                let very_new_entry = very_new_entry.unwrap();
                                let very_new_path = very_new_entry.path();

                                let file_name =
                                    util::extract_filename(&very_new_path);

                                profile_ids.push(file_name.clone());

                                let file = fs::File::open(very_new_path)
                                    .expect("file should open read only");
                                let reader = BufReader::new(file);
                                match new_path
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                {
                                    "rating_history" => {
                                        aoe2net_mock_responses.insert(
                                            format!("rh_{}", file_name),
                                            serde_json::from_reader(reader)
                                                .expect(
                                                "file should be proper JSON",
                                            ),
                                        );
                                    }
                                    "leaderboard" => {
                                        aoe2net_mock_responses.insert(
                                            format!("ldb_{}", file_name),
                                            serde_json::from_reader(reader)
                                                .expect(
                                                "file should be proper JSON",
                                            ),
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "last_match.json" => {
                            let file = fs::File::open(new_path)
                                .expect("file should open read only");
                            let reader = BufReader::new(file);
                            *last_match = serde_json::from_reader(reader)
                                .expect("file should be proper JSON");
                        }
                        _ => {}
                    }
                }
            }
            "languages" => {
                path.clone().push("/languages/");
                for new_entry in fs::read_dir(path).unwrap() {
                    let new_entry = new_entry.unwrap();
                    let new_path = new_entry.path();
                    let file_name = util::extract_filename(&new_path);

                    let file = fs::File::open(new_path)
                        .expect("file should open read only");
                    let reader = BufReader::new(file);
                    language_mock_responses.insert(
                        file_name,
                        serde_json::from_reader(reader)
                            .expect("file should be proper JSON"),
                    );
                }
            }
            "match_info_result.ron" => {
                let file =
                    fs::File::open(path).expect("file should open read only");
                let reader = BufReader::new(file);
                *ron_result = ron::de::from_reader(reader).unwrap();
            }
            "ref-data" => {
                // println!("Folder: {:?}", file_name);
                path.clone().push("/ref-data/");
                // println!("New path: {:?}", path);
                for new_entry in fs::read_dir(path).unwrap() {
                    let new_entry = new_entry.unwrap();
                    let new_path = new_entry.path();
                    let file_name = util::extract_filename(&new_path);

                    let file = fs::File::open(new_path)
                        .expect("file should open read only");
                    let reader = BufReader::new(file);
                    github_mock_responses.insert(
                        file_name,
                        serde_json::from_reader(reader)
                            .expect("file should be proper JSON"),
                    );
                }
            }
            _ => {}
        }
    }
}

async fn mount_mocks(
    aoe2net_api_roots: Vec<&str>,
    profile_ids: Vec<String>,
    last_match: serde_json::Value,
    mock_server: &MockServer,
    aoe2net_mock_responses: HashMap<String, serde_json::Value>,
    language_mock_responses: HashMap<String, serde_json::Value>,
    github_mock_responses: HashMap<String, serde_json::Value>,
) {
    for root in aoe2net_api_roots {
        let url_string = &format!("{}", root.clone());
        match root {
            "/api/player/lastmatch" => {
                // "/api/player/lastmatch?game=aoe2de&profile_id=196240"
                for profile_id in &profile_ids {
                    Mock::given(method("GET"))
                        .and(wiremock::matchers::path(url_string.to_string()))
                        .and(wiremock::matchers::query_param("game", "aoe2de"))
                        .and(wiremock::matchers::query_param(
                            "profile_id",
                            profile_id,
                        ))
                        .respond_with(
                            ResponseTemplate::new(200)
                                .set_body_json(&last_match),
                        )
                        .mount(mock_server)
                        .await;
                }
            }
            "/api/leaderboard" => {
                // "/api/leaderboard?game=aoe2de&profile_id=196240&
                // leaderboard_id=3"
                for profile_id in &profile_ids {
                    let json = aoe2net_mock_responses
                        .get(&format!("ldb_{}", profile_id))
                        .map_or(&serde_json::Value::Null, |val| val);

                    Mock::given(method("GET"))
                        .and(wiremock::matchers::path(url_string.to_string()))
                        .and(wiremock::matchers::query_param("game", "aoe2de"))
                        .and(wiremock::matchers::query_param(
                            "profile_id",
                            profile_id,
                        ))
                        .and(wiremock::matchers::query_param(
                            "leaderboard_id",
                            &last_match["last_match"]["leaderboard_id"]
                                .to_string(),
                        ))
                        .respond_with(
                            ResponseTemplate::new(200).set_body_json(json),
                        )
                        .mount(mock_server)
                        .await;
                }
            }
            "/api/player/ratinghistory" => {
                // "/api/player/ratinghistory?game=aoe2de&profile_id=196240&
                // leaderboard_id=3&count=1"
                for profile_id in &profile_ids {
                    let json = aoe2net_mock_responses
                        .get(&format!("rh_{}", profile_id))
                        .map_or(&serde_json::Value::Null, |val| val);

                    Mock::given(method("GET"))
                        .and(wiremock::matchers::path(url_string.to_string()))
                        .and(wiremock::matchers::query_param("game", "aoe2de"))
                        .and(wiremock::matchers::query_param(
                            "profile_id",
                            profile_id,
                        ))
                        .and(wiremock::matchers::query_param(
                            "count",
                            "1".to_string(),
                        ))
                        .and(wiremock::matchers::query_param(
                            "leaderboard_id",
                            &last_match["last_match"]["leaderboard_id"]
                                .to_string(),
                        ))
                        .respond_with(
                            ResponseTemplate::new(200).set_body_json(json),
                        )
                        .mount(mock_server)
                        .await;
                }
            }
            "/api/strings" => {
                // Language mocking
                for (lang_short, json) in &language_mock_responses {
                    let url_string = &format!("{}", root.clone());
                    Mock::given(method("GET"))
                        .and(wiremock::matchers::path(url_string.to_string()))
                        .and(wiremock::matchers::query_param("game", "aoe2de"))
                        .and(wiremock::matchers::query_param(
                            "language", lang_short,
                        ))
                        .respond_with(
                            ResponseTemplate::new(200).set_body_json(json),
                        )
                        .mount(mock_server)
                        .await;
                }
            }
            "/SiegeEngineers/aoc-reference-data/master/data/" => {
                // Github FileRequest Mock
                // https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/
                for file in create_github_file_list() {
                    let json = github_mock_responses
                        .get(&format!("{}", file.name()))
                        .map_or(&serde_json::Value::Null, |val| val);

                    let url_string = &format!("{}{}", root.clone(), file);
                    Mock::given(method("GET"))
                        .and(wiremock::matchers::path(url_string.to_string()))
                        .respond_with(
                            ResponseTemplate::new(200).set_body_json(json),
                        )
                        .mount(mock_server)
                        .await;
                }
            }
            _ => {}
        }
    }
}