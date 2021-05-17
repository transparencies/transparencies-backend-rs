# FROM rust as planner
# WORKDIR transparencies-backend-rs
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# RUN cargo install cargo-chef --version 0.1.18
# Replace with copying from filesystem
# COPY . .
# RUN git clone https://github.com/transparencies/transparencies-backend-rs.git .
# RUN cargo chef prepare  --recipe-path recipe.json

# FROM rust as cacher
# WORKDIR transparencies-backend-rs
# RUN cargo install cargo-chef --version 0.1.18
# COPY --from=planner /transparencies-backend-rs/recipe.json recipe.json
# RUN cargo chef cook --recipe-path recipe.json

FROM rust as builder
WORKDIR transparencies-backend-rs
COPY . .
# Copy over the cached dependencies
# COPY --from=cacher /transparencies-backend-rs/target target
# COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --bin transparencies-backend-rs

FROM debian as runtime
WORKDIR transparencies-backend-rs
COPY ./configuration /usr/local/bin/configuration
COPY --from=builder /transparencies-backend-rs/target/release/transparencies-backend-rs /usr/local/bin
EXPOSE 8000/tcp
ENTRYPOINT ["./usr/local/bin/transparencies-backend-rs -d "]
