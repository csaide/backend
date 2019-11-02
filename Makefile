# Copyright (c) 2019 Christian Saide <supernomad>
# Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

.PHONY: dev setup-dev debug release docker fmt lint check clean

BUILD_TYPE := "debug"

dev: fmt check debug

setup-dev:
	@bash dist/bin/print.sh "SETTING UP LOCAL DEV"
	@rustup toolchain install nightly
	@rustup component add rustfmt --toolchain nightly

debug:
	@bash dist/bin/print.sh "BUILDING DEBUG"
	@cargo build

release:
	@bash dist/bin/print.sh "BUILDING RELEASE"
	@cargo build --release

docker-%:
	@bash dist/bin/print.sh "BUILDING DOCKER"
	@DOCKER_BUILDKIT=1 docker build \
		--build-arg BUILD=$(BUILD_TYPE) \
		--build-arg APP=$* \
		-t csaide/$*:$(BUILD_TYPE) \
		-f ./dist/docker/Dockerfile .

docker: docker-restd

fmt:
	@bash dist/bin/print.sh "FORMATTING CODE"
	@cargo +nightly fmt --all -- --emit=files

lint:
	@bash dist/bin/print.sh "LINTING"
	@cargo +nightly fmt --all -- --check

check:
	@bash dist/bin/print.sh "TESTING"
	@cargo test --all

clean:
	@bash dist/bin/print.sh "CLEANING"
	@rm -rf target/debug target/release
