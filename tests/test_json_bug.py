def test_json_bug():
    import tantivy

    schema_builder = tantivy.SchemaBuilder()
    schema_builder.add_json_field("data", stored=True)
    schema = schema_builder.build()

    index = tantivy.Index(schema)

    index_writer = index.writer()

    data = {
        "name": "John Doe",
        "age": 30,
        "email": "john.doe@example.com",
        "interests": ["reading", "hiking", "coding"],
    }
    import json
    json_data = json.dumps(data)

    doc = tantivy.Document()
    doc.add_json("data", json_data)
    index_writer.add_document(doc)
    index_writer.commit()
    index_writer.wait_merging_threads()
    index.reload()

    searcher = index.searcher()

    query = "*"
    q = index.parse_query(query)
    top_docs = searcher.search(q, limit=10)

    print(f"Total hits: {top_docs}")
    for score, hit in top_docs.hits:
        doc = searcher.doc(hit)
        print(doc["data"])
        assert doc["data"] == [{'age': 30,
             'email': 'john.doe@example.com',
             'interests': ['reading', 'hiking', 'coding'],
             'name': 'John Doe'
        }]
