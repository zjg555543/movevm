.PHONY: all build build-rust build-go test

# Builds the Rust library libmovevm
BUILDERS_PREFIX := cosmwasm/go-ext-builder:0012
# Contains a full Go dev environment in order to run Go tests on the built library
ALPINE_TESTER := cosmwasm/go-ext-builder:0012-alpine

USER_ID := $(shell id -u)
USER_GROUP = $(shell id -g)

SHARED_LIB_SRC = "" # File name of the shared library as created by the Rust build system
SHARED_LIB_DST = "" # File name of the shared library that we store
ifeq ($(OS),Windows_NT)
	SHARED_LIB_SRC = wasmvm.dll
	SHARED_LIB_DST = wasmvm.dll
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		SHARED_LIB_SRC = libmovevm.so
		SHARED_LIB_DST = libmovevm.$(shell rustc --print cfg | grep target_arch | cut  -d '"' -f 2).so
	endif
	ifeq ($(UNAME_S),Darwin)
		SHARED_LIB_SRC = libmovevm.dylib
		SHARED_LIB_DST = libmovevm.dylib
	endif
endif

test-filenames:
	echo $(SHARED_LIB_DST)
	echo $(SHARED_LIB_SRC)

all: build

build: build-rust-debug build-go

build-rust: build-rust-release

# Use debug build for quick testing.
# In order to use "--features backtraces" here we need a Rust nightly toolchain, which we don't have by default
build-rust-debug:
	(cd libmovevm && cargo build)
	cp libmovevm/target/debug/$(SHARED_LIB_SRC) api/$(SHARED_LIB_DST)
	make update-bindings

# use release build to actually ship - smaller and much faster
#
# See https://github.com/CosmWasm/wasmvm/issues/222#issuecomment-880616953 for two approaches to
# enable stripping through cargo (if that is desired).
build-rust-release:
	(cd libmovevm && cargo build --release)
	cp libmovevm/target/release/$(SHARED_LIB_SRC) api/$(SHARED_LIB_DST)
	make update-bindings
	@ #this pulls out ELF symbols, 80% size reduction!

build-go:
	go build ./...

test:
	# Use package list mode to include all subdirectores. The -count=1 turns off caching.
	RUST_BACKTRACE=1 go test -v -count=1 ./...

test-safety:
	# Use package list mode to include all subdirectores. The -count=1 turns off caching.
	GODEBUG=cgocheck=2 go test -race -v -count=1 ./...

# Creates a release build in a containerized build environment of the shared library for glibc Linux (.so)
release-build-linux:
	rm -rf libmovevm/target/release
	docker run --rm -u $(USER_ID):$(USER_GROUP) -v $(shell pwd)/libmovevm:/code $(BUILDERS_PREFIX)-centos7
	cp libmovevm/artifacts/libmovevm.x86_64.so api
	cp libmovevm/artifacts/libmovevm.aarch64.so api
	make update-bindings

# Creates a release build in a containerized build environment of the shared library for macOS (.dylib)
release-build-macos:
	rm -rf libmovevm/target/x86_64-apple-darwin/release
	rm -rf libmovevm/target/aarch64-apple-darwin/release
	docker run --rm -u $(USER_ID):$(USER_GROUP) -v $(shell pwd)/libmovevm:/code $(BUILDERS_PREFIX)-cross build_macos.sh
	cp libmovevm/artifacts/libmovevm.dylib api
	make update-bindings

update-bindings:
	# After we build libmovevm, we have to copy the generated bindings for Go code to use.
	cp libmovevm/bindings.h api

release-build:
	make release-build-macos
