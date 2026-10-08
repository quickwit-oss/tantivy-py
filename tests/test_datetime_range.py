import datetime
import zoneinfo

import pytest

import tantivy

ONE_MICROSECOND = datetime.timedelta(microseconds=1)
UTC = datetime.timezone.utc
PLUS_FIVE = datetime.timezone(datetime.timedelta(hours=5))
MINUS_FIVE = datetime.timezone(datetime.timedelta(hours=-5))

SCHEMA = tantivy.SchemaBuilder().add_date_field("d", stored=True).build()


def add_date(value):
    tantivy.Document().add_date("d", value)


def from_dict(value):
    tantivy.Document.from_dict({"d": value}, SCHEMA)


# Every conversion path that should report the same errors.
convert = pytest.mark.parametrize(
    "convert", [add_date, from_dict], ids=["add_date", "from_dict"]
)


def test_bounds_match_measured_range():
    assert tantivy.MIN_DATETIME == datetime.datetime(
        1677, 9, 21, 0, 12, 43, 145225, tzinfo=UTC
    )
    assert tantivy.MAX_DATETIME == datetime.datetime(
        2262, 4, 11, 23, 47, 16, 854775, tzinfo=UTC
    )
    for bound in (tantivy.MIN_DATETIME, tantivy.MAX_DATETIME):
        assert bound.utcoffset() == datetime.timedelta(0)


@pytest.mark.parametrize("bound", ["MIN_DATETIME", "MAX_DATETIME"])
def test_add_date_accepts_bounds(bound):
    doc = tantivy.Document()
    doc.add_date("d", getattr(tantivy, bound))
    assert doc.get_first("d") == getattr(tantivy, bound)


@pytest.mark.parametrize("bound", ["MIN_DATETIME", "MAX_DATETIME"])
def test_bounds_accepted_at_non_utc_offset(bound):
    utc_bound = getattr(tantivy, bound)
    doc = tantivy.Document()
    doc.add_date("d", utc_bound.astimezone(PLUS_FIVE))
    assert doc.get_first("d") == utc_bound


def test_zoneinfo_bounds():
    try:
        tz = zoneinfo.ZoneInfo("Asia/Kolkata")
    except zoneinfo.ZoneInfoNotFoundError:
        pytest.skip("tz database not available")
    doc = tantivy.Document()
    doc.add_date("d", tantivy.MAX_DATETIME.astimezone(tz))
    assert doc.get_first("d") == tantivy.MAX_DATETIME
    with pytest.raises(ValueError, match="out of range"):
        add_date((tantivy.MAX_DATETIME + ONE_MICROSECOND).astimezone(tz))


@convert
@pytest.mark.parametrize(
    "value",
    [
        tantivy.MIN_DATETIME - ONE_MICROSECOND,
        tantivy.MAX_DATETIME + ONE_MICROSECOND,
        (tantivy.MIN_DATETIME - ONE_MICROSECOND).astimezone(PLUS_FIVE),
        (tantivy.MAX_DATETIME + ONE_MICROSECOND).astimezone(PLUS_FIVE),
        datetime.datetime(1, 1, 1, tzinfo=UTC),
        datetime.datetime(9999, 12, 31, 23, 59, 59),
    ],
)
def test_out_of_range_message(convert, value):
    with pytest.raises(ValueError) as excinfo:
        convert(value)
    message = str(excinfo.value)
    assert value.isoformat() in message
    assert tantivy.MIN_DATETIME.isoformat() in message
    assert tantivy.MAX_DATETIME.isoformat() in message
    assert "Expected DateTime" not in message
    assert "ValueError:" not in message


@convert
@pytest.mark.parametrize(
    "value",
    [
        datetime.datetime(1, 1, 1, tzinfo=PLUS_FIVE),
        datetime.datetime(9999, 12, 31, 23, tzinfo=MINUS_FIVE),
    ],
)
def test_out_of_range_via_offset_conversion(convert, value):
    # astimezone() itself overflows for these; the user still gets the range
    # and the original error stays attached as the cause.
    with pytest.raises(ValueError) as excinfo:
        convert(value)
    message = str(excinfo.value)
    assert "out of range" in message
    assert tantivy.MIN_DATETIME.isoformat() in message
    assert "Expected DateTime" not in message
    assert isinstance(excinfo.value.__cause__, OverflowError)


def test_from_dict_names_the_field():
    with pytest.raises(ValueError, match="field d: datetime"):
        from_dict(datetime.datetime(1, 1, 1, tzinfo=UTC))


def test_from_dict_wrong_type_keeps_type_message():
    with pytest.raises(ValueError, match="Expected DateTime type"):
        from_dict("2020-01-01")


class _BadOffset(datetime.tzinfo):
    def utcoffset(self, dt):
        return datetime.timedelta(days=2)

    def dst(self, dt):
        return None


@convert
def test_unrelated_value_error_is_not_a_range_error(convert):
    with pytest.raises(ValueError) as excinfo:
        convert(datetime.datetime(2020, 1, 1, tzinfo=_BadOffset()))
    assert "out of range" not in str(excinfo.value)
    assert "Expected DateTime" not in str(excinfo.value)


def test_from_dict_list_with_out_of_range_item():
    good = datetime.datetime(2020, 1, 1, tzinfo=UTC)
    bad = datetime.datetime(1, 1, 1, tzinfo=UTC)
    with pytest.raises(ValueError, match="out of range") as excinfo:
        from_dict([good, bad])
    assert bad.isoformat() in str(excinfo.value)


def test_term_query_out_of_range():
    bad = datetime.datetime(1, 1, 1, tzinfo=UTC)
    with pytest.raises(ValueError, match="out of range") as excinfo:
        tantivy.Query.term_query(SCHEMA, "d", bad)
    assert tantivy.MIN_DATETIME.isoformat() in str(excinfo.value)


def test_bounds_round_trip_through_index():
    # Stored values keep full microsecond precision even though the indexed
    # (searchable) value is truncated to seconds by default.
    index = tantivy.Index(SCHEMA)
    writer = index.writer(heap_size=15_000_000, num_threads=1)
    bounds = (tantivy.MIN_DATETIME, tantivy.MAX_DATETIME)
    for bound in bounds:
        doc = tantivy.Document()
        doc.add_date("d", bound)
        writer.add_document(doc)
    writer.commit()
    index.reload()
    searcher = index.searcher()
    hits = searcher.search(tantivy.Query.all_query(), 10).hits
    stored = {searcher.doc(address).get_first("d") for _, address in hits}
    assert stored == set(bounds)
