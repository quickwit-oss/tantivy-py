# Development tasks for tantivy-py. Run `just` to list recipes.

# Show available recipes.
default:
    @just --list

# Regenerate the committed API reference under docs/api/ from the tantivy module.
# pdoc introspects the compiled extension, so uv builds the project first; this
# keeps the generated docs in sync with the current source. Commit the result.
docs-api:
    uv run --with-requirements requirements-dev.txt python docs/gen_api.py

# Serve the docs locally with live reload (markdown only, no extension build).
docs-serve:
    uv run --no-project --with-requirements docs/requirements.txt mkdocs serve

# Build the docs site into ./site, the same way Read the Docs does.
docs-build:
    uv run --no-project --with-requirements docs/requirements.txt mkdocs build
