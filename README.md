# transparencies-backend-rs [![AGPLv3+](https://www.gnu.org/graphics/agplv3-88x31.png)](https://www.gnu.org/licenses/agpl.txt)

## A backend for dynamic stream overlays written in Rust

## Benchmarking

## Profiling

You can use `flamegraph` for benchmarking with `cargo flamegraph --bin transparencies-backend-rs`.
The `flamegraph.svg` will be found in the root of the repository.

## Testing

## Creating a new test case

- First create a copy of the `/tests/integration/test_case_template` folder.
Rename it to something reasonable.

- Then edit the `match_info_request.ron` file in this newly created folder structure
and let it point to a combination of settings you want to test.



### Update data for a case of an integration test

First create a copy of the `/tests/integration/test_case_template` folder.
You can already rename it, if you want to. But the standard export path for
the files is the standard template folder.

Then run `cargo run --example export-test-data -- --test-case-folder <test-case-export-path>`
to download a set of responses that are serialized into JSON-files into the corresponding
folder.

Then run `cargo test`/`just test` to run our test suite.

## Documentation

Documentation can be easily build and opened with `cargo doc --no-deps
--document-private-items --open`.

## License

**GNU AGPLv3** or later; see [LICENSE](LICENSE).
