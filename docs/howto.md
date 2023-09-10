# How-to Guides

## Installation

tantivy-py can be installed using from [pypi](pypi.org) using pip:

    pip install tantivy

If no binary wheel is present for your operating system the bindings will be
build from source, this means that Rust needs to be installed before building
can succeed.

Note that the bindings are using [PyO3](https://github.com/PyO3/pyo3), which
only supports python3.

## Set up a development environment to work on tantivy-py itself

Setting up a development environment can be done in a virtual environment using
[`nox`](https://nox.thea.codes) or using local packages using the provided `Makefile`.

For the `nox` setup install the virtual environment and build the bindings using:

    python3 -m pip install nox
    nox

For the `Makefile` based setup run:

    make

Running the tests is done using:

    make test

The `nox` test session will pass pytest arguments through. For example,
to run only the tests including "simple_search" in the test name, and only
on Python 3.11:

    nox -s test-3.11 -- -k simple_search

## Working on tantivy-py documentation

Please be aware that this documentation is structured using the [Diátaxis](https://diataxis.fr/) framework. In very simple terms, this framework will suggest the correct location for different kinds of documentation. Please make sure you gain a basic understanding of the goals of the framework before making large pull requests with new documentation.

This documentation uses the [MkDocs](https://mkdocs.readthedocs.io/en/stable/) framework. This package is specified as an optional dependency in the `pyproject.toml` file. To install all optional dev dependencies into your virtual env, run the following command:

    pip install .[dev]

The [MkDocs](https://mkdocs.readthedocs.io/en/stable/) documentation itself is comprehensive. MkDocs provides some additional context and help around [writing with markdown](https://mkdocs.readthedocs.io/en/stable/user-guide/writing-your-docs/#writing-with-markdown).

If all you want to do is make a few edits right away, the documentation content is in the `/docs` directory and consists of [Markdown](https://www.markdownguide.org/) files, which can be edited with any text editor.

The most efficient way to work is to run a MkDocs livereload server in the background. This will launch a local web server on your dev machine, serve the docs (by default at `http://localhost:8000`), and automatically reload the page after you save any changes to the documentation files.
