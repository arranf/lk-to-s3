FROM arm32v7/debian:stretch-slim
RUN apt-get update && apt-get install -y ca-certificates
ADD ./build/lk-to-s3 .
CMD ["/lk-to-s3"]