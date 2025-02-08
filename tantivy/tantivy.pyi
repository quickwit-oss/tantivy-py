import datetime
from enum import Enum
from typing import Any, Optional, Sequence, TypeVar, Union


class Schema:
    pass


class SchemaBuilder:
    @staticmethod
    def is_valid_field_name(name: str) -> bool:
        pass

    def add_text_field(
        self,
        name: str,
        stored: bool = False,
        tokenizer_name: str = "default",
        index_option: str = "position",
    ) -> SchemaBuilder:
        pass

    def add_integer_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
    ) -> SchemaBuilder:
        pass

    def add_float_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
    ) -> SchemaBuilder:
        pass

    def add_unsigned_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
    ) -> SchemaBuilder:
        pass

    def add_boolean_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
    ) -> SchemaBuilder:
        pass

    def add_date_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
    ) -> SchemaBuilder:
        pass

    def add_json_field(
        self,
        name: str,
        stored: bool = False,
        tokenizer_name: str = "default",
        index_option: str = "position",
    ) -> SchemaBuilder:
        pass

    def add_facet_field(
        self,
        name: str,
    ) -> SchemaBuilder:
        pass

    def add_bytes_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
        index_option: str = "position",
    ) -> SchemaBuilder:
        pass

    def add_ip_addr_field(
        self,
        name: str,
        stored: bool = False,
        indexed: bool = False,
        fast: bool = False,
    ) -> SchemaBuilder:
        pass

    def build(self) -> Schema:
        pass


class Facet:
    @staticmethod
    def from_encoded(encoded_bytes: bytes) -> Facet:
        pass

    @classmethod
    def root(cls) -> Facet:
        pass

    @classmethod
    def from_string(cls, facet_string: str) -> Facet:
        pass

    @property
    def is_root(self) -> bool:
        pass

    def is_prefix_of(self, other: Facet) -> bool:
        pass

    def to_path(self) -> list[str]:
        pass

    def to_path_str(self) -> str:
        pass


class Document:
    def __new__(cls, **kwargs) -> Document:
        pass

    def extend(self, py_dict: dict, schema: Optional[Schema]) -> None:
        pass

    @staticmethod
    def from_dict(py_dict: dict, schema: Optional[Schema] = None) -> Document:
        pass

    def to_dict(self) -> Any:
        pass

    def add_text(self, field_name: str, text: str) -> None:
        pass

    def add_unsigned(self, field_name: str, value: int) -> None:
        pass

    def add_integer(self, field_name: str, value: int) -> None:
        pass

    def add_float(self, field_name: str, value: float) -> None:
        pass

    def add_boolean(self, field_name: str, value: bool) -> None:
        pass

    def add_date(self, field_name: str, value: datetime.datetime) -> None:
        pass

    def add_facet(self, field_name: str, facet: Facet) -> None:
        pass

    def add_bytes(self, field_name: str, bytes: bytes) -> None:
        pass

    def add_json(self, field_name: str, value: Any) -> None:
        pass

    def add_ip_addr(self, field_name: str, ip_addr: str) -> None:
        pass

    @property
    def num_fields(self) -> int:
        pass

    @property
    def is_empty(self) -> bool:
        pass

    def get_first(self, field_name: str) -> Optional[Any]:
        pass

    def get_all(self, field_name: str) -> list[Any]:
        pass


class Occur(Enum):
    Must = 1
    Should = 2
    MustNot = 3


class FieldType(Enum):
    Text = 1
    Unsigned = 2
    Integer = 3
    Float = 4
    Boolean = 5
    Date = 6
    Facet = 7
    Bytes = 8
    Json = 9
    IpAddr = 10


_RangeType = TypeVar(
    "_RangeType", bound=int | float | datetime.datetime | bool | str | bytes
)


class Query:
    @staticmethod
    def term_query(
        schema: Schema,
        field_name: str,
        field_value: Any,
        index_option: str = "position",
    ) -> Query:
        pass

    @staticmethod
    def term_set_query(
        schema: Schema, field_name: str, field_values: Sequence[Any]
    ) -> Query:
        pass

    @staticmethod
    def all_query() -> Query:
        pass

    @staticmethod
    def fuzzy_term_query(
        schema: Schema,
        field_name: str,
        text: str,
        distance: int = 1,
        transposition_cost_one: bool = True,
        prefix=False,
    ) -> Query:
        pass

    @staticmethod
    def phrase_query(
        schema: Schema,
        field_name: str,
        words: list[Union[str, tuple[int, str]]],
        slop: int = 0,
    ) -> Query:
        pass

    @staticmethod
    def boolean_query(subqueries: Sequence[tuple[Occur, Query]]) -> Query:
        pass

    @staticmethod
    def disjunction_max_query(
        subqueries: Sequence[Query], tie_breaker: Optional[float] = None
    ) -> Query:
        pass

    @staticmethod
    def boost_query(query: Query, boost: float) -> Query:
        pass

    @staticmethod
    def regex_query(schema: Schema, field_name: str, regex_pattern: str) -> Query:
        pass

    @staticmethod
    def more_like_this_query(
        doc_address: DocAddress,
        min_doc_frequency: Optional[int] = 5,
        max_doc_frequency: Optional[int] = None,
        min_term_frequency: Optional[int] = 2,
        max_query_terms: Optional[int] = 25,
        min_word_length: Optional[int] = None,
        max_word_length: Optional[int] = None,
        boost_factor: Optional[float] = 1.0,
        stop_words: list[str] = [],
    ) -> Query:
        pass

    @staticmethod
    def const_score_query(query: Query, score: float) -> Query:
        pass

    @staticmethod
    def range_query(
        schema: Schema,
        field_name: str,
        field_type: FieldType,
        lower_bound: _RangeType,
        upper_bound: _RangeType,
        include_lower: bool = True,
        include_upper: bool = True,
    ) -> Query:
        pass


class Order(Enum):
    Asc = 1
    Desc = 2


class DocAddress:
    def __new__(cls, segment_ord: int, doc: int) -> DocAddress:
        pass

    @property
    def segment_ord(self) -> int:
        pass

    @property
    def doc(self) -> int:
        pass


class SearchResult:
    @property
    def hits(self) -> list[tuple[Any, DocAddress]]:
        pass


class Searcher:
    def search(
        self,
        query: Query,
        limit: int = 10,
        count: bool = True,
        order_by_field: Optional[str] = None,
        offset: int = 0,
        order: Order = Order.Desc,
    ) -> SearchResult:
        pass

    def aggregate(
        self,
        search_query: Query,
        agg_query: dict,
    ) -> dict:
        pass

    @property
    def num_docs(self) -> int:
        pass

    @property
    def num_segments(self) -> int:
        pass

    def doc(self, doc_address: DocAddress) -> Document:
        pass

    def doc_freq(self, field_name: str, field_value: Any) -> int:
        pass


class IndexWriter:
    def add_document(self, doc: Document) -> int:
        pass

    def add_json(self, json: str) -> int:
        pass

    def commit(self) -> int:
        pass

    def rollback(self) -> int:
        pass

    def garbage_collect_files(self) -> None:
        pass

    def delete_all_documents(self) -> None:
        pass

    @property
    def commit_opstamp(self) -> int:
        pass

    def delete_documents(self, field_name: str, field_value: Any) -> int:
        pass

    def wait_merging_threads(self) -> None:
        pass


class Index:
    def __new__(
        cls, schema: Schema, path: Optional[str] = None, reuse: bool = True
    ) -> Index:
        pass

    @staticmethod
    def open(path: str) -> Index:
        pass

    def writer(self, heap_size: int = 128_000_000, num_threads: int = 0) -> IndexWriter:
        pass

    def config_reader(
        self, reload_policy: str = "commit", num_warmers: int = 0
    ) -> None:
        pass

    def searcher(self) -> Searcher:
        pass

    @staticmethod
    def exists(path: str) -> bool:
        pass

    @property
    def schema(self) -> Schema:
        pass

    def reload(self) -> None:
        pass

    def parse_query(
        self, query: str, default_field_names: Optional[list[str]] = None
    ) -> Query:
        pass

    def parse_query_lenient(
        self, query: str, default_field_names: Optional[list[str]] = None
    ) -> Query:
        pass

    def register_tokenizer(
        self, name: str, text_analyzer: TextAnalyzer
    ) -> None: ...


class Range:
    @property
    def start(self) -> int:
        pass

    @property
    def end(self) -> int:
        pass


class Snippet:
    def to_html(self) -> str:
        pass

    def highlighted(self) -> list[Range]:
        pass

    def fragment(self) -> str:
        pass

class SnippetGenerator:
    @staticmethod
    def create(
        searcher: Searcher, query: Query, schema: Schema, field_name: str
    ) -> SnippetGenerator:
        pass

    def snippet_from_doc(self, doc: Document) -> Snippet:
        pass

    def set_max_num_chars(self, max_num_chars: int) -> None:
        pass


class Tokenizer:
    @staticmethod
    def raw() -> Tokenizer:
        pass

    @staticmethod
    def simple() -> Tokenizer:
        pass

    @staticmethod
    def whitespace() -> Tokenizer:
        pass

    @staticmethod
    def regex(pattern: str) -> Tokenizer:
        pass

    @staticmethod
    def ngram(
        min_gram: int = 2, max_gram: int = 3, prefix_only: bool = False
    ) -> Tokenizer:
        pass

    @staticmethod
    def facet() -> Tokenizer:
        pass


class Filter:

    @staticmethod
    def alphanum_only() -> Filter:
        pass

    @staticmethod
    def ascii_fold() -> Filter:
        pass

    @staticmethod
    def lowercase() -> Filter:
        pass

    @staticmethod
    def remove_long(length_limit: int) -> Filter:
        pass

    @staticmethod
    def stemmer(language: str) -> Filter:
        pass

    @staticmethod
    def stopword(language: str) -> Filter:
        pass

    @staticmethod
    def custom_stopword(stopwords: list[str]) -> Filter:
        pass

    @staticmethod
    def split_compound(constituent_words: list[str]) -> Filter:
        pass
    

class TextAnalyzer:

    def analyze(self, text: str) -> list[str]:
        pass


class TextAnalyzerBuilder:

    def __init__(self, tokenizer: Tokenizer):
        pass

    def filter(self, filter: Filter) -> TextAnalyzerBuilder:
        pass

    def build(self) -> TextAnalyzer:
        pass


__version__: str
