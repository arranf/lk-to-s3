set -e

cross build --target arm-unknown-linux-gnueabihf --release
mkdir -p ./build
cp ./target/arm-unknown-linux-gnueabihf/release/lk-to-s3 ./build/
docker buildx build --platform linux/arm,linux/arm64 -t arranf/lk-to-s3:0.1.3 . --push
rm -rf ./build