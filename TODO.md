# TODOS

## Release 1.0.0 - Basic `match` endpoint, error handling, testing, logging and documentation

### Additions
- [X] Call from `handler` into `data_processing` with `MatchInfoRequest` data
- [ ] GET API data from aoe2net
    - [ ] make all requests that are needed for getting all valuable information for matchinfo
    - [ ] Add a language parameter to call from the frontend to use translations
        - [ ] GET and CACHE commonly used translations (ENG, ESP, GER, ITA, FRA, POR) at system startup and
              let them be updated every now and then (maybe even same thread as Github files)
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
    - [ ] Q: What's the best way in Rust to automatically map Datastructures
- [ ] Overall Procedure:
    - [ ] Client: SUBSCRIBE lastmatch(player_id=197751)
    - [ ] Server: CREATE Observable
    - [ ] Server: SEND initial state to CLIENT
    - [ ] Server: POLL AoE2.net / Caching
    - [ ] Server: UPDATE Observable
    - [ ] Server: ON UPDATE of Observable send PATCH with DELTA to CLIENT
    - [ ] Client: UPDATE internal data structure with PATCH

### Refactoring
- [X] create only new clients for each new api-root not for each request to us
- [ ] Q: how can we make creating requests easier and less boilerplate? (trait objects, etc.)
- [ ] what (other) architectural changes need to be made to support many clients on our api(?)
- [ ] async stuff done right?
- [ ] use <https://docs.rs/reqwest/0.11.0/reqwest/struct.Url.html#method.join> for `base_path` and joining files for DS: `reqwest::Url`
- [ ] structured logging: use `tracing` crate in addition to `log` and refactor accordingly
      __Note:__ already partly done

### Error Handling
- [ ] Implement good error handling
    - [ ] use crates error types for better `Error handling` e.g. `reqwest::Error`
    - [ ] use `claim` for better error reports
    - [ ] use `thiserror` in library part
    - [ ] use `eyre` consistently for results with reports in binary part (?)
    - [ ] use `.map_err` and return HTTP status codes
- [ ] implement `todo!()`s
- [ ] Send `log entry` to Client for better error handling on client-side
- [ ] Special cases done right?
    - [ ] Data structure does not match with data from aoe2net
        - [ ] Q: take a look for a `serde` attribute to mark fields in structs that are not as important for our processing,
              so we don't throw a parsing error if non-essential fields don't match/exist
    - [ ] New players without ranking
    - [ ] Deranked players
    - [ ] Coop games
    - [ ] Game Type except RM (0) and DM (2)
    - [ ] On error, no match_info & error status code

### Testing
- [ ] implement useful tests/raise test-coverage to a (valuable) maximum
    - [ ] put unit tests into the same file of the type they refer to
    - [ ] use `wiremock` for HTTP-mocking and test requests made by the `api_handler`

### Documentation
- [ ] create good documentation (!!!)

## Release 1.1.0 - User management and persistence

- [ ] User database (PostgreSQL/MariaDB), user log-in and dashboard
- [ ] Transparency management in user dashboard

### Performance optimisation
- [ ] 
