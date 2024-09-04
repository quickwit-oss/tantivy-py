// # Custom collector example
//
// This example shows how you can implement your own
// collector. As an example, we will compute a collector
// that computes the standard deviation of a given fast field.
//
// Of course, you can have a look at the tantivy's built-in collectors
// such as the `CountCollector` for more examples.

use tantivy::collector::{Collector, SegmentCollector};
use tantivy::fastfield::Column;
use tantivy::{Score, SegmentReader};

#[derive(Default, Debug)]
pub struct Stats {
    // Vec of tuples: (document_id, frame_id, sentence_id, score)
    pub hits: Vec<(u64, u64, u64, f32)>,
}

impl Stats {}

pub struct StatsCollector {
    document_id_field_name: String,
    frame_id_field_name: String,
    sentence_id_field_name: String,
}

impl StatsCollector {
    pub fn new() -> StatsCollector {
        StatsCollector {
            document_id_field_name: "document_id__".to_string(),
            frame_id_field_name: "frame_id__".to_string(),
            sentence_id_field_name: "sentence_id__".to_string(),
        }
    }

    fn get_fast_field_reader(
        &self,
        field: &str,
        segment_reader: &SegmentReader,
    ) -> tantivy::Result<Column> {
        // Look up the correct `Field` instance from the string name
        let _f = segment_reader
            .schema()
            .get_field(field)
            .expect("Given field doesn't exist.");
        segment_reader.fast_fields().u64(field)
    }
}

impl Collector for StatsCollector {
    type Fruit = Option<Stats>;

    type Child = StatsSegmentCollector;

    fn for_segment(
        &self,
        _segment_local_id: u32,
        segment_reader: &SegmentReader,
    ) -> tantivy::Result<StatsSegmentCollector> {
        // Create fastfield readers for document_id and frame_id.
        let fast_field_reader_document_id = self.get_fast_field_reader(
            &self.document_id_field_name,
            segment_reader,
        )?;
        let fast_field_reader_frame_id = self
            .get_fast_field_reader(&self.frame_id_field_name, segment_reader)?;
        let fast_field_reader_sentence_id = self.get_fast_field_reader(
            &self.sentence_id_field_name,
            segment_reader,
        )?;
        Ok(StatsSegmentCollector {
            fast_field_reader_document_id,
            fast_field_reader_frame_id,
            fast_field_reader_sentence_id,
            stats: Stats::default(),
        })
    }

    fn requires_scoring(&self) -> bool {
        false
    }

    fn merge_fruits(
        &self,
        segment_stats: Vec<Option<Stats>>,
    ) -> tantivy::Result<Option<Stats>> {
        let mut stats = Stats::default();
        for segment_stats in segment_stats.into_iter().flatten() {
            stats.hits.extend(segment_stats.hits);
        }
        Ok(Some(stats))
    }
}

pub struct StatsSegmentCollector {
    fast_field_reader_document_id: Column,
    fast_field_reader_frame_id: Column,
    fast_field_reader_sentence_id: Column,
    stats: Stats,
}

impl SegmentCollector for StatsSegmentCollector {
    type Fruit = Option<Stats>;

    fn collect(&mut self, doc: u32, _score: Score) {
        let f = self.fast_field_reader_frame_id.values.get_val(doc);
        let d = self.fast_field_reader_document_id.values.get_val(doc);
        let s = self.fast_field_reader_sentence_id.values.get_val(doc);
        self.stats.hits.push((d, f, s, _score));
    }

    fn harvest(self) -> <Self as SegmentCollector>::Fruit {
        Some(self.stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use tantivy::collector::FilterCollector;
    use tantivy::query::QueryParser;
    use tantivy::schema::{Schema, FAST, INDEXED, STORED, TEXT};
    use tantivy::{doc, Index};

    #[test]
    fn test_collectors() -> tantivy::Result<()> {
        // # Defining the schema
        //
        // The Tantivy index requires a very strict schema.
        // The schema declares which fields are in the index,
        // and for each field, its type and "the way it should
        // be indexed".

        // first we need to define a schema ...
        let mut schema_builder = Schema::builder();

        // We'll assume a fictional index containing
        // products, and with a name, a description, and a price.
        let product_name = schema_builder.add_text_field("name", TEXT);
        let product_description =
            schema_builder.add_text_field("description", TEXT);
        let price = schema_builder.add_u64_field("price", INDEXED | FAST);
        let document_id = schema_builder
            .add_u64_field("document_id__", STORED | INDEXED | FAST);
        let frame_id =
            schema_builder.add_u64_field("frame_id__", STORED | INDEXED | FAST);
        let _sentence_id = schema_builder
            .add_u64_field("sentence_id__", STORED | INDEXED | FAST);
        let schema = schema_builder.build();

        // # Indexing documents
        //
        // Lets index a bunch of fake documents for the sake of
        // this example.
        let index = Index::create_in_ram(schema);

        let mut index_writer = index.writer(50_000_000)?;
        index_writer.add_document(doc!(
            product_name => "Super Broom 2000",
            product_description => "While it is ok for short distance travel, this broom \
            was designed quiditch. It will up your game.",
            price => 30_200u64,
            document_id => 1u64,
            frame_id => 1u64,
        ))?;
        index_writer.add_document(doc!(
            product_name => "Turbulobroom",
            product_description => "You might have heard of this broom before : it is the sponsor of the Wales team.\
                You'll enjoy its sharp turns, and rapid acceleration",
            price => 29_240u64,
            document_id => 1u64,
            frame_id => 2u64,
        ))?;
        index_writer.add_document(doc!(
            product_name => "Not relevant",
            product_description => "We don't care about floor-cleaning.",
            price => 29_240u64,
            document_id => 1u64,
            frame_id => 1u64,
        ))?;
        index_writer.add_document(doc!(
            product_name => "Betterbroom",
            product_description => "broom broom broom broom broom broom",
            price => 29_240u64,
            document_id => 1u64,
            frame_id => 1u64,
        ))?;
        index_writer.add_document(doc!(
            product_name => "Broomio",
            product_description => "Great value for the price. This broom is a market favorite. What a broom! Super brooms.",
            price => 21_240u64,
            document_id => 2u64,
            frame_id => 5u64,
        ))?;
        index_writer.add_document(doc!(
            product_name => "Whack a Mole",
            product_description => "Prime quality bat.",
            price => 5_200u64,
            document_id => 3u64,
            frame_id => 6u64,
        ))?;
        index_writer.commit()?;

        let reader = index.reader()?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(
            &index,
            vec![product_name, product_description],
        );

        // here we want to search for `broom` and use `StatsCollector` on the hits.
        let query = query_parser.parse_query("broom")?;

        let analysis_frame_ids = [1u64, 2u64, 5u64];
        // https://docs.rs/pyo3/latest/pyo3/types/struct.PySet.html
        let analysis_filter = BTreeSet::from(analysis_frame_ids);

        if let Some(stats) = searcher.search(
            &query,
            &FilterCollector::new(
                "frame_id__".to_string(),
                move |value: u64| analysis_filter.contains(&value),
                StatsCollector::new(),
            ),
        )? {
            println!("{stats:?}");
        }

        Ok(())
    }
}
