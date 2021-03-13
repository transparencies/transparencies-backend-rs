# transparencies-backend-rs [![AGPLv3+](https://www.gnu.org/graphics/agplv3-88x31.png)](https://www.gnu.org/licenses/agpl.txt)

## A backend for dynamic stream overlays written in Rust

## Benchmarking

## Profiling

You can use `flamegraph` for benchmarking with `cargo flamegraph --bin transparencies-backend-rs`.
The `flamegraph.svg` will be found in the root of the repository.

## Testing

## Creating a new test case

- First create a copy of the `/tests/matchinfo-integration/test_case_template` folder.
Rename it to something reasonable.

- Then edit the `match_info_request.ron` file in this newly created folder structure
and let it point to a combination of settings you want to test.

- Then run `cargo run --example export-test-data -- --test-case-folder <test-case-export-path>`
to download a set of responses that are serialized into JSON-files into the corresponding
folder.

- Afterwards open `/tests/matchinfo-integration/main.rs` and depending on what you
want to achieve either copy one of the functions marked with `#[tokio::test]` and
adapt them or add another `TestCase` to one of these functions. `TestCases` that
are added with `test_cases.add(path)` should be only added if they test the same
functionality/feature. Otherwhise create a new function from. This makes it easier
to see if a test fails which feature is not working.

### Update data for a case of an integration test

- Make sure the data that will be exported again will depict the actual test case.
Because everything is redownloaded it's not that sure.

- Then run `cargo run --example export-test-data -- --test-case-folder <test-case-export-path>`
to download a set of responses that are serialized into JSON-files into the corresponding
folder.

Then run `cargo test`/`just test` to run our test suite.

## Documentation

Documentation can be easily build and opened with `cargo doc --no-deps
--document-private-items --open`.

## License

**GNU AGPLv3** or later; see [LICENSE](LICENSE).
