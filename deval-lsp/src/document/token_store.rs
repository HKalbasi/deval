use deval_data_model::{Annotated, AnnotatedData, FullAnnotation, SemanticType};
use std::cmp::Ordering;

/// A token with its semantic information and span
#[derive(Debug, Clone)]
pub struct SemanticToken {
    pub start: usize,
    pub end: usize,
    pub token_type: SemanticType,
}

impl SemanticToken {
    pub fn new(start: usize, end: usize, token_type: SemanticType) -> Self {
        Self {
            start,
            end,
            token_type,
        }
    }

    /// Check if this token's span contains the given position
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }

    /// Check if this token's span is contained within the given range
    pub fn is_in_range(&self, start: usize, end: usize) -> bool {
        self.start >= start && self.end <= end
    }
}

impl PartialEq for SemanticToken {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for SemanticToken {}

impl PartialOrd for SemanticToken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemanticToken {
    fn cmp(&self, other: &Self) -> Ordering {
        // First by start position, then by end position (longest first)
        self.start
            .cmp(&other.start)
            .then_with(|| other.end.cmp(&self.end))
    }
}

/// A data structure for efficiently storing and retrieving semantic tokens
#[derive(Debug, Default)]
pub struct TokenStore {
    tokens: Vec<SemanticToken>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Build the token store from annotated data
    pub fn build_from_annotated(&mut self, annotated: &Annotated<AnnotatedData, FullAnnotation>) {
        self.tokens.clear();
        self.collect_tokens(annotated);
        self.tokens.sort();
    }

    /// Collect all tokens from annotated data
    fn collect_tokens(&mut self, annotated: &Annotated<AnnotatedData, FullAnnotation>) {
        annotated.value.walk(&mut |annotation: FullAnnotation| {
            for span in &annotation.span.0 {
                if let Some(token_type) = annotation.semantic_type {
                    self.tokens
                        .push(SemanticToken::new(span.start, span.end, token_type));
                }
            }
        });
    }

    /// Get all tokens whose span is within the given range, sorted by position
    pub fn tokens_in_range(&self, start: usize, end: usize) -> Vec<&SemanticToken> {
        self.tokens
            .iter()
            .filter(|token| token.is_in_range(start, end))
            .collect()
    }

    /// Get the smallest token that contains the given position
    pub fn smallest_token_containing(&self, pos: usize) -> Option<&SemanticToken> {
        // Binary search for the first token that starts at or after pos
        let idx = match self
            .tokens
            .binary_search_by(|token| token.start.cmp(&pos).then(std::cmp::Ordering::Greater))
        {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        // Check tokens before idx (that might contain pos)
        for i in (0..idx).rev() {
            let token = &self.tokens[i];
            if token.contains(pos) {
                // Since tokens are sorted, the first one we find is the smallest
                return Some(token);
            }
        }

        None
    }
}
