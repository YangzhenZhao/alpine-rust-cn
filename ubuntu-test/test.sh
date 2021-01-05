#!bin/sh

cd quotes-cli
cargo clean
docker run -it -v ${PWD}:/volume/ --rm muslrust-cn cargo run --release --target x86_64-unknown-linux-musl -- -h
docker run -it -v ${PWD}/target/x86_64-unknown-linux-musl/release:/volume/ --rm alpine ./volume/quotes-cli -h
docker run -it -v ${PWD}/target/x86_64-unknown-linux-musl/release:/volume/ --rm centos ./volume/quotes-cli -h
