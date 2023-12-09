class Schema:
    pass

class SchemaBuilder:
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