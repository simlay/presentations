
build:
	cargo build --target aarch64-apple-ios-sim

bundle: build
	cp ./target/aarch64-apple-ios-sim/debug/hello-world ./RustWrapper.app/

install: bundle
	xcrun simctl install booted ./RustWrapper.app/

run: install
	xcrun simctl launch --console --terminate-running-process booted RustWrapper

watch:
	cargo watch -s 'make run' -w ./src
