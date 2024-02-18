import pytest
pytestmark = pytest.mark.lindera

from tantivy import SchemaBuilder, Index, Document


@pytest.mark.parametrize("mode", [
    "normal",
    "decompose",
])
def test_basic(mode):
    # The import is here so that the non-lindera tests
    # can run without lindera installed.
    from tantivy import lindera

    if mode == "normal":
        mode = lindera.LNormal()
    else:
        mode = lindera.LDecompose()

    sb = SchemaBuilder()
    sb.add_text_field("title", stored=True, tokenizer_name="lang_ja")
    schema = sb.build()
    index = Index(schema)
    index.register_lindera_tokenizer(
        "lang_ja",
        mode,
        lindera.LinderaDictionaryKind.IPADIC,
    )
    writer = index.writer(50_000_000)
    doc = Document()
    doc.add_text("title", "成田国際空港")
    writer.add_document(doc)
    writer.commit()
    index.reload()
