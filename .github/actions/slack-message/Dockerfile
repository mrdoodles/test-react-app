# build stage
FROM rust:slim as build

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
# RUN cargo build --release; \
#     rm src/*.rs

# copy project source and necessary files
COPY ./src ./src

RUN cargo build --release
#  \
#   mv ./target/release/slack-message /usr/local/bin/

# # rebuild app with project source
# RUN rm ./target/release/slack-message*; \
#       rm ./target/release/deps/slack_message*; \
#       cargo build --release

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq and libsqlite
RUN apt-get update; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/slack-message /usr/local/bin/

# run the binary
CMD ["/usr/local/bin/slack-message"]

# USER root
# CMD ["tail", "-f", "/dev/null"]