import tantivy
import pytest


class TestKapicheTokenizers:
    """Tests for Kapiche tokenizers exposed to Python"""

    def test_kapiche_tokenizer_exists(self):
        """Test that kapiche_tokenizer() function exists and returns TextAnalyzer"""
        analyzer = tantivy.kapiche_tokenizer()
        assert analyzer is not None
        assert type(analyzer).__name__ == 'TextAnalyzer'

    def test_kapiche_tokenizer_lower_exists(self):
        """Test that kapiche_tokenizer_lower() function exists and returns TextAnalyzer"""
        analyzer = tantivy.kapiche_tokenizer_lower()
        assert analyzer is not None
        assert type(analyzer).__name__ == 'TextAnalyzer'

    def test_kapiche_tokenizer_analyzes_text(self):
        """Test that kapiche_tokenizer can analyze text"""
        analyzer = tantivy.kapiche_tokenizer()
        tokens = analyzer.analyze("hello world")
        assert isinstance(tokens, list)
        assert len(tokens) > 0

    def test_kapiche_tokenizer_lower_lowercases(self):
        """Test that kapiche_tokenizer_lower lowercases tokens"""
        analyzer = tantivy.kapiche_tokenizer_lower()
        tokens = analyzer.analyze("Hello World HELLO")
        assert tokens == ['hello', 'world', 'hello']


class TestCountTokens:
    """Tests for count_tokens() method with unique parameter"""

    def test_count_tokens_basic(self):
        """Test basic token counting without unique parameter"""
        analyzer = tantivy.kapiche_tokenizer()
        text = "hello world hello"
        count = analyzer.count_tokens(text)
        assert count == 3

    def test_count_tokens_unique_false(self):
        """Test count_tokens with unique=False (explicit)"""
        analyzer = tantivy.kapiche_tokenizer()
        text = "hello world hello"
        count = analyzer.count_tokens(text, unique=False)
        assert count == 3

    def test_count_tokens_unique_true(self):
        """Test count_tokens with unique=True"""
        analyzer = tantivy.kapiche_tokenizer()
        text = "hello world hello"
        unique_count = analyzer.count_tokens(text, unique=True)
        assert unique_count == 2

    def test_count_tokens_matches_analyze_total(self):
        """Test that count_tokens(unique=False) matches len(analyze())"""
        analyzer = tantivy.kapiche_tokenizer()
        text = "the quick brown fox jumps over the lazy dog"
        tokens = analyzer.analyze(text)
        count = analyzer.count_tokens(text, unique=False)
        assert count == len(tokens)

    def test_count_tokens_matches_analyze_unique(self):
        """Test that count_tokens(unique=True) matches len(set(analyze()))"""
        analyzer = tantivy.kapiche_tokenizer()
        text = "the quick brown fox jumps over the lazy dog"
        tokens = analyzer.analyze(text)
        unique_count = analyzer.count_tokens(text, unique=True)
        assert unique_count == len(set(tokens))

    def test_count_tokens_with_duplicates(self):
        """Test counting with many duplicates"""
        analyzer = tantivy.kapiche_tokenizer_lower()
        text = "apple banana apple cherry banana apple"
        total_count = analyzer.count_tokens(text, unique=False)
        unique_count = analyzer.count_tokens(text, unique=True)
        assert total_count == 6
        assert unique_count == 3

    def test_count_tokens_empty_string(self):
        """Test counting tokens in empty string"""
        analyzer = tantivy.kapiche_tokenizer()
        text = ""
        total_count = analyzer.count_tokens(text, unique=False)
        unique_count = analyzer.count_tokens(text, unique=True)
        assert total_count == 0
        assert unique_count == 0

    def test_count_tokens_single_token(self):
        """Test counting with single token"""
        analyzer = tantivy.kapiche_tokenizer()
        text = "hello"
        total_count = analyzer.count_tokens(text, unique=False)
        unique_count = analyzer.count_tokens(text, unique=True)
        assert total_count == 1
        assert unique_count == 1

    def test_count_tokens_case_sensitivity_with_lower(self):
        """Test that lowercase analyzer treats different cases as same token"""
        analyzer = tantivy.kapiche_tokenizer_lower()
        text = "Hello hello HELLO"
        total_count = analyzer.count_tokens(text, unique=False)
        unique_count = analyzer.count_tokens(text, unique=True)
        assert total_count == 3
        assert unique_count == 1  # All should be lowercased to 'hello'

    def test_count_tokens_large_text(self):
        """Test counting tokens in larger text"""
        analyzer = tantivy.kapiche_tokenizer_lower()
        text = """
        The quick brown fox jumps over the lazy dog.
        The dog was sleeping under a tree.
        The fox was very quick and clever.
        """
        # First verify against analyze() to get expected values
        tokens = analyzer.analyze(text)
        expected_total = len(tokens)
        expected_unique = len(set(tokens))

        unique_count = analyzer.count_tokens(text, unique=True)
        total_count = analyzer.count_tokens(text, unique=False)

        # Verify exact counts match analyze()
        assert total_count == expected_total
        assert unique_count == expected_unique
        # Sanity check: unique should be less than total
        assert unique_count < total_count


class TestKapicheTokenizerWithStopwords:
    """Tests for kapiche_tokenizer_lower_with_stopwords()"""

    def test_kapiche_tokenizer_lower_with_stopwords_english(self):
        """Test that stopwords are removed using Kapiche custom English list"""
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        tokens = analyzer.analyze("The quick brown fox")
        assert tokens == ["quick", "brown", "fox"]

    def test_count_tokens_with_stopwords(self):
        """Test count_tokens excludes stopwords"""
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        # "the", "and", and "a" are stopwords
        count = analyzer.count_tokens("the quick brown fox and a dog", unique=True)
        # Tokens: quick, brown, fox, dog = 4 ("the", "and", "a" are stopwords)
        assert count == 4

    def test_stopwords_with_punctuation_and_possessives(self):
        """Test that filter order works correctly"""
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        tokens = analyzer.analyze("John's the best!")
        # "John's" -> "john" (lowercased, possessive removed)
        # "the" -> removed (stopword)
        # "best!" -> "best" (punctuation removed)
        assert tokens == ["john", "best"]

    def test_case_insensitive_stopword_removal(self):
        """Test that uppercase stopwords are removed after lowercasing"""
        analyzer = tantivy.kapiche_tokenizer_lower_with_stopwords()
        tokens = analyzer.analyze("THE QUICK")
        # "THE" lowercased to "the", then removed as stopword
        assert tokens == ["quick"]


class TestUsagePattern:
    """Test the actual usage pattern mentioned in the requirements"""

    def test_usage_pattern(self):
        """Test the usage pattern: analyzer reuse with unique counting"""
        # Get the analyzer once
        analyzer = tantivy.kapiche_tokenizer_lower()

        # Fast unique token count for unstructured text
        unstructured_text = "The quick brown fox jumps over the lazy dog. The dog was sleeping."
        token_count = analyzer.count_tokens(unstructured_text, unique=True)

        # For sentence
        sentence_text = "The quick brown fox jumps over the lazy dog."
        sentence_token_count = analyzer.count_tokens(sentence_text, unique=True)

        # Verify exact counts by comparing with analyze()
        unstructured_tokens = analyzer.analyze(unstructured_text)
        sentence_tokens = analyzer.analyze(sentence_text)

        assert token_count == len(set(unstructured_tokens))
        assert sentence_token_count == len(set(sentence_tokens))

        # Unstructured has "sleeping" and "was" that sentence doesn't have
        # So unique count should be higher
        assert token_count == 10  # the, quick, brown, fox, jumps, over, lazy, dog, was, sleeping
        assert sentence_token_count == 8  # the, quick, brown, fox, jumps, over, lazy, dog
