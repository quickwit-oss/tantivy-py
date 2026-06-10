"""Generate the API reference pages under docs/api/ using pdoc.

pdoc introspects the *installed* tantivy module, so the compiled extension must
be importable when this runs (e.g. `maturin develop` first). Because of that,
API docs are generated locally as a dev action and the resulting markdown is
committed to the repository. The Read the Docs build then only has to serve
plain markdown and never needs to compile the Rust extension.

Run via `just docs-api`, or directly with `python docs/gen_api.py`.

pdoc emits HTML; we render it through a stripped-down template (no page layout
CSS, so it nests inside the mkdocs theme) and rename the .html files to .md so
that mkdocs ingests them as raw-HTML markdown. For the relative links pdoc
writes (e.g. `tantivy.html`) to resolve, mkdocs must be configured with
`use_directory_urls: false`.

The modern pdoc only emits HTML (Markdown output was removed in its rewrite),
and mkdocs builds the navigation sub-tree shown in the theme sidebar from a
page's Markdown headings -- which a single HTML blob does not have. So after
generating, we rewrite each page so every top-level class/function sits in its
own HTML block with a Markdown `## <Name>` heading in front of it. mkdocs then
lists those members as a collapsible sub-tree under "API", while pdoc's HTML
continues to render the bodies.

The headings must sit at the document's top level to be parsed as Markdown:
Python-Markdown treats a balanced block element (pdoc wraps the whole module in
one `<main class="pdoc">`) as opaque raw HTML, so a heading inside it would be
emitted verbatim. We therefore split that single wrapper into one wrapper per
member with the headings in between.

The page also needs a single Markdown `# <module>` heading: the readthedocs
theme renders the sidebar sub-tree from the *children* of a page's first
heading (it treats that first heading as the page title). Without a Markdown
h1 the member headings would be top-level siblings with no parent to nest
under, and the theme would show nothing. pdoc's own raw-HTML module title is
removed so the page has exactly one title.
"""

import re
import shutil
from pathlib import Path

from pdoc import pdoc, render

HERE = Path(__file__).parent
TEMPLATE_DIR = HERE / "pdoc-template"
OUT_DIR = HERE / "api"

# pdoc renders each top-level member and the module preamble as a `<section>`;
# methods/attributes are plain `<div>`s, so sections never nest and a
# non-greedy match is safe. A member section's id is its name; the module
# preamble uses `class="module-info"` instead and so gets no heading.
_SECTION = re.compile(r"<section\b[^>]*>.*?</section>", re.DOTALL)
_MEMBER_ID = re.compile(r'<section id="([^".]+)">')
_FRONTMATTER = re.compile(r"\A---\n.*?\n---\n", re.DOTALL)
_TITLE = re.compile(r"^title:\s*(.+)$", re.MULTILINE)
_MODULE_H1 = re.compile(r'<h1 class="modulename">.*?</h1>', re.DOTALL)
_STYLE = re.compile(r"<style>.*?</style>", re.DOTALL)


def _add_nav_headings(md_file: Path) -> None:
    """Give the page a Markdown h1 and each member its own h2 + HTML block.

    The h2s become mkdocs table-of-contents entries that the theme renders as a
    sidebar sub-tree under the h1 page title. pdoc's own `id="Name"` section
    anchors are preserved, so existing cross-links keep working. Pages without
    member sections (e.g. the redirect index) are left untouched.
    """
    text = md_file.read_text()
    sections = _SECTION.findall(text)
    if not sections:
        return

    frontmatter = (m.group(0) if (m := _FRONTMATTER.match(text)) else "")
    title = (m.group(1).strip() if (m := _TITLE.search(frontmatter)) else "API")
    styles = "".join(_STYLE.findall(text))

    parts = [frontmatter.rstrip("\n"), "", f"# {title}", ""]
    for section in sections:
        section = _MODULE_H1.sub("", section)  # drop pdoc's duplicate page title
        if name := _MEMBER_ID.match(section):
            parts += [f"## {name.group(1)}", ""]
        # Keep the pdoc CSS scope (`.pdoc`) on every block, with no blank lines
        # inside so Python-Markdown captures each as a single raw HTML block.
        parts += ['<main class="pdoc">', section, "</main>", ""]
    parts += [styles, ""]

    md_file.write_text("\n".join(parts))


def main() -> None:
    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)

    render.configure(template_directory=TEMPLATE_DIR)
    pdoc("tantivy", output_directory=OUT_DIR)

    # Rename pdoc's .html output to .md so mkdocs picks it up, then add the
    # navigation headings. The index page is a bare redirect with no members.
    for f in OUT_DIR.glob("**/*.html"):
        md = f.with_suffix(".md")
        f.rename(md)
        if md.name != "index.md":
            _add_nav_headings(md)


if __name__ == "__main__":
    main()
