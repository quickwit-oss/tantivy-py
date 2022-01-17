import nox


@nox.session(python=["3.7", "3.8", "3.9", "3.10"])
def test(session):
    session.install("-rrequirements-dev.txt")
    session.install("-e", ".", "--no-build-isolation")
    session.run("pytest")
