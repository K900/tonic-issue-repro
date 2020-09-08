FROM rust:alpine AS build

COPY . /src
WORKDIR /src

RUN apk add musl-dev protoc
ENV PROTOC=/usr/bin/protoc
ENV PROTOC_INCLUDE=/usr/include
RUN cargo build --release

FROM curlimages/curl
COPY --from=build /src/target/release/server /server
COPY --from=build /src/target/release/proxy /proxy
COPY do_test.sh /do_test.sh
ENTRYPOINT "/do_test.sh"

