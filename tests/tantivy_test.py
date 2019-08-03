import json
import tantivy

import pytest


@pytest.fixture(scope="class")
def ram_index():
        # assume all tests will use the same documents for now
        # other methods may set up function-local indexes
        builder = tantivy.SchemaBuilder()

        title = builder.add_text_field("title", stored=True)
        body = builder.add_text_field("body")

        schema = builder.build()
        index = tantivy.Index(schema)

        writer = index.writer()

        # 2 ways of adding documents
        # 1
        doc = tantivy.Document()
        # create a document instance
        # add field-value pairs
        doc.add_text(title, "The Old Man and the Sea")
        doc.add_text(body, ("He was an old man who fished alone in a skiff in"
                            "the Gulf Stream and he had gone eighty-four days "
                            "now without taking a fish."))
        writer.add_document(doc)
        # 2 use the built-in json support
        # keys need to coincide with field names
        doc = schema.parse_document(json.dumps({
            "title": "Of Mice and Men",
            "body": ("A few miles south of Soledad, the Salinas River drops "
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
                     "branches that arch over the pool")
        }))

        writer.add_document(doc)

        doc = schema.parse_document(json.dumps({
            "title": ["Frankenstein", "The Modern Prometheus"],
            "body": ("You will rejoice to hear that no disaster has "
                     "accompanied the commencement of an enterprise which you "
                     "have regarded with such evil forebodings.  I arrived "
                     "here yesterday, and my first task is to assure my dear "
                     "sister of my welfare and increasing confidence in the "
                     "success of my undertaking.")
        }))

        writer.add_document(doc)
        writer.commit()

        reader = index.reader()
        searcher = reader.searcher()
        index = index
        schema = schema
        default_args = [title, body]
        ret = (index, searcher, schema, default_args, title, body)
        return ret


class TestClass(object):

    def test_simple_search(self, ram_index):
        index, searcher, schema, default_args, title, body = ram_index
        query_parser = tantivy.QueryParser.for_index(index, default_args)
        query = query_parser.parse_query("sea whale")

        top_docs = tantivy.TopDocs(10)

        result = searcher.search(query, top_docs)
        print(result)

        assert len(result) == 1

        _, doc_address = result[0]

        searched_doc = searcher.doc(doc_address)
        assert searched_doc.get_first(title) == "The Old Man and the Sea"

    def test_doc(self):
        builder = tantivy.SchemaBuilder()
        title = builder.add_text_field("title", stored=True)

        doc = tantivy.Document()
        assert doc.is_empty

        doc.add_text(title, "The Old Man and the Sea")

        assert doc.get_first(title) == "The Old Man and the Sea"

        assert doc.len == 1
        assert not doc.is_empty

    def test_and_query(self, ram_index):
        index, searcher, schema, default_args, title, body = ram_index
        q_parser = tantivy.QueryParser.for_index(index, default_args)
        # look for an intersection of documents
        query = q_parser.parse_query("title:men AND body:summer")
        top_docs = tantivy.TopDocs(10)

        result = searcher.search(query, top_docs)
        print(result)

        # summer isn't present
        assert len(result) == 0

        query = q_parser.parse_query("title:men AND body:winter")
        result = searcher.search(query, top_docs)

        assert len(result) == 1

    def test_query_errors(self, ram_index):
        index, searcher, schema, default_args, title, body = ram_index
        q_parser = tantivy.QueryParser.for_index(index, default_args)
        # no "bod" field
        with pytest.raises(ValueError):
            q_parser.parse_query("bod:title")


@pytest.fixture(scope="class")
def disk_index():
    builder = tantivy.SchemaBuilder()
    title = builder.add_text_field("title", stored=True)
    body = builder.add_text_field("body")
    default_args = [title, body]
    schema = builder.build()
    schema = schema
    index = tantivy.Index(schema)
    path_to_index = "tests/test_index/"
    return index, path_to_index, schema, default_args, title, body


class TestFromDiskClass(object):

    def test_exists(self, disk_index):
        # prefer to keep it separate in case anyone deletes this
        # runs from the root directory
        index, path_to_index, _, _, _, _ = disk_index
        assert index.exists(path_to_index)

    def test_opens_from_dir(self, disk_index):
        _, path_to_index, schema, _, _, _ = disk_index
        tantivy.Index(schema, path_to_index)

    def test_create_readers(self, disk_index):
        _, path_to_index, schema, _, _, _ = disk_index
        idx = tantivy.Index(schema, path_to_index)
        reload_policy = "OnCommit"  # or "Manual"
        assert idx.reader(reload_policy, 4)
        assert idx.reader("Manual", 4)

    def test_create_writer_and_reader(self, disk_index):
        _, path_to_index, schema, default_args, title, body = disk_index
        idx = tantivy.Index(schema, path_to_index)
        writer = idx.writer()
        reload_policy = "OnCommit"  # or "Manual"
        reader = idx.reader(reload_policy, 4)

        # check against the opstamp in the meta file
        meta_fname = "meta.json"
        with open("{}{}".format(path_to_index, meta_fname)) as f:
            json_file = json.load(f)
            expected_last_opstamp = json_file["opstamp"]
            # ASSUMPTION
            # We haven't had any deletes in the index
            # so max_doc per index coincides with the value of `num_docs`
            # summing them in all segments, gives the number of documents
            expected_num_docs = sum([segment["max_doc"]
                                     for segment in json_file["segments"]])
        assert writer.commit_opstamp == expected_last_opstamp

        q_parser = tantivy.QueryParser.for_index(idx, default_args)
        # get all documents
        query = q_parser.parse_query("*")
        top_docs = tantivy.TopDocs(10)

        docs = reader.searcher().search(query, top_docs)
        for (_score, doc_addr) in docs:
            print(reader.searcher().doc(doc_addr))
        assert expected_num_docs == len(docs)
