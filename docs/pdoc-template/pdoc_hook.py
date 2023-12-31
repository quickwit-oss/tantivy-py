def on_pre_build(config):
    # From here: https://github.com/mitmproxy/pdoc/blob/main/examples/mkdocs/make.py
    from pathlib import Path
    import shutil

    from pdoc import pdoc
    from pdoc import render

    here = Path(__file__).parent.parent
    out = here / "api"
    if out.exists():
        shutil.rmtree(out)

    # Render parts of pdoc's documentation into docs/api...
    render.configure(template_directory=here / "pdoc-template")
    pdoc("tantivy", output_directory=out)

    # ...and rename the .html files to .md so that mkdocs picks them up!
    for f in out.glob("**/*.html"):
        f.rename(f.with_suffix(".md"))
