use std::sync::Arc;

use deval_data_model::{Annotated, AnnotatedData, Format};
use deval_validator::Validator;
use line_index::LineIndex;

pub mod token_store;
pub use token_store::TokenStore;

pub struct Document {
    pub annotated: Option<Annotated<AnnotatedData>>,
    pub line_index: LineIndex,
    pub token_store: TokenStore,
    format: Arc<dyn Format>,
    schema: Arc<dyn Validator>,
}

impl Document {
    pub fn new(text: &str, format: Arc<dyn Format>, schema: Arc<dyn Validator>) -> Self {
        let mut this = Self {
            line_index: LineIndex::new(""),
            annotated: None,
            token_store: TokenStore::new(),
            format,
            schema,
        };
        this.update_text(text);
        this
    }

    pub fn update_text(&mut self, text: &str) {
        self.line_index = LineIndex::new(text);
        let parsed = match self.format.parse(text, "") {
            Ok(v) => v,
            Err(_) => {
                self.annotated = None;
                return;
            }
        };
        let annotated = self.schema.validate(parsed).result;
        self.annotated = Some(annotated.clone());

        // Update the token store with the new annotated data
        self.token_store.build_from_annotated(&annotated);
    }
}
