import nox


@nox.session(python=["3.8", "3.9", "3.10", "3.11", "3.12"])
def test(session):
    session.install("-rrequirements-dev.txt")
    session.install("-e", ".", "--no-build-isolation")
    session.run("pytest", *session.posargs)


@nox.session(python=["3.8", "3.9", "3.10", "3.11", "3.12"])
def test_lindera(session):
    session.install("-rrequirements-dev.txt")
    session.install(
        "--no-build-isolation",
        '--config-settings',
        'build-args="--features=lindera"',
        "-e",
        ".",
    )
    session.run("pytest", *session.posargs)
