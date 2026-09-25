import pytest

from tantivy import Document, PreTokenizedString, Query, Token
from tantivy.tantivy import TextAnalyzerBuilder, Tokenizer


@pytest.mark.parametrize(
    ("text", "tokens"),
    [
        ("Emoji Dick", (Token("Emoji", 0, 0), Token("Dick", 1, 6))),
        (
            "\u260e\ufe0f \U0001f9d4\U0001f3fb \u26f5 \U0001f433 \U0001f44c\U0001f3fb",
            (
                Token("\u260e\ufe0f", 0, 0),
                Token("\U0001f9d4\U0001f3fb", 1, 3),
                Token("\u26f5", 2, 6),
                Token("\U0001f433", 3, 8),
                Token("\U0001f44c\U0001f3fb", 4, 10),
            ),
        ),
    ],
)
def test_to_pre_tokenized(text: str, tokens: list[Token]):
    analyzer = TextAnalyzerBuilder(Tokenizer.whitespace()).build()
    pre_tokenized_string = analyzer.pre_tokenize(text)
    round_trip_tokens = tuple(pre_tokenized_string.tokens)
    assert round_trip_tokens == tokens


@pytest.mark.parametrize(
    ("text", "tokens"),
    [
        ("Emoji Dick", (Token("Emoji", 0, 0), Token("Dick", 1, 6))),
        (
            "\u260e\ufe0f\U0001f9d4\U0001f3fb\u26f5\U0001f433\U0001f44c\U0001f3fb",
            (
                Token("\u260e\ufe0f", 0, 0),
                Token("\U0001f9d4\U0001f3fb", 1, 2),
                Token("\u26f5", 2, 4),
                Token("\U0001f433", 3, 5),
                Token("\U0001f44c\U0001f3fb", 4, 6),
            ),
        ),
    ],
)
def test_round_trip_pre_tokenized(text: str, tokens: list[Token]):
    pre_tokenized_string = PreTokenizedString(text, tokens)
    round_trip_tokens = tuple(pre_tokenized_string.tokens)
    assert round_trip_tokens == tokens


def test_write_pre_tokenized(ram_index):
    title = "Emoji Dick"
    tokens = (Token("Emoji", 0, 0), Token("Dick", 1, 6))

    pre_tokenized_title = PreTokenizedString(title, tokens)

    body = "\u260e\ufe0f\U0001f9d4\U0001f3fb\u26f5\U0001f433\U0001f44c\U0001f3fb"
    tokens = (
        Token("\u260e\ufe0f", 0, 0),
        Token("\U0001f9d4\U0001f3fb", 1, 2),
        Token("\u26f5", 2, 4),
        Token("\U0001f433", 3, 5),
        Token("\U0001f44c\U0001f3fb", 4, 6),
    )
    pre_tokenized_body = PreTokenizedString(body, tokens)

    doc = Document()
    doc.add_pre_tokenized_text("title", pre_tokenized_title)
    doc.add_pre_tokenized_text("body", pre_tokenized_body)

    assert doc["title"] == [pre_tokenized_title]

    with ram_index.writer() as writer:
        writer.add_document(doc)

    ram_index.reload()
    schema = ram_index.schema
    searcher = ram_index.searcher()

    query = Query.term_query(schema, "body", "\U0001f9d4\U0001f3fb")
    r = searcher.search(query)
    assert len(r.hits) == 1

    query = Query.phrase_query(schema, "body", ["\U0001f9d4\U0001f3fb", "\u26f5"])
    r = searcher.search(query)
    assert len(r.hits) == 1
