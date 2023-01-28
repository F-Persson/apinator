###########
### Use this when I have a running app that can be deployed to image
# FROM rust:1.67 as builder
# WORKDIR /apinator # make sure to be in this directory kind of
# COPY . .
# RUN cargo install --path .

# FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
# CMD ["myapp"]

#############
# Build an image that has Rust
FROM rust:1.67 as build

# create a new empty shell project
RUN USER=root cargo new --bin apinator
WORKDIR /apinator

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/apinator*
RUN cargo build --release

# our final base
FROM rust:1.67-slim-buster

# copy the build artifact from the build stage
COPY --from=build /apinator/target/release/apinator .

# set the startup command to run your binary
CMD ["./apinator"]

########################
# Old way - More than 2gb
# This 
# # 1. use this Rust version
# FROM rust:1.67

# # 2. Copy files from this folder to the ./ of the docker image
# COPY ./ ./

# # 3. Build my program for relase or Build
# RUN cargo build --release

# # 4. Run the library
# CMD ["./target/release/apinator"]