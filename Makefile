ifeq ($(shell uname),Darwin)
  EXT := dylib
else
  EXT := so
endif

source_files := $(wildcard src/*.rs)

all: format lint build test

PHONY: test format

lint:
	cargo clippy

test: tantivy/tantivy.$(EXT)
	python3 -m pytest

format:
	cargo fmt

tantivy/tantivy.$(EXT): target/debug/libtantivy.$(EXT)
	cp target/debug/libtantivy.$(EXT) tantivy/tantivy.so

target/debug/libtantivy.$(EXT): $(source_files)
	cargo build