#########################
## rust docker image ##
#########################
# this is a multi-stage build, it results in a clean last image with only the binary for running the api server
# to build use:
# docker build -t NAME:TAG .
# to remove intermediary containers after:
# docker rmi $(docker images -f "dangling=true" -q)
# to run
# docker run -it --net=host --env-file=.env NAME

ARG PROJECT_NAME="image-compare-api"

FROM rust:1-slim-bookworm as build
ARG PROJECT_NAME

# new empy shell project
RUN USER=root cargo new --bin ${PROJECT_NAME}
WORKDIR /${PROJECT_NAME}

# copy over the toolchain and cargo manifest
COPY ./rust-toolchain.toml ./
COPY ./Cargo.* ./

# build dependencies (for layer caching)
RUN cargo build --release
RUN rm -r src

# copy over the source tree
COPY ./src ./src

# copy over the sqlx metadata (for building without a database)
COPY ./.sqlx ./.sqlx

# copy over the migrations directory
COPY ./migrations ./migrations

# build
RUN /bin/bash -c 'rm -r target/release/deps/${PROJECT_NAME//-/_}*'
RUN cargo build --release

# final base: slim version of debian
FROM debian:bookworm-slim
ARG PROJECT_NAME

# copy binary from build container
COPY --from=build /${PROJECT_NAME}/target/release/${PROJECT_NAME} .

# copy over the migrations directory
COPY ./migrations ./migrations

# set command to run the binary
ENV PROJECT_NAME=$PROJECT_NAME
CMD ["sh", "-c", "./${PROJECT_NAME}"]
