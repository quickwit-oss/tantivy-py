import tantivy
import pytest

from tantivy import Document, Index, SchemaBuilder


def schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True)
        .add_text_field("body")
        .build()
    )

def schema_numeric_fields():
    return (
        SchemaBuilder()
        .add_integer_field("id", stored=True, indexed=True)
        .add_float_field("rating", stored=True, indexed=True)
        .add_text_field("body", stored=True)
        .build()
    )

def create_index(dir=None):
    # assume all tests will use the same documents for now
    # other methods may set up function-local indexes
    index = Index(schema(), dir)
    writer = index.writer()

    # 2 ways of adding documents
    # 1
    doc = Document()
    # create a document instance
    # add field-value pairs
    doc.add_text("title", "The Old Man and the Sea")
    doc.add_text(
        "body",
        (
            "He was an old man who fished alone in a skiff in"
            "the Gulf Stream and he had gone eighty-four days "
            "now without taking a fish."
        ),
    )
    writer.add_document(doc)
    # 2 use the built-in json support
    # keys need to coincide with field names
    doc = Document.from_dict(
        {
            "title": "Of Mice and Men",
            "body": (
                "A few miles south of Soledad, the Salinas River drops "
                "in close to the hillside bank and runs deep and "
                "green. The water is warm too, for it has slipped "
                "twinkling over the yellow sands in the sunlight "
                "before reaching the narrow pool. On one side of the "
                "river the golden foothill slopes curve up to the "
                "strong and rocky Gabilan Mountains, but on the valley "
                "side the water is lined with trees—willows fresh and "
                "green with every spring, carrying in their lower leaf "
                "junctures the debris of the winter’s flooding; and "
                "sycamores with mottled, white, recumbent limbs and "
                "branches that arch over the pool"
            ),
        }
    )
    writer.add_document(doc)
    writer.add_json(
        """{
            "title": ["Frankenstein", "The Modern Prometheus"],
            "body": "You will rejoice to hear that no disaster has accompanied the commencement of an enterprise which you have regarded with such evil forebodings.  I arrived here yesterday, and my first task is to assure my dear sister of my welfare and increasing confidence in the success of my undertaking."
        }"""
    )
    writer.commit()
    index.reload()
    return index

def create_index_with_numeric_fields(dir=None):
    index = Index(schema_numeric_fields(), dir)
    writer = index.writer()

    doc = Document()
    doc.add_integer("id", 1)
    doc.add_float("rating", 3.5)
    doc.add_text(
        "body",
        (
            "He was an old man who fished alone in a skiff in"
            "the Gulf Stream and he had gone eighty-four days "
            "now without taking a fish."
        ),
    )
    writer.add_document(doc)
    doc = Document.from_dict(
        {
            "id": 2,
            "rating": 4.5,
            "body": (
                "A few miles south of Soledad, the Salinas River drops "
                "in close to the hillside bank and runs deep and "
                "green. The water is warm too, for it has slipped "
                "twinkling over the yellow sands in the sunlight "
                "before reaching the narrow pool. On one side of the "
                "river the golden foothill slopes curve up to the "
                "strong and rocky Gabilan Mountains, but on the valley "
                "side the water is lined with trees—willows fresh and "
                "green with every spring, carrying in their lower leaf "
                "junctures the debris of the winter’s flooding; and "
                "sycamores with mottled, white, recumbent limbs and "
                "branches that arch over the pool"
            ),
        }
    )
    writer.add_document(doc)
    writer.commit()
    index.reload()
    return index

def spanish_schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True, tokenizer_name='es_stem')
        .add_text_field("body", tokenizer_name='es_stem')
        .build()
    )


def create_spanish_index():
    # assume all tests will use the same documents for now
    # other methods may set up function-local indexes
    index = Index(spanish_schema(), None)
    writer = index.writer()

    # 2 ways of adding documents
    # 1
    doc = Document()
    # create a document instance
    # add field-value pairs
    doc.add_text("title", "El viejo y el mar")
    doc.add_text(
        "body",
        (
            "Era un viejo que pescaba solo en un bote en el Gulf Stream y hacía ochenta y cuatro días que no cogía un pez. "
        ),
    )
    writer.add_document(doc)
    # 2 use the built-in json support
    # keys need to coincide with field names
    doc = Document.from_dict(
        {
            "title": "De ratones y hombres",
            "body": (
                "Unas millas al sur de Soledad, el río Salinas se ahonda junto al margen de la ladera y fluye profundo y verde. Es tibia el agua, porque se ha deslizado chispeante sobre la arena amarilla y al calor del sol antes de llegar a la angosta laguna. A un lado del río, la dorada falda de la ladera se curva hacia arriba trepando hasta las montañas Gabilán, fuertes y rocosas, pero del lado del valle los árboles bordean la orilla: sauces frescos y verdes cada primavera, que en la s junturas más bajas de sus hojas muestran las consecuencias de la crecida invernal; y sicomoros de troncos veteados, blancos, recostados, y ramas quesear quean sobre el estanque"
            ),
        }
    )
    writer.add_document(doc)
    writer.add_json(
        """{
            "title": ["Frankenstein", "El moderno Prometeo"],
            "body": "Te alegrará saber que no ha ocurrido ningún percance al principio de una aventura que siempre consideraste cargada de malos presagios. Llegué aquí ayer, y mi primera tarea es asegurarle a mi querida hermana que me hallo perfectamente y que tengo una gran confianza en el éxito de mi empresa."
        }"""
    )
    writer.commit()
    index.reload()
    return index


@pytest.fixture()
def dir_index(tmpdir):
    return (tmpdir, create_index(str(tmpdir)))


@pytest.fixture(scope="class")
def ram_index():
    return create_index()


@pytest.fixture(scope="class")
def spanish_index():
    return create_spanish_index()


@pytest.fixture(scope="class")
def ram_index_numeric_fields():
    return create_index_with_numeric_fields()


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
        assert searcher.doc(result.hits[0][1])['rating'][0] == 3.5

        integer_query = index.parse_query("1", ["id"])
        result = searcher.search(integer_query)
        assert len(result.hits) == 1

        # 0 result
        integer_query = index.parse_query("10", ["id"])
        result = searcher.search(integer_query)
        assert len(result.hits) == 0

    def test_and_query_parser_default_fields(self, ram_index):
        query = ram_index.parse_query("winter", default_field_names=["title"])
        assert repr(query) == """Query(TermQuery(Term(type=Str, field=0, "winter")))"""

    def test_and_query_parser_default_fields_undefined(self, ram_index):
        query = ram_index.parse_query("winter")
        assert (
            repr(query)
            == """Query(BooleanQuery { subqueries: [(Should, TermQuery(Term(type=Str, field=0, "winter"))), (Should, TermQuery(Term(type=Str, field=1, "winter")))] })"""
        )

    def test_query_errors(self, ram_index):
        index = ram_index
        # no "bod" field
        with pytest.raises(ValueError):
            index.parse_query("bod:men", ["title", "body"])

    def test_order_by_search(self):
        schema = (
            SchemaBuilder()
            .add_unsigned_field("order", fast="single")
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
        assert repr(ram_index_numeric_fields.searcher()) == "Searcher(num_docs=2, num_segments=1)"


class TestDocument(object):
    def test_document(self):
        doc = tantivy.Document(name="Bill", reference=[1, 2])
        assert doc["reference"] == [1, 2]
        assert doc["name"] == ["Bill"]
        assert doc.get_first("name") == "Bill"
        assert doc.get_first("reference") == 1
        assert doc.to_dict() == {"name": ["Bill"], "reference": [1, 2]}

    def test_document_with_date(self):
        import datetime

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

    def test_document_error(self):
        with pytest.raises(ValueError):
            tantivy.Document(name={})


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
            """{
                "order":1.2,
                "target": "submit-button",
                "cart": {"product_id": 133},
                "description": "das keyboard"
            }""",
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
