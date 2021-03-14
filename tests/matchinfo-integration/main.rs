use reqwest::get;

use dashmap::DashMap;
use serde_json::Value as JsonValue;

use std::{
    self,
    fs,
    io::BufReader,
    sync::Arc,
};

use url::Url;

// Internal Configuration
use pretty_assertions::assert_eq;
use tokio::sync::Mutex;
use transparencies_backend_rs::{
    self,
    domain::{
        data_processing::process_match_info_request,
        types::{
            error::TestCaseError,
            requests::ApiClient,
            testing::{
                TestCase,
                TestCases,
            },
            InMemoryDb,
        },
        util,
    },
    persistence::in_memory_db::data_preloading::{
        create_github_file_list,
        preload_data,
    },
    setup::telemetry::{
        get_subscriber,
        init_subscriber,
    },
};
use wiremock::{
    matchers::method,
    Mock,
    MockServer,
    ResponseTemplate,
};

// Ensure that the `tracing` stack is only initialised once using `lazy_static`
lazy_static::lazy_static! {
static ref TRACING: () = {
let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { "" };
let subscriber = get_subscriber("integration test".into(), filter.into());
init_subscriber(subscriber);
};
}

#[tokio::test]
async fn matchinfo_pipeline_works() {
    let current_dir = std::env::current_dir().unwrap();

    let test_cases = TestCases::default()
        .add_case(
            [
                &format!("{}", current_dir.display()),
                "tests",
                "matchinfo-integration",
                "standard",
            ]
            .iter()
            .collect(),
        )
        .unwrap();

    mock_test_match_info_result(test_cases).await
}

#[tokio::test]
async fn last_match_404() {
    let current_dir = std::env::current_dir().unwrap();

    let test_cases = TestCases::default()
        .add_case(
            [
                &format!("{}", current_dir.display()),
                "tests",
                "matchinfo-integration",
                "last_match_404",
            ]
            .iter()
            .collect(),
        )
        .unwrap();

    mock_test_match_info_result(test_cases).await
}

async fn mock_test_match_info_result(test_cases: TestCases) {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    lazy_static::initialize(&TRACING);

    // Start a background HTTP server on a random local port
    let mock_server = MockServer::start().await;

    let aoe2net_api_roots: Vec<&str> = vec![
        "/api/strings",
        "/api/player/lastmatch",
        "/api/leaderboard",
        "/api/player/ratinghistory",
        "/SiegeEngineers/aoc-reference-data/master/data/",
    ];

    // Preloaded data
    let language_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>> =
        Arc::new(Mutex::new(DashMap::with_capacity(18)));
    let language_mock_responses_clone = language_mock_responses.clone();

    let github_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>> =
        Arc::new(Mutex::new(DashMap::with_capacity(3)));
    let github_mock_responses_clone = github_mock_responses.clone();

    let aoe2net_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>> =
        Arc::new(Mutex::new(DashMap::with_capacity(16)));
    let aoe2net_mock_responses_clone = aoe2net_mock_responses.clone();

    let in_memory_db = Arc::new(Mutex::new(InMemoryDb::default()));
    let in_memory_db_clone = in_memory_db.clone();

    let api_clients = ApiClient::new_with_https(false);

    let github_root = Url::parse(&format!("{}", &mock_server.uri())).unwrap();
    let aoe2_net_root =
        Url::parse(&format!("{}/api", &mock_server.uri())).unwrap();

    // URL
    let missing_link_url =
        Url::parse(&format!("{}/missing", &mock_server.uri())).unwrap();

    // If the request doesn't match any `Mock` mounted on our `MockServer` a 404
    // is returned.
    let status = get(missing_link_url).await.unwrap().status();
    assert_eq!(status.as_u16(), 404);

    let mut ran_once: bool = false;

    for mut test_case in test_cases.0 {
        // each test_case could be as well run in
        // a thread from here on
        load_responses_from_fs(
            &mut test_case,
            aoe2net_mock_responses_clone.clone(),
            language_mock_responses_clone.clone(),
            github_mock_responses_clone.clone(),
        )
        .await
        .unwrap();

        mount_mocks(
            &aoe2net_api_roots,
            test_case.profile_ids.clone(),
            test_case.last_match(),
            &mock_server,
            aoe2net_mock_responses_clone.clone(),
            language_mock_responses_clone.clone(),
            github_mock_responses_clone.clone(),
        )
        .await;

        if ran_once == false {
            preload_data(
                Some(api_clients.github.clone()),
                Some(api_clients.aoe2net.clone()),
                in_memory_db_clone.clone(),
                github_root.clone(),
                aoe2_net_root.clone(),
                None,
                true,
            )
            .await
            .expect("Preloading data failed.");

            ran_once = true;
        }

        let result = process_match_info_request(
            test_case.parsed_request,
            api_clients.aoe2net.clone(),
            aoe2_net_root.to_owned(),
            in_memory_db_clone.clone(),
            None,
        )
        .await;

        assert_eq!(test_case.parsed_result, result);
    }
}

async fn load_responses_from_fs(
    test_case: &mut TestCase,
    aoe2net_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>>,
    language_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>>,
    github_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>>,
) -> Result<(), TestCaseError> {
    for entry in fs::read_dir(test_case.resource_dir()).unwrap() {
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

                                test_case.profile_ids.push(file_name.clone());

                                let val: JsonValue =
                                    serde_json::from_reader(BufReader::new(
                                        fs::File::open(very_new_path).unwrap(),
                                    ))
                                    .unwrap();
                                match new_path
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                {
                                    "rating_history" => {
                                        let guard =
                                            aoe2net_mock_responses.lock().await;
                                        guard.insert(
                                            format!("rh_{}", file_name),
                                            val,
                                        );
                                    }
                                    "leaderboard" => {
                                        let guard =
                                            aoe2net_mock_responses.lock().await;
                                        guard.insert(
                                            format!("ldb_{}", file_name),
                                            val,
                                        );
                                    }
                                    _ => {}
                                }
                            }
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

                    let val: JsonValue = serde_json::from_reader(
                        BufReader::new(fs::File::open(new_path).unwrap()),
                    )
                    .unwrap();
                    {
                        let guard = language_mock_responses.lock().await;
                        guard.insert(file_name, val);
                    }
                }
            }
            "ref-data" => {
                // println!("Folder: {:?}", file_name);
                path.clone().push("/ref-data/");
                // println!("New path: {:?}", path);
                for new_entry in fs::read_dir(path).unwrap() {
                    let new_entry = new_entry.unwrap();
                    let new_path = new_entry.path();
                    let file_name = util::extract_filename(&new_path);

                    let val: JsonValue = serde_json::from_reader(
                        BufReader::new(fs::File::open(new_path).unwrap()),
                    )
                    .unwrap();
                    {
                        let guard = github_mock_responses.lock().await;
                        guard.insert(file_name, val);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn mount_mocks(
    aoe2net_api_roots: &Vec<&str>,
    profile_ids: Vec<String>,
    last_match: &JsonValue,
    mock_server: &MockServer,
    aoe2net_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>>,
    language_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>>,
    github_mock_responses: Arc<Mutex<DashMap<String, JsonValue>>>,
) {
    for root in aoe2net_api_roots.iter() {
        let url_string = &format!("{}", *root);
        match *root {
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
                    #[allow(unused_assignments)]
                    let mut json = JsonValue::default();
                    {
                        let guard = aoe2net_mock_responses.lock().await;

                        json = guard
                            .get(&format!("ldb_{}", profile_id))
                            .map_or(JsonValue::Null, |val| val.value().clone());
                    }
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
                    #[allow(unused_assignments)]
                    let mut json = JsonValue::default();
                    {
                        let guard = aoe2net_mock_responses.lock().await;

                        json = guard
                            .get(&format!("rh_{}", profile_id))
                            .map_or(JsonValue::Null, |val| val.value().clone());
                    }

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
                #[allow(unused_assignments)]
                let mut clone_language_mock_responses: DashMap<
                    std::string::String,
                    JsonValue,
                > = DashMap::with_capacity(18);

                {
                    let guard = language_mock_responses.lock().await;
                    clone_language_mock_responses = guard.clone();
                }

                for multiref in clone_language_mock_responses.iter() {
                    let (lang_short, json) = (multiref.key(), multiref.value());
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
                    #[allow(unused_assignments)]
                    let mut json = JsonValue::default();
                    {
                        let guard = github_mock_responses.lock().await;
                        json = guard
                            .get(&format!("{}", file.name()))
                            .map_or(JsonValue::Null, |val| val.value().clone());
                    }

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
