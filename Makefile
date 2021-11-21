ifeq ($(shell UNAME),Darwin)
  EXT := dylib
else
  EXT := so
endif

source_files := $(wildcard src/*.rs)

all: tantivy/tantivy.$(EXT)

PHONY: test format

test: tantivy/tantivy.$(EXT)
	python3 -m pytest

format:
	rustfmt src/*.rs

tantivy/tantivy.$(EXT): target/debug/libtantivy.$(EXT)
	cp target/debug/libtantivy.$(EXT) tantivy/tantivy.so

target/debug/libtantivy.$(EXT): $(source_files)
	cargo build
