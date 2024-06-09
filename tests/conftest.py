from datetime import datetime
import pytest

from tantivy import SchemaBuilder, Index, Document


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
        .add_integer_field("id", stored=True, indexed=True, fast=True)
        .add_float_field("rating", stored=True, indexed=True, fast=True)
        .add_boolean_field("is_good", stored=True, indexed=True)
        .add_text_field("body", stored=True, fast=True)
        .build()
    )

def schema_with_date_field():
    return (
        SchemaBuilder()
        .add_integer_field("id", stored=True, indexed=True)
        .add_float_field("rating", stored=True, indexed=True)
        .add_date_field("date", stored=True, indexed=True)
        .build()
    )

def schema_with_ip_addr_field():
    return (
        SchemaBuilder()
        .add_integer_field("id", stored=True, indexed=True)
        .add_float_field("rating", stored=True, indexed=True)
        .add_ip_addr_field("ip_addr", stored=True, indexed=True)
        .build()
    )

def create_index(dir=None):
    # assume all tests will use the same documents for now
    # other methods may set up function-local indexes
    index = Index(schema(), dir)
    writer = index.writer(15_000_000, 1)

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
    writer.wait_merging_threads()
    index.reload()
    return index


def create_index_with_numeric_fields(dir=None):
    index = Index(schema_numeric_fields(), dir)
    writer = index.writer(15_000_000, 1)

    doc = Document()
    doc.add_integer("id", 1)
    doc.add_float("rating", 3.5)
    doc.add_boolean("is_good", True)
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
            "is_good": False,
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
        },
    )
    writer.add_document(doc)
    writer.commit()
    writer.wait_merging_threads()
    index.reload()
    return index

def create_index_with_date_field(dir=None):
    index = Index(schema_with_date_field(), dir)
    writer = index.writer(15_000_000, 1)

    doc = Document()
    doc.add_integer("id", 1)
    doc.add_float("rating", 3.5)
    doc.add_date("date", datetime(2021, 1, 1))
    
    writer.add_document(doc)
    doc = Document.from_dict(
        {
            "id": 2,
            "rating": 4.5,
            "date": datetime(2021, 1, 2),
        },
    )
    writer.add_document(doc)
    writer.commit()
    writer.wait_merging_threads()
    index.reload()
    return index           

def create_index_with_ip_addr_field(dir=None):
    schema = schema_with_ip_addr_field()
    index = Index(schema, dir)
    writer = index.writer(15_000_000, 1)

    doc = Document()
    doc.add_integer("id", 1)
    doc.add_float("rating", 3.5)
    doc.add_ip_addr("ip_addr", "10.0.0.1")
    writer.add_document(doc)
    
    doc = Document.from_dict(
        {
            "id": 2,
            "rating": 4.5,
            "ip_addr": "127.0.0.1",
        },
        schema
    )
    writer.add_document(doc)
    doc = Document.from_dict(
        {
            "id": 2,
            "rating": 4.5,
            "ip_addr": "::1",
        },
        schema
    )
    writer.add_document(doc)
    writer.commit()
    writer.wait_merging_threads()
    index.reload()
    return index

def spanish_schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True, tokenizer_name="es_stem")
        .add_text_field("body", tokenizer_name="es_stem")
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
    writer.wait_merging_threads()
    index.reload()
    return index


@pytest.fixture()
def dir_index(tmpdir):
    return (tmpdir, create_index(str(tmpdir)))


@pytest.fixture(scope="class")
def ram_index():
    return create_index()


@pytest.fixture(scope="class")
def ram_index_numeric_fields():
    return create_index_with_numeric_fields()

@pytest.fixture(scope="class")
def ram_index_with_date_field():
    return create_index_with_date_field()

@pytest.fixture(scope="class")
def ram_index_with_ip_addr_field():
    return create_index_with_ip_addr_field()

@pytest.fixture(scope="class")
def spanish_index():
    return create_spanish_index()
