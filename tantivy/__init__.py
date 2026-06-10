from .tantivy import *
from . import tantivy as _tantivy

# Mark the public API explicitly. The wildcard import above already pulls in
# every public name from the compiled extension, but defining __all__ tells
# type checkers and documentation tooling what the package exports. In
# particular, it lets pdoc render the entire API on the top-level `tantivy`
# page rather than scattering it onto the internal `tantivy.tantivy` submodule.
__all__ = [name for name in dir(_tantivy) if not name.startswith("_")]