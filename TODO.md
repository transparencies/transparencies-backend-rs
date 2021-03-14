# TODOS

## Release 1.0.0 - Basic `match` endpoint, error handling, testing, logging and documentation

### Additions

- [X] Call from `handler` into `data_processing` with `MatchInfoRequest` data
- [X] GET API data from aoe2net
    - [X] make all requests that are needed for getting all valuable information
        for matchinfo
    - [X] Add a language parameter to call from the frontend to use translations
- [X] GET and CACHE (in-memory DB, `Arc<Mutex<T>>`) commonly used translations
    (ENG, ESP, GER, ITA, FRA, POR) at system startup and let them be updated every
    now and then
    - spawn another thread for this task and don't use the github one (client
    encapsulation, easier debugging)
    - don't use static types for this, to be less error prone if AoE2net updates
    something at these endpoints, we don't want to have
    errors every ten minutes if something breaks
- [x] GET json/yaml file(s) from github (periodically?) [teams, platforms, players]
    - Sources:
        - [X] <https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml>
        - [X] <https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/platforms.json>
        - [X] <https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/teams.json>
    - [x] periodically:
        - [x] at the start of the server
        - [x] once every 10 minutes
- [X] Merge various data sources into a `MatchInfo` datastructure for giving back
    to client
    - [x] await json from polska for new matchinfo DS for merging/exposing to our
    frontend
    - [x] Q: What's the best way in Rust to automatically map Datastructures
- [X] Sort the player vector for each team first for the requested player
    on `vec[0]`
    - [ ] then the remaining players either by name or by rating. highest
    rating after `vec[0] == requested player`

### Error Handling

- [ ] **WIP** Implement good error handling
    - [X] use crates error types for better `Error handling` e.g. `reqwest::Error`
    - [X] use `thiserror` in library part
    - [X] use `eyre` consistently for results with reports in binary part
    - [X] use `.map_err` and return HTTP status codes
    - [X] Handle errors that got bubbled up to the MatchInfoProcessor gracefully
        and return a maximum of valuable information on the MatchInfo and the
        errors to the client
        - [X] Send `log entry` to Client for better error handling on client-side
        - [X] On `hard error`, no match_info but instead error status code (HTTP)
    - [X] handle `serde_json::Value::Null` errors better when parsing data from `aoe2.net`
- [X] implement `todo!()`s
- [ ] don't overwrite `aoc_ref_data` if not able to parse it in thread, so we have
    at least one working version

### Testing

- [ ] implement functionality to download a specific match via `/api/match` for
usage in test cases
    - [ ] create a command-line parameter for `export-test-data` example to run
    it with a specific `matchid`/`match-uuid` (UUID is probably preferable because
    of validation with `uuid` crate)
    - [ ] then run all other functions to download the resources need to answer
    with that specific request
    - [ ] rebuild and export the MatchInfoRequest for a random player from it
    for test case usage of our normal functionality
- [ ] implement useful tests/raise test-coverage to a (valuable) maximum
    - [ ] put unit tests into the same file of the type they refer to
    - [ ] use `claim` for tests
- [ ] Special cases done right? (talk through them together)
    - [ ] Implement (integration) test cases for these
    - [X] Data structure does not match with data from aoe2net
        - [X] Q: take a look for a `serde` attribute to mark fields in structs that
            are not as important for our processing, so we don't throw a parsing
            error if non-essential fields don't match/exist
        - **A:** We only parse `Players` of `last_match` into some losely-typed
        datastructure for easier handling, the rest is `serde_json::Value` and
        parsing on the run
    - [ ] New players without ranking
    - [ ] Deranked players
    - [ ] Coop games
    - [ ] Game Type except RM (0) and DM (2)
    - [ ] FFA with teams set to ’-1’
- [X] Write functionality to save a set of JSON responses (also our own) to a file
to use them inside the integration tests and be able to update frequently
    - [X] Parse requests and use `wiremock` for HTTP-mocking and test requests
    made by the `api_handler`
    - [X] Compare our parsed initial response (manually checked) with the one in
    memory from the offline data

### Fixes

- [ ] Fix character escaping in e.g. `"name": "\"[RUS-F]GriN\""`
- [ ] Make error message more  understandable for frontend:

    ```sh
    "GenericResponderError":
    "Other ApiRequestError: HTTP-Client experienced an error: error decoding response
    body: EOF while parsing a value at line 1 column 0."
    ```

    probably from

    ```sh
    http: error: ConnectionError: ('Connection aborted.', RemoteDisconnected('
    Remote end closed connection without response')) while doing a GET request 
    to URL: http://127.0.0.1:8000/matchinfo?id_type=profile_id&id_number=224786&language=en&game=aoe2de
    ```

- [ ] Investigate HTTPie errors for more edge cases
- [X] in case `team == -1` start setting from Team 1/2 not from the back (7/8)

### Refactoring

- [X] Parse `MatchInfoRequest` for `export-sample-data` and `full-integration` test
from `ron` file for ease of testing/exporting
    - [X] create struct that contains a `MatchInfoRequest` and a folder-layout and
    for other useful/needed information so we can create different test cases easier
    for `integration` testing
- [X] Refactor both, parsing and mock binding logic in full integration test
- [X] create only new clients for each new api-root not for each request to us
- [ ] **Q:** how can we make creating requests easier and less boilerplate? (trait
objects, etc.)
    - [ ] Create API client struct that wraps `ApiRequests` and `ApiResponses`
    - [ ] Also think about the openAPI parsing and request generating logics
    for the future
    - [ ] `parse_into::<T>` method for `ApiRequest` and `FileRequest`
    - [ ] `ParsableRequest` trait
- [X] async stuff done right?
- [X] use <https://docs.rs/reqwest/0.11.0/reqwest/struct.Url.html#method.join>
for `base_path` and joining files for DS: `reqwest::Url`
- [X] structured logging: use `tracing` crate instead of `log` and refactor
accordingly
    - [X] use [tracing-tree](https://github.com/transparencies/tracing-tree) for
    structured summaries of tracing
- [X] Use a concurrent hashmap instead of a HashMap: <https://crates.io/crates/dashmap>
- [ ] Check value of <https://crates.io/crates/indexmap> for the player alias indexing

### Documentation

- [X] Create good base documentation
- [ ] Add more documentation

### Performance

[Rust Performance Book](https://nnethercote.github.io/perf-book/)

#### Benchmarking

- [ ] Q: how is our backend reacting to 100+ concurrent API requests?
    - [ ] implement benchmark getting all ’MatchInfoResult’s for the To100
    - [ ] what architectural changes need to be made to support many clients
    on our api
- [ ] Use [bencher](https://crates.io/crates/bencher) for benchmarking features
on stable
- [ ] Maybe [criterion](https://github.com/bheisler/criterion.rs) which is a more
sophisticated alternative
- [ ] Can dig deeper after profiling with [counts](https://github.com/nnethercote/counts/)

#### Flamegraph

- [ ] Use [flamegraph](https://github.com/flamegraph-rs/flamegraph). A very simple
and portable tool to understand where the time is spent in the application. For
more detailed info, try: `perf record -g --call-graph=dwarf /path/to/your/application`,
then [load it into Firefox Profiler](https://profiler.firefox.com/docs/#/./guide-perf-profiling).

## Release 1.1.0 - SUBSCRIPTION requests

### Additions

- [ ] GET `last_match` for `aoc-reference-data` profile ids in another `thread`
and save the content to a `HashMap` -> for later subscriptions
    - [ ] if a `profile id` from this list asks for a `MatchInfo` we can shorten
    the path/do less requests
    - [ ] new subscriptions that are not on `aoc-reference-data` can be made persistent
    within a `ron` file that gets parsed on startup
    - [ ] `active subscribed` profile ids get copied from this `HashMap` into an
    `ActiveSubs`-`HashMap` where requests are made more frequently to check for
    changes and send out a `delta`

### Intended Procedure

- [ ] Client: SUBSCRIBE lastmatch(player_id=197751)
- [ ] Server: CREATE Observable
- [ ] Server: SEND initial state to CLIENT
- [ ] Server: POLL AoE2.net / Caching
- [ ] Server: UPDATE Observable
- [ ] Server: ON UPDATE of Observable send PATCH with DELTA to CLIENT
- [ ] Client: UPDATE internal data structure with PATCH

### Performance optimisation

## Release 1.2.0 - User management and persistence

- [ ] User database (PostgreSQL/MariaDB), user log-in and dashboard
- [ ] Transparency management in user dashboard
