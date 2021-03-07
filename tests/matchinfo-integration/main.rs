use std::{
    collections::HashMap,
    fs,
};

use reqwest::{
    get,
    Url,
};
use wiremock::{
    matchers::{
        method,
        path,
        query_param,
    },
    Mock,
    MockServer,
    ResponseTemplate,
};

#[tokio::main]
async fn main() {
    let current_dir = std::env::current_dir().unwrap();

    let _mock_responses: HashMap<String, serde_json::Value> =
        HashMap::with_capacity(41);

    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // TODO: use name of file when parsing into HashMap
        // TODO: if the name of the file contains:
        // `leaderboard_` | `rating_history_` then parse the profile ID from it
        // as well and use it in the response path and as the hashmap, so we can
        // easily look it up with `mock_entry.get()` TODO: if the name
        // contains `language_` parse into another hashmap or
        // so (needs more thinking)

        let _file = fs::File::open(path).expect("file should open read only");

        // mock_responses.insert( <TODO> ,
        //     serde_json::from_reader(file).expect("file should be proper
        // JSON"), );
    }

    // let first_name = json
    //     .get("FirstName")
    //     .expect("file should have FirstName key");

    // Start a background HTTP server on a random local port
    let mock_server = MockServer::start().await;

    // URL
    let health_check_url =
        Url::parse(&format!("{}/health_check", &mock_server.uri())).unwrap();
    let missing_link_url =
        Url::parse(&format!("{}/missing", &mock_server.uri())).unwrap();

    // Mock Requests needed
    let mock_paths: Vec<&str> = vec![
    "/api/leaderboard?game=aoe2de&profile_id=459658&leaderboard_id=3",
    "/api/player/ratinghistory?game=aoe2de&profile_id=459658&leaderboard_id=3&count=1",
    "/api/leaderboard?game=aoe2de&profile_id=196240&leaderboard_id=3",
    "/api/player/ratinghistory?game=aoe2de&profile_id=196240&leaderboard_id=3&count=1",
    "/api/player/lastmatch?game=aoe2de&profile_id=196240",
    "/api/strings?game=aoe2de&language=zh-TW",
    ];

    for mock_path in &mock_paths {
        let url_string = Url::parse(&format!("{}", *mock_path)).unwrap();
        Mock::given(method("GET"))
            .and(path(url_string.to_string()))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
    }

    for mock_path in &mock_paths {
        let url_string =
            Url::parse(&format!("{}{}", &mock_server.uri(), *mock_path))
                .unwrap();
        // If we probe the MockServer using any HTTP client it behaves as
        // expected.
        let status = get(url_string).await.unwrap().status();
        assert_eq!(status.as_u16(), 200);
    }

    // Arrange the behaviour of the MockServer adding a Mock:
    // when it receives a GET request on '/health_check' it will respond with a
    // 200.
    Mock::given(method("GET"))
        .and(path("/health_check"))
        .respond_with(ResponseTemplate::new(200))
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;

    // If we probe the MockServer using any HTTP client it behaves as expected.
    let status = get(health_check_url).await.unwrap().status();
    assert_eq!(status.as_u16(), 200);

    // If the request doesn't match any `Mock` mounted on our `MockServer` a 404
    // is returned.
    let status = get(missing_link_url).await.unwrap().status();
    assert_eq!(status.as_u16(), 404);

    Mock::given(query_param("hello", "world"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
}
