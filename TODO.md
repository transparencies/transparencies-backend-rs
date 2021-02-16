# TODOS

## Release 1.0.0 - Basic `match` endpoint, error handling, testing, logging and documentation

### Additions
- [X] Call from `handler` into `data_processing` with `MatchInfoRequest` data
- [ ] GET API data from aoe2net
    - [ ] make all requests that are needed for getting all valuable information for matchinfo
    - [ ] Add a language parameter to call from the frontend to use translations
    - [ ] Should we add a `last_game_id` parameter we can check against at `last_match` to answer
          faster if nothing is new? (client -> sends us `last_game_id` the overlay shows -> we check against `last_match`
        - [ ] if `last_match` show finished match we wait 3-5 minutes until frontend fades out
        - [ ] Q: how to get away from client-side polling? (frequent requests to `last_match` and WS to client? worth?)
            - [ ] maybe let clients subscribe to us and frequently poll aoe2.net for these subscribed client ids? (high load on AoE2.net)
                - only query `last-match` endpoint and if something changed, then fire other requests will reduce load
                - maybe frontend should send us `match_id` that it is currently showing so we always have its `last_match` 
                  and can easily compare with `last_match` from aoe2.net
    - [ ] Q: can we cache anything?
    - [ ] Q: how can we make creating requests easier and less boilerplate?
- [ ] GET json/yaml file(s) from github (periodically?) [teams, platforms, players]
    - Sources:
        - [X] https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/players.yaml
        - [X] https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/platforms.json
        - [X] https://raw.githubusercontent.com/SiegeEngineers/aoc-reference-data/master/data/teams.json
    - [ ] periodically:
        - [ ] at the start of the server
        - [ ] once per hour
        - [ ] non-persistent, but only overwrite internally if parsing new datastructure was successful
            - [ ] create in-memory DB (`Arc<Mutex<RefData>>` or some other in-memory thread-safe storage)
- [ ] Merge various data sources into a `MatchInfo` datastructure for giving back to client
    - [ ] await json from polska for new matchinfo DS for merging/exposing to our frontend
### Refactoring
- [ ] create only new clients for each new api-root not for each request to us
    - [ ] only if that would make overall performance better (<1.2 sec per request)(?)
- [ ] what (other) architectural changes need to be made to support many clients on our api(?)
- [ ] async stuff done right?
- [ ] use <https://docs.rs/reqwest/0.11.0/reqwest/struct.Url.html#method.join> for `base_path` and joining files for DS: `reqwest::Url`
- [ ] structured logging: use `tracing` crate in addition to `log` and refactor accordingly

### Error Handling
- [ ] Implement good error handling
    - [ ] use crates error types for better `Error handling` e.g. `reqwest::Error`
    - [ ] use `claim` for better error reports
    - [ ] use `thiserror` in library part
    - [ ] use `eyre` consistently for results with reports in binary part (?)
    - [ ] use `.map_err` and return HTTP status codes
- [ ] implement `todo!()`s

### Testing
- [ ] implement useful tests/raise test-coverage to a (valuable) maximum
    - [ ] put unit tests into the same file of the type they refer to
    - [ ] use `wiremock` for HTTP-mocking and test requests made by the `api_handler`

### Documentation
- [ ] create good documentation (!!!)

## Release 1.1.0 - User management and persistence

- [ ] User database (PostgreSQL/MariaDB), user log-in and dashboard
- [ ] Transparency management in user dashboard
