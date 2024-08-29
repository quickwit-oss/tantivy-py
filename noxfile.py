import nox


<<<<<<< HEAD
@nox.session(python=["3.8", "3.9", "3.10", "3.11", "3.12"])
=======
@nox.session(python=["3.8", "3.9", "3.10", "3.11", "3.12", "3.13"])
>>>>>>> upstream/master
def test(session):
    session.install("-rrequirements-dev.txt")
    session.install("-e", ".", "--no-build-isolation")
    session.run("pytest", *session.posargs)
