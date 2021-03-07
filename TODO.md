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
- [ ] Sort the player vector for each team first for the requested player on `vec[0]`,
    then the remaining players either by name or by rating. highest rating after
    `vec[0] == requested player`

### Error Handling

- [ ] **WIP** Implement good error handling
    - [X] use crates error types for better `Error handling` e.g. `reqwest::Error`
    - [X] use `thiserror` in library part
    - [X] use `eyre` consistently for results with reports in binary part
    - [ ] use `.map_err` and return HTTP status codes
    - [ ] Handle errors that got bubbled up to the MatchInfoProcessor gracefully
        and return a maximum of valuable information on the MatchInfo and the
        errors to the client
        - [ ] Send `log entry` to Client for better error handling on client-side
        - [ ] On `hard error`, no match_info but instead error status code (HTTP)
    - [ ] handle `serde_json::Value::Null` errors better when parsing data from `aoe2.net`
- [x] implement `todo!()`s
- [ ] don't overwrite `aoc_ref_data` if not able to parse it in thread, so we have
    at least one working version
- [ ] Special cases done right? (talk through them together)
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

### Testing

- [ ] implement useful tests/raise test-coverage to a (valuable) maximum
    - [ ] put unit tests into the same file of the type they refer to
    - [ ] use `wiremock` for HTTP-mocking and test requests made by the `api_handler`
    - [ ] use `claim` for tests
- [ ] Write functionality to save a set of JSON responses (also our own) to a file
to use them inside the integration tests and be able to update frequently
    - [ ] Parse requests into `MatchDataResponses` and shove them up through the
    `MatchInfoProcessor` to test offline as well
    - [ ] Compare our parsed initial response (manually checked) with the one in
    memory from the offline data

### Refactoring

- [X] create only new clients for each new api-root not for each request to us
- [ ] Q: how can we make creating requests easier and less boilerplate? (trait
objects, etc.)
- [X] async stuff done right?
- [ ] use <https://docs.rs/reqwest/0.11.0/reqwest/struct.Url.html#method.join>
for `base_path` and joining files for DS: `reqwest::Url`
- [X] structured logging: use `tracing` crate in addition to `log` and refactor
accordingly
    - [X] use [tracing-tree](https://github.com/transparencies/tracing-tree) for
    structured summaries of tracing

### Documentation

- [X] Create good documentation

### Benchmarking

- [ ] Q: how is our backend reacting to 100+ concurrent API requests?
    - [ ] what architectural changes need to be made to support many clients
    on our api

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
