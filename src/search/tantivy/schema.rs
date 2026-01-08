use tantivy::Index;
use tantivy::schema::{IndexRecordOption, NumericOptions, STRING, Schema, TEXT, TextFieldIndexing};
use tantivy::tokenizer::{
    Language, LowerCaser, RawTokenizer, Stemmer, TextAnalyzer, Token, TokenFilter, TokenStream,
    Tokenizer,
};
use tantivy_jieba::JiebaTokenizer;

#[derive(Clone)]
struct AlnumTokenFilter;

struct AlnumTokenStream<T> {
    tail: T,
}

impl TokenFilter for AlnumTokenFilter {
    type Tokenizer<T: Tokenizer> = AlnumTokenFilterWrapper<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> Self::Tokenizer<T> {
        AlnumTokenFilterWrapper(tokenizer)
    }
}

#[derive(Clone)]
struct AlnumTokenFilterWrapper<T>(T);

impl<T: Tokenizer> Tokenizer for AlnumTokenFilterWrapper<T> {
    type TokenStream<'a> = AlnumTokenStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        AlnumTokenStream {
            tail: self.0.token_stream(text),
        }
    }
}

impl<T: TokenStream> TokenStream for AlnumTokenStream<T> {
    fn advance(&mut self) -> bool {
        while self.tail.advance() {
            if self
                .tail
                .token()
                .text
                .chars()
                .any(|c| c.is_alphanumeric())
            {
                return true;
            }
        }
        false
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}

pub(super) fn initialise_tokenizer(index: &Index) {
    let mut tokenizer = JiebaTokenizer::new();
    tokenizer.set_ordinal_position_mode(true);
    let analyzer = TextAnalyzer::builder(tokenizer)
        .filter(AlnumTokenFilter)
        .filter(LowerCaser)
        .filter(Stemmer::new(Language::English))
        .build();
    index.tokenizers().register("memvid_default", analyzer);
    index.tokenizers().register("raw", RawTokenizer::default());
}

pub(super) fn build_schema() -> Schema {
    let mut schema_builder = tantivy::schema::SchemaBuilder::default();

    let content_options = TextFieldIndexing::default()
        .set_tokenizer("memvid_default")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let content_field = TEXT.set_stored().set_indexing_options(content_options);
    schema_builder.add_text_field("content", content_field);

    let keyword_indexing = TextFieldIndexing::default()
        .set_tokenizer("memvid_default")
        .set_index_option(IndexRecordOption::Basic);
    let keyword_field = STRING
        .set_stored()
        .set_indexing_options(keyword_indexing.clone());
    schema_builder.add_text_field("tags", keyword_field.clone());
    schema_builder.add_text_field("labels", keyword_field.clone());
    schema_builder.add_text_field("track", keyword_field);

    let uri_indexing = TextFieldIndexing::default()
        .set_tokenizer("raw")
        .set_index_option(IndexRecordOption::Basic);
    let uri_field = STRING.set_stored().set_indexing_options(uri_indexing);
    schema_builder.add_text_field("uri", uri_field);

    let timestamp_options = NumericOptions::default()
        .set_indexed()
        .set_fast()
        .set_stored();
    schema_builder.add_i64_field("timestamp", timestamp_options);

    let frame_id_options = NumericOptions::default().set_indexed().set_stored();
    schema_builder.add_u64_field("frame_id", frame_id_options);

    schema_builder.build()
}
