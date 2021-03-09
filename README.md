# transparencies-backend-rs [![AGPLv3+](https://www.gnu.org/graphics/agplv3-88x31.png)](https://www.gnu.org/licenses/agpl.txt)

## A backend for dynamic stream overlays written in Rust

## Benchmarking

## Profiling

You can use `flamegraph` for benchmarking with `cargo flamegraph --bin transparencies-backend-rs`.
The `flamegraph.svg` will be found in the root of the repository.

## Testing

### Update data for integration tests

First run `cargo run --example export-test-data` to download a set of responses that
are serialized into JSON-files in `tests/integration/resources`. They are used
for running `cargo test` to check the processing pipeline.

Then run `cargo test` to run our test suite.

## Documentation

Documentation can be easily build and opened with `cargo doc --no-deps
--document-private-items --open`.

## License

**GNU AGPLv3** or later; see [LICENSE](LICENSE).
