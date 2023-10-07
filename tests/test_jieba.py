import tantivy


def test_jieba() -> None:
    # Declaring our schema.
    schema_builder = tantivy.SchemaBuilder()
    schema_builder.add_text_field("title", stored=True, tokenizer_name="jieba")
    schema_builder.add_text_field("body", stored=True, tokenizer_name="jieba")
    schema_builder.add_integer_field("doc_id", stored=True)
    schema = schema_builder.build()

    # Creating our index (in memory)
    index = tantivy.Index(schema)

    # Adding one document
    writer = index.writer()
    writer.add_document(
        tantivy.Document(
            doc_id=1,
            title=["联合国宪章"],
            body=[
                """
我联合国人民同兹决心欲免后世再遭今代人类两度身历惨不堪言之战祸，
重申基本人权，人格尊严与价值，以及男女与大小各国平等权利之信念，
创造适当环境，俾克维持正义，尊重由条约与国际法其他渊源而起之义务，久而弗懈，
促成大自由中之社会进步及较善之民生，
并为达此目的力行容恕，彼此以善邻之道，和睦相处，
集中力量，以维持国际和平及安全，
接受原则，确立方法，以保证非为公共利益，不得使用武力，
运用国际机构，以促成全球人民经济及社会之进展，
用是发愤立志，务当同心协力，以竟厥功爰由我各本国政府，经齐集金山市之代表各将所奉全权证书，互相校阅，均属妥善，议定本联合国宪章，并设立国际组织，定名联合国。
"""
            ],
        )
    )
    writer.add_document(
        tantivy.Document(
            doc_id=1,
            title=["世界人权宣言"],
            body=[
                """
鉴于对人类家庭所有成员的固有尊严及其平等的和不移的权利的承认，乃是世界自由、正义与和平的基础，
鉴于对人权的无视和侮蔑已发展为野蛮暴行，这些暴行玷污了人类的良心，而一个人人享有言论和信仰自由并免予恐惧和匮乏的世界的来临，已被宣布为普通人民的最高愿望，
鉴于为使人类不致迫不得已铤而走险对暴政和压迫进行反叛，有必要使人权受法治的保护，
鉴于有必要促进各国间友好关系的发展，
鉴于各联合国国家的人民已在联合国宪章中重申他们对基本人权、人格尊严和价值以及男女平等权利的信念，并决心促成较大自由中的社会进步和生活水平的改善，
鉴于各会员国业已誓愿同联合国合作以促进对人权和基本自由的普遍尊重和遵行，
鉴于对这些权利和自由的普遍了解对于这个誓愿的充分实现具有很大的重要性，
因此现在，
大会，
发布这一世界人权宣言，作为所有人民和所有国家努力实现的共同标准，以期每一个人和社会机构经常铭念本宣言，努力通过教诲和教育促进对权利和自由的尊重，并通过国家的和国际的渐进措施，使这些权利和自由在各会员国本身人民及在其管辖下领土的人民中得到普遍和有效的承认和遵行；
"""
            ],
        )
    )
    # ... and committing
    writer.commit()

    # Reload the index to ensure it points to the last commit.
    index.reload()
    searcher = index.searcher()

    query = index.parse_query("邻", ["title", "body"])
    result = searcher.search(query, 2)
    assert len(result.hits) == 0

    query = index.parse_query("公共", ["title", "body"])
    result = searcher.search(query, 2)
    assert len(result.hits) == 1

    query = index.parse_query("人民", ["title", "body"])
    result = searcher.search(query, 2)
    assert len(result.hits) == 2
