from pathlib import Path
import pytest

from mktestdocs import check_md_file


@pytest.mark.parametrize("filepath", Path("docs").glob("**/*.md"), ids=str)
def test_docs(filepath):
    check_md_file(filepath, memory=True)
