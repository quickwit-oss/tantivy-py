from io import BytesIO

import copy
import datetime
import json
import pickle
import pytest

import tantivy
from conftest import schema, schema_numeric_fields
from tantivy import Document, Index, SchemaBuilder, SnippetGenerator, Query, Occur


class TestClass(object):
    def test_simple_search_in_dir(self, dir_index):
        _, index = dir_index
        query = index.parse_query("sea whale", ["title", "body"])

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1

    def test_simple_search_after_reuse(self, dir_index):
        index_dir, _ = dir_index
        index = Index(schema(), str(index_dir))
        query = index.parse_query("sea whale", ["title", "body"])

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1

    def test_simple_search_in_ram(self, ram_index):
        index = ram_index
        query = index.parse_query("sea whale", ["title", "body"])

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["The Old Man and the Sea"]

    def test_simple_search_in_spanish(self, spanish_index):
        index = spanish_index
        query = index.parse_query("vieja", ["title", "body"])

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        _, doc_address = result.hits[0]
        search_doc = index.searcher().doc(doc_address)
        assert search_doc["title"] == ["El viejo y el mar"]

    def test_and_query(self, ram_index):
        index = ram_index
        query = index.parse_query(
            "title:men AND body:summer", default_field_names=["title", "body"]
        )
        # look for an intersection of documents
        searcher = index.searcher()
        result = searcher.search(query, 10)

        # summer isn't present
        assert len(result.hits) == 0

        query = index.parse_query("title:men AND body:winter", ["title", "body"])
        result = searcher.search(query)

        assert len(result.hits) == 1

    def test_and_query_numeric_fields(self, ram_index_numeric_fields):
        index = ram_index_numeric_fields
        searcher = index.searcher()

        # 1 result
        float_query = index.parse_query("3.5", ["rating"])
        result = searcher.search(float_query)
        assert len(result.hits) == 1
        assert searcher.doc(result.hits[0][1])["rating"][0] == 3.5

        integer_query = index.parse_query("1", ["id"])
        result = searcher.search(integer_query)
        assert len(result.hits) == 1

        # 0 result
        integer_query = index.parse_query("10", ["id"])
        result = searcher.search(integer_query)
        assert len(result.hits) == 0

    def test_and_query_parser_default_fields(self, ram_index):
        query = ram_index.parse_query("winter", default_field_names=["title"])
        assert repr(query) == """Query(TermQuery(Term(field=0, type=Str, "winter")))"""

    def test_and_query_parser_default_fields_undefined(self, ram_index):
        query = ram_index.parse_query("winter")
        assert (
            repr(query)
            == """Query(BooleanQuery { subqueries: [(Should, TermQuery(Term(field=0, type=Str, "winter"))), (Should, TermQuery(Term(field=1, type=Str, "winter")))] })"""
        )

    def test_parse_query_field_boosts(self, ram_index):
        query = ram_index.parse_query("winter", field_boosts={"title": 2.3})
        assert (
            repr(query)
            == """Query(BooleanQuery { subqueries: [(Should, Boost(query=TermQuery(Term(field=0, type=Str, "winter")), boost=2.3)), (Should, TermQuery(Term(field=1, type=Str, "winter")))] })"""
        )

    def test_parse_query_fuzzy_fields(self, ram_index):
        query = ram_index.parse_query("winter", fuzzy_fields={"title": (True, 1, False)})
        assert (
            repr(query)
            == """Query(BooleanQuery { subqueries: [(Should, FuzzyTermQuery { term: Term(field=0, type=Str, "winter"), distance: 1, transposition_cost_one: false, prefix: true }), (Should, TermQuery(Term(field=1, type=Str, "winter")))] })"""
        )

    def test_query_errors(self, ram_index):
        index = ram_index
        # no "bod" field
        with pytest.raises(ValueError):
            index.parse_query("bod:men", ["title", "body"])

    def test_query_lenient(self, ram_index_numeric_fields):
        from tantivy import query_parser_error

        index = ram_index_numeric_fields

        query, errors = index.parse_query_lenient("rating:3.5")
        assert len(errors) == 0
        assert repr(query) == """Query(TermQuery(Term(field=1, type=F64, 3.5)))"""

        _, errors = index.parse_query_lenient("bod:men")
        assert len(errors) == 1
        assert isinstance(errors[0], query_parser_error.FieldDoesNotExistError)

        query, errors = index.parse_query_lenient(
            "body:'hello' AND id:<3.5 OR rating:'hi'"
        )
        assert len(errors) == 2
        assert isinstance(errors[0], query_parser_error.ExpectedIntError)
        assert isinstance(errors[1], query_parser_error.ExpectedFloatError)
        assert (
            repr(query)
            == """Query(BooleanQuery { subqueries: [(Should, BooleanQuery { subqueries: [(Must, TermQuery(Term(field=3, type=Str, "hello")))] })] })"""
        )

    def test_order_by_search(self):
        schema = (
            SchemaBuilder()
            .add_unsigned_field("order", fast=True)
            .add_text_field("title", stored=True)
            .build()
        )

        index = Index(schema)
        writer = index.writer()

        doc = Document()
        doc.add_unsigned("order", 0)
        doc.add_text("title", "Test title")

        writer.add_document(doc)

        doc = Document()
        doc.add_unsigned("order", 2)
        doc.add_text("title", "Final test title")
        writer.add_document(doc)

        doc = Document()
        doc.add_unsigned("order", 1)
        doc.add_text("title", "Another test title")

        writer.add_document(doc)

        writer.commit()
        index.reload()

        query = index.parse_query("test")

        searcher = index.searcher()

        result = searcher.search(query, 10, offset=2, order_by_field="order")

        assert len(result.hits) == 1

        result = searcher.search(query, 10, order_by_field="order")

        assert len(result.hits) == 3

        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Final test title"]

        _, doc_address = result.hits[1]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Another test title"]

        _, doc_address = result.hits[2]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Test title"]

        result = searcher.search(
            query, 10, order_by_field="order", order=tantivy.Order.Asc
        )

        assert len(result.hits) == 3

        _, doc_address = result.hits[2]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Final test title"]

        _, doc_address = result.hits[1]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Another test title"]

        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Test title"]

    def test_order_by_search_without_fast_field(self):
        schema = (
            SchemaBuilder()
            .add_unsigned_field("order")
            .add_text_field("title", stored=True)
            .build()
        )

        index = Index(schema)
        writer = index.writer()

        doc = Document()
        doc.add_unsigned("order", 0)
        doc.add_text("title", "Test title")

        query = index.parse_query("test")

        searcher = index.searcher()
        result = searcher.search(query, 10, order_by_field="order")
        assert len(result.hits) == 0

    def test_with_merges(self):
        # This test is taken from tantivy's test suite:
        # https://github.com/quickwit-oss/tantivy/blob/42acd334f49d5ff7e4fe846b5c12198f24409b50/src/indexer/index_writer.rs#L1130
        schema = SchemaBuilder().add_text_field("text", stored=True).build()

        index = Index(schema)
        index.config_reader(reload_policy="Manual")

        writer = index.writer()

        for _ in range(100):
            doc = Document()
            doc.add_text("text", "a")

            writer.add_document(doc)

        writer.commit()

        for _ in range(100):
            doc = Document()
            doc.add_text("text", "a")

            writer.add_document(doc)

        # This should create 8 segments and trigger a merge.
        writer.commit()
        writer.wait_merging_threads()

        # Accessing the writer again should result in an error.
        with pytest.raises(RuntimeError):
            writer.wait_merging_threads()

        index.reload()

        query = index.parse_query("a")
        searcher = index.searcher()
        result = searcher.search(query, limit=500, count=True)
        assert result.count == 200

        assert searcher.num_segments < 8

    def test_doc_from_dict_numeric_validation(self):
        schema = (
            SchemaBuilder()
            .add_unsigned_field("unsigned")
            .add_integer_field("signed")
            .add_float_field("float")
            .build()
        )

        good = Document.from_dict(
            {"unsigned": 1000, "signed": -5, "float": 0.4},
            schema,
        )

        good = Document.from_dict(
            {"unsigned": 1000, "signed": -5, "float": 0.4},
            schema,
        )

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {"unsigned": -50, "signed": -5, "float": 0.4},
                schema,
            )

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {"unsigned": 1000, "signed": 50.4, "float": 0.4},
                schema,
            )

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {
                    "unsigned": 1000,
                    "signed": -5,
                    "float": "bad_string",
                },
                schema,
            )

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {
                    "unsigned": [1000, -50],
                    "signed": -5,
                    "float": 0.4,
                },
                schema,
            )

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {
                    "unsigned": 1000,
                    "signed": [-5, 150, -3.14],
                    "float": 0.4,
                },
                schema,
            )

    def test_doc_from_dict_bytes_validation(self):
        schema = SchemaBuilder().add_bytes_field("bytes").build()

        good = Document.from_dict({"bytes": b"hello"}, schema)
        good = Document.from_dict({"bytes": [[1, 2, 3], [4, 5, 6]]}, schema)
        good = Document.from_dict({"bytes": [1, 2, 3]}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"bytes": [1, 2, 256]}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"bytes": "hello"}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"bytes": [1024, "there"]}, schema)

    def test_doc_from_dict_ip_addr_validation(self):
        schema = SchemaBuilder().add_ip_addr_field("ip").build()

        good = Document.from_dict({"ip": "127.0.0.1"}, schema)
        good = Document.from_dict({"ip": "::1"}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"ip": 12309812348}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"ip": "256.100.0.1"}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {"ip": "1234:5678:9ABC:DEF0:1234:5678:9ABC:DEF0:1234"}, schema
            )

        with pytest.raises(ValueError):
            bad = Document.from_dict(
                {"ip": "1234:5678:9ABC:DEF0:1234:5678:9ABC:GHIJ"}, schema
            )

    def test_doc_from_dict_json_validation(self):
        # Test implicit JSON
        good = Document.from_dict({"dict": {"hello": "world"}})

        schema = SchemaBuilder().add_json_field("json").build()

        good = Document.from_dict({"json": {}}, schema)
        good = Document.from_dict({"json": {"hello": "world"}}, schema)
        good = Document.from_dict(
            {"nested": {"hello": ["world", "!"]}, "numbers": [1, 2, 3]}, schema
        )

        list_of_jsons = [
            {"hello": "world"},
            {"nested": {"hello": ["world", "!"]}, "numbers": [1, 2, 3]},
        ]
        good = Document.from_dict({"json": list_of_jsons}, schema)

        good = Document.from_dict({"json": json.dumps(list_of_jsons[1])}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"json": 123}, schema)

        with pytest.raises(ValueError):
            bad = Document.from_dict({"json": "hello"}, schema)

    def test_search_result_eq(self, ram_index, spanish_index):
        eng_index = ram_index
        eng_query = eng_index.parse_query("sea whale", ["title", "body"])

        esp_index = spanish_index
        esp_query = esp_index.parse_query("vieja", ["title", "body"])

        eng_result1 = eng_index.searcher().search(eng_query, 10)
        eng_result2 = eng_index.searcher().search(eng_query, 10)
        esp_result = esp_index.searcher().search(esp_query, 10)

        assert eng_result1 == eng_result2
        assert eng_result1 != esp_result
        assert eng_result2 != esp_result

    def test_search_result_pickle(self, ram_index):
        index = ram_index
        query = index.parse_query("sea whale", ["title", "body"])

        orig = index.searcher().search(query, 10)
        pickled = pickle.loads(pickle.dumps(orig))

        assert orig == pickled

    def test_delete_all_documents(self, ram_index):
        index = ram_index
        writer = index.writer()
        writer.delete_all_documents()
        writer.commit()

        index.reload()
        query = index.parse_query("sea whale", ["title", "body"])
        result = index.searcher().search(query, 10)

        assert len(result.hits) == 0


class TestUpdateClass(object):
    def test_delete_update(self, ram_index):
        query = ram_index.parse_query("Frankenstein", ["title"])
        result = ram_index.searcher().search(query, 10)
        assert len(result.hits) == 1

        writer = ram_index.writer()

        with pytest.raises(ValueError):
            writer.delete_documents("fake_field", "frankenstein")

        with pytest.raises(ValueError):
            writer.delete_documents("title", b"frankenstein")

        writer.delete_documents("title", "frankenstein")
        writer.commit()
        ram_index.reload()

        result = ram_index.searcher().search(query)
        assert len(result.hits) == 0


class TestFromDiskClass(object):
    def test_opens_from_dir_invalid_schema(self, dir_index):
        invalid_schema = SchemaBuilder().add_text_field("🐱").build()
        index_dir, _ = dir_index
        with pytest.raises(ValueError):
            Index(invalid_schema, str(index_dir), reuse=True)

    def test_opens_from_dir(self, dir_index):
        index_dir, _ = dir_index

        index = Index(schema(), str(index_dir), reuse=True)
        assert index.searcher().num_docs == 3

    def test_create_readers(self):
        # not sure what is the point of this test.
        idx = Index(schema())
        idx.config_reader("Manual", 4)
        assert idx.searcher().num_docs == 0
        # by default this is manual mode
        writer = idx.writer(30000000, 1)
        writer.add_document(Document(title="mytitle", body="mybody"))
        writer.commit()
        assert idx.searcher().num_docs == 0
        # Manual is the default setting.
        # In this case, change are reflected only when
        # the index is manually reloaded.
        idx.reload()
        assert idx.searcher().num_docs == 1
        idx.config_reader("OnCommit", 4)
        writer.add_document(Document(title="mytitle2", body="mybody2"))
        writer.commit()
        import time

        for i in range(50):
            # The index should be automatically reloaded.
            # Wait for at most 5s for it to happen.
            time.sleep(0.1)
            if idx.searcher().num_docs == 2:
                return
        assert False


class TestSearcher(object):
    def test_searcher_repr(self, ram_index, ram_index_numeric_fields):
        assert repr(ram_index.searcher()) == "Searcher(num_docs=3, num_segments=1)"
        assert (
            repr(ram_index_numeric_fields.searcher())
            == "Searcher(num_docs=2, num_segments=1)"
        )


class TestDocument(object):
    def test_document(self):
        doc = tantivy.Document(name="Bill", reference=[1, 2])
        assert doc["reference"] == [1, 2]
        assert doc["name"] == ["Bill"]
        assert doc.get_first("name") == "Bill"
        assert doc.get_first("reference") == 1
        assert doc.to_dict() == {"name": ["Bill"], "reference": [1, 2]}

    def test_document_with_date(self):
        date = datetime.datetime(2019, 8, 12, 13, 0, 0)
        doc = tantivy.Document(name="Bill", date=date)
        assert doc["date"][0] == date

    def test_document_repr(self):
        doc = tantivy.Document(name="Bill", reference=[1, 2])
        assert repr(doc) == "Document(name=[Bill],reference=[1,2])"

    def test_document_repr_utf8(self):
        doc = tantivy.Document(name="野菜食べないとやばい", reference=[1, 2])
        assert repr(doc) == "Document(name=[野菜食べないとやばい],reference=[1,2])"

    def test_document_with_facet(self):
        doc = tantivy.Document()
        facet = tantivy.Facet.from_string("/europe/france")
        doc.add_facet("facet", facet)
        assert doc["facet"][0].to_path() == ["europe", "france"]
        doc = tantivy.Document()
        facet = tantivy.Facet.from_string("/asia\\/oceania/fiji")
        doc.add_facet("facet", facet)
        assert doc["facet"][0].to_path() == ["asia/oceania", "fiji"]
        assert doc["facet"][0].to_path_str() == "/asia\\/oceania/fiji"
        assert repr(doc["facet"][0]) == "Facet(/asia\\/oceania/fiji)"
        doc = tantivy.Document(facet=facet)
        assert doc["facet"][0].to_path() == ["asia/oceania", "fiji"]

    def test_document_eq(self):
        doc1 = tantivy.Document(name="Bill", reference=[1, 2])
        doc2 = tantivy.Document.from_dict({"name": "Bill", "reference": [1, 2]})
        doc3 = tantivy.Document(name="Bob", reference=[3, 4])

        assert doc1 == doc2
        assert doc1 != doc3
        assert doc2 != doc3

    def test_document_copy(self):
        doc1 = tantivy.Document(name="Bill", reference=[1, 2])
        doc2 = copy.copy(doc1)
        doc3 = copy.deepcopy(doc2)

        assert doc1 == doc2
        assert doc1 == doc3
        assert doc2 == doc3

    def test_document_pickle(self):
        orig = Document()
        orig.add_unsigned("unsigned", 1)
        orig.add_integer("integer", 5)
        orig.add_float("float", 1.0)
        orig.add_date("birth", datetime.datetime(2019, 8, 12, 13, 0, 5))
        orig.add_text("title", "hello world!")
        orig.add_json("json", '{"a": 1, "b": 2}')
        orig.add_bytes("bytes", b"abc")

        facet = tantivy.Facet.from_string("/europe/france")
        orig.add_facet("facet", facet)

        pickled = pickle.loads(pickle.dumps(orig))

        assert orig == pickled


class TestJsonField:
    def test_query_from_json_field(self):
        schema = (
            SchemaBuilder()
            .add_json_field(
                "attributes",
                stored=True,
                tokenizer_name="default",
                index_option="position",
            )
            .build()
        )

        index = Index(schema)

        writer = index.writer()

        doc = Document()
        doc.add_json(
            "attributes",
            """{
                "order":1.1,
                "target": "submit-button",
                "cart": {"product_id": 103},
                "description": "the best vacuum cleaner ever"
            }""",
        )

        writer.add_document(doc)

        doc = Document()
        doc.add_json(
            "attributes",
            {
                "order": 1.2,
                "target": "submit-button",
                "cart": {"product_id": 133},
                "description": "das keyboard",
            },
        )

        writer.add_document(doc)

        writer.commit()
        index.reload()

        query = index.parse_query("target:submit-button", ["attributes"])
        result = index.searcher().search(query, 2)
        assert len(result.hits) == 2

        query = index.parse_query("target:submit", ["attributes"])
        result = index.searcher().search(query, 2)
        assert len(result.hits) == 2

        query = index.parse_query("order:1.1", ["attributes"])
        result = index.searcher().search(query, 2)
        assert len(result.hits) == 1

        # query = index.parse_query_for_attributes("cart.product_id:103")
        # result = index.searcher().search(query, 1)
        # assert len(result.hits) == 1

        # query = index.parse_query_for_attributes(
        #     "target:submit-button AND cart.product_id:133"
        # )
        # result = index.searcher().search(query, 2)
        # assert len(result.hits) == 1


@pytest.mark.parametrize("bytes_kwarg", [True, False])
@pytest.mark.parametrize(
    "bytes_payload",
    [
        b"abc",
        bytearray(b"abc"),
        memoryview(b"abc"),
        BytesIO(b"abc").read(),
        BytesIO(b"abc").getbuffer(),
    ],
)
def test_bytes(bytes_kwarg, bytes_payload):
    schema = SchemaBuilder().add_bytes_field("embedding").build()
    index = Index(schema)
    writer = index.writer()

    if bytes_kwarg:
        doc = Document(id=1, embedding=bytes_payload)
    else:
        doc = Document(id=1)
        doc.add_bytes("embedding", bytes_payload)

    writer.add_document(doc)
    writer.commit()
    index.reload()


def test_schema_eq():
    schema1 = schema()
    schema2 = schema()
    schema3 = schema_numeric_fields()

    assert schema1 == schema2
    assert schema1 != schema3
    assert schema2 != schema3


def test_facet_eq():
    facet1 = tantivy.Facet.from_string("/europe/france")
    facet2 = tantivy.Facet.from_string("/europe/france")
    facet3 = tantivy.Facet.from_string("/europe/germany")

    assert facet1 == facet2
    assert facet1 != facet3
    assert facet2 != facet3


def test_schema_pickle():
    orig = (
        SchemaBuilder()
        .add_integer_field("id", stored=True, indexed=True)
        .add_unsigned_field("unsigned")
        .add_float_field("rating", stored=True, indexed=True)
        .add_text_field("body", stored=True)
        .add_date_field("date")
        .add_json_field("json")
        .add_bytes_field("bytes")
        .build()
    )

    pickled = pickle.loads(pickle.dumps(orig))

    assert orig == pickled


def test_facet_pickle():
    orig = tantivy.Facet.from_string("/europe/france")
    pickled = pickle.loads(pickle.dumps(orig))

    assert orig == pickled


def test_doc_address_pickle():
    orig = tantivy.DocAddress(42, 123)
    pickled = pickle.loads(pickle.dumps(orig))

    assert orig == pickled


class TestSnippets(object):
    def test_document_snippet(self, dir_index):
        index_dir, _ = dir_index
        doc_schema = schema()
        index = Index(doc_schema, str(index_dir))
        query = index.parse_query("sea whale", ["title", "body"])
        searcher = index.searcher()
        result = searcher.search(query)
        assert len(result.hits) == 1

        snippet_generator = SnippetGenerator.create(
            searcher, query, doc_schema, "title"
        )

        for score, doc_address in result.hits:
            doc = searcher.doc(doc_address)
            snippet = snippet_generator.snippet_from_doc(doc)
            highlights = snippet.highlighted()
            assert len(highlights) == 1
            first = highlights[0]
            assert first.start == 20
            assert first.end == 23
            html_snippet = snippet.to_html()
            assert html_snippet == "The Old Man and the <b>Sea</b>"


class TestQuery(object):
    def test_term_query(self, ram_index):
        index = ram_index
        query = Query.term_query(index.schema, "title", "sea")

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["The Old Man and the Sea"]

    def test_term_set_query(self, ram_index):
        index = ram_index

        # Should match 1 document that contains both terms
        terms = ["old", "man"]
        query = Query.term_set_query(index.schema, "title", terms)
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["The Old Man and the Sea"]

        # Should not match any document since the term does not exist in the index
        terms = ["a long term that does not exist in the index"]
        query = Query.term_set_query(index.schema, "title", terms)
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 0

        # Should not match any document when the terms list is empty
        terms = []
        query = Query.term_set_query(index.schema, "title", terms)
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 0

        # Should fail to create the query due to the invalid list object in the terms list
        with pytest.raises(ValueError, match = r"Can't create a term for Field `title` with value `\[\]`"):
            terms = ["old", [], "man"]
            query = Query.term_set_query(index.schema, "title", terms)

    def test_all_query(self, ram_index):
        index = ram_index
        query = Query.all_query()

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 3

    #def test_phrase_query(self, ram_index):
    #    index = ram_index
    #    searcher = index.searcher()
    #
    #    query = Query.phrase_query(index.schema, "title", ["old", "man"])
    #    result = searcher.search(query, 10)
    #    assert len(result.hits) == 1
    #
    #    query = Query.phrase_query(index.schema, "title", ["man", "old"])
    #    # sholdn't match any document
    #    result = searcher.search(query, 10)
    #    assert len(result.hits) == 0
    #
    #    query = Query.phrase_query(index.schema, "title", ["man", "sea"])
    #    # sholdn't match any document
    #    result = searcher.search(query, 10)
    #    assert len(result.hits) == 0

    def test_phrase_query_offset_slop(self, ram_index):
        index = ram_index
        searcher = index.searcher()

        query = Query.phrase_query_offset_slop(index.schema, "title", ["man", "old"], [1, 0])
        # should match "old man" with custom offsets
        result = searcher.search(query, 10)
        assert len(result.hits) == 1

        query = Query.phrase_query_offset_slop(index.schema, "title", ["man", "sea"], [0, 1], 2)
        # should match "man and the sea" with slop = 2
        result = searcher.search(query, 10)
        assert len(result.hits) == 1
    def test_fuzzy_term_query(self, ram_index):
        index = ram_index
        query = Query.fuzzy_term_query(index.schema, "title", "ice")
        # the query "ice" should match "mice"
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Of Mice and Men"]

        query = Query.fuzzy_term_query(index.schema, "title", "mna")
        # the query "mna" should match "man" since the default transposition cost is 1.
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        titles = set()
        for _, doc_address in result.hits:
            titles.update(index.searcher().doc(doc_address)["title"])
        assert titles == {"The Old Man and the Sea"}

        query = Query.fuzzy_term_query(index.schema, "title", "mna", transposition_cost_one=False)
        # the query "mna" should not match any doc since the default distance is 1 and transposition cost is set to 2.
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 0

        query = Query.fuzzy_term_query(index.schema, "title", "mna", distance=2, transposition_cost_one=False)
        # the query "mna" should match both "man" and "men" since distance is set to 2.
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2
        titles = set()
        for _, doc_address in result.hits:
            titles.update(index.searcher().doc(doc_address)["title"])
        assert titles == {"The Old Man and the Sea", "Of Mice and Men"}

        query = Query.fuzzy_term_query(index.schema, "title", "fraken")
        # the query "fraken" should not match any doc.
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 0

        query = Query.fuzzy_term_query(index.schema, "title", "fraken", prefix=True)
        # the query "fraken" should match "franken", the prefix of "frankenstein", with edit distance 1.
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        titles = set()
        for _, doc_address in result.hits:
            titles.update(index.searcher().doc(doc_address)["title"])
        assert titles == {"Frankenstein", "The Modern Prometheus"}

    def test_boolean_query(self, ram_index):
        index = ram_index
        query1 = Query.fuzzy_term_query(index.schema, "title", "ice")
        query2 = Query.fuzzy_term_query(index.schema, "title", "mna")
        query = Query.boolean_query([
            (Occur.Must, query1), 
            (Occur.Must, query2)
        ])

        # no document should match both queries
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 0

        query = Query.boolean_query([
            (Occur.Should, query1), 
            (Occur.Should, query2)
        ])

        # two documents should match, one for each query
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2

        titles = set()
        for _, doc_address in result.hits:
            titles.update(index.searcher().doc(doc_address)["title"])
        assert (
            "The Old Man and the Sea" in titles and  
            "Of Mice and Men" in titles
        )

        query = Query.boolean_query([
            (Occur.MustNot, query1), 
            (Occur.Must, query1)
        ])

        # must not should take precedence over must
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 0

        query = Query.boolean_query((
            (Occur.Should, query1), 
            (Occur.Should, query2)
        ))

        # the Vec signature should fit the tuple signature
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2

        # test invalid queries
        with pytest.raises(ValueError, match = "expected tuple of length 2, but got tuple of length 3"):
            Query.boolean_query([
                (Occur.Must, Occur.Must, query1),
            ])

        # test swapping the order of the tuple
        with pytest.raises(TypeError, match = r"'Query' object cannot be converted to 'Occur'"):
            Query.boolean_query([
                (query1, Occur.Must),
            ])

    def test_disjunction_max_query(self, ram_index):
        index = ram_index

        # query1 should match the doc: "The Old Man and the Sea"
        query1 = Query.term_query(index.schema, "title", "sea")
        # query2 should matches the doc: "Of Mice and Men"
        query2 = Query.term_query(index.schema, "title", "mice")
        # the disjunction max query should match both docs.
        query = Query.disjunction_max_query([query1, query2])

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2

        # the disjunction max query should also take a tie_breaker parameter
        query = Query.disjunction_max_query([query1, query2], tie_breaker=0.5)
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2

        with pytest.raises(TypeError, match = r"'str' object cannot be converted to 'Query'"):
            query = Query.disjunction_max_query([query1, "not a query"], tie_breaker=0.5)


    def test_boost_query(self, ram_index):
        index = ram_index
        query1 = Query.term_query(index.schema, "title", "sea")
        boosted_query = Query.boost_query(query1, 2.0)

        # Normal boost query
        assert (
            repr(boosted_query)
            == """Query(Boost(query=TermQuery(Term(field=0, type=Str, "sea")), boost=2))"""
        )

        query2 = Query.fuzzy_term_query(index.schema, "title", "ice")
        combined_query = Query.boolean_query([
            (Occur.Should, boosted_query), 
            (Occur.Should, query2)
        ])
        boosted_query = Query.boost_query(combined_query, 2.0)

        # Boosted boolean query
        assert (
            repr(boosted_query)
            == """Query(Boost(query=BooleanQuery { subqueries: [(Should, Boost(query=TermQuery(Term(field=0, type=Str, "sea")), boost=2)), (Should, FuzzyTermQuery { term: Term(field=0, type=Str, "ice"), distance: 1, transposition_cost_one: true, prefix: false })] }, boost=2))"""
        )

        boosted_query = Query.boost_query(query1, 0.1)

        # Check for decimal boost values
        assert(
            repr(boosted_query)
            == """Query(Boost(query=TermQuery(Term(field=0, type=Str, "sea")), boost=0.1))"""
        )

        boosted_query = Query.boost_query(query1, 0.0)

        # Check for zero boost values
        assert(
            repr(boosted_query)
            == """Query(Boost(query=TermQuery(Term(field=0, type=Str, "sea")), boost=0))"""
        )
        result = index.searcher().search(boosted_query, 10)
        for _score, _ in result.hits:
            # the score should be 0.0
            assert _score == pytest.approx(0.0)  

        boosted_query = Query.boost_query(
            Query.boost_query(
                query1, 0.1
            ), 0.1
        )

        # Check for nested boost queries
        assert(
            repr(boosted_query)
            == """Query(Boost(query=Boost(query=TermQuery(Term(field=0, type=Str, "sea")), boost=0.1), boost=0.1))"""
        )
        result = index.searcher().search(boosted_query, 10)
        for _score, _ in result.hits:
            # the score should be very small, due to 
            # the unknown score of BM25, we can only check for the relative difference
            assert _score == pytest.approx(0.01, rel = 1)  


        boosted_query = Query.boost_query(
            query1, -0.1
        )

        # Check for negative boost values
        assert(
            repr(boosted_query)
            == """Query(Boost(query=TermQuery(Term(field=0, type=Str, "sea")), boost=-0.1))"""
        )

        result = index.searcher().search(boosted_query, 10)
        # Even with a negative boost, the query should still match the document
        assert len(result.hits) == 1
        titles = set()
        for _score, doc_address in result.hits:

            # the score should be negative
            assert _score < 0
            titles.update(index.searcher().doc(doc_address)["title"])
        assert titles == {"The Old Man and the Sea"}

        # wrong query type
        with pytest.raises(TypeError, match = r"'int' object cannot be converted to 'Query'"):
            Query.boost_query(1, 0.1)

        # wrong boost type
        with pytest.raises(TypeError, match = r"argument 'boost': must be real number, not str"):
            Query.boost_query(query1, "0.1")
        
        # no boost type error
        with pytest.raises(TypeError, match = r"Query.boost_query\(\) missing 1 required positional argument: 'boost'"):
            Query.boost_query(query1)


    def test_regex_query(self, ram_index):
        index = ram_index

        query = Query.regex_query(index.schema, "body", "fish")
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1
        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["The Old Man and the Sea"]

        query = Query.regex_query(index.schema, "title", "(?:man|men)")
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2
        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["The Old Man and the Sea"]
        _, doc_address = result.hits[1]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["Of Mice and Men"]

        # unknown field in the schema
        with pytest.raises(
            ValueError, match="Field `unknown_field` is not defined in the schema."
        ):
            Query.regex_query(index.schema, "unknown_field", "fish")

        # invalid regex pattern
        with pytest.raises(
            ValueError, match=r"An invalid argument was passed: 'fish\('"
        ):
            Query.regex_query(index.schema, "body", "fish(")
