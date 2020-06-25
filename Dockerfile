FROM arm32v7/debian:stretch-slim
COPY . /target/arm-unknown-linux-gnueabihf/release/lk-to-s3/
CMD ["/lk-to-s3"]