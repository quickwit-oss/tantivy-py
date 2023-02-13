ifeq ($(shell uname),Darwin)
  EXT := dylib
else
  EXT := so
endif

source_files := $(wildcard src/*.rs)

all: build

PHONY: test format

test: tantivy/tantivy.$(EXT)
	python3 -m pytest

format:
	cargo fmt

build:
	maturin build --interpreter python3.7 python3.8 python3.9 python3.10 python3.11

tantivy/tantivy.$(EXT): target/debug/libtantivy.$(EXT)
	cp target/debug/libtantivy.$(EXT) tantivy/tantivy.so

target/debug/libtantivy.$(EXT): $(source_files)
	cargo build
