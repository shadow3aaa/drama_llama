use llama_cpp_sys_4::llama_token;

use crate::Model;

/// A list of very common words for various languages. These can be used to
/// ignore certain tokens for the purposes of repetition detection, etc.
#[cfg_attr(
    feature = "serde",
    derive(rocket::serde::Deserialize, rocket::serde::Serialize)
)]
#[cfg_attr(feature = "serde", serde(crate = "rocket::serde"))]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum StopWords {
    // NOTE: If you add a new language here, add it to ALL and sort this list
    // and ALL in alphabetical order.
    // TODO: static assert all this.
    English,
}

impl StopWords {
    pub const ALL: [StopWords; 1] = [StopWords::English];

    pub const fn as_str(&self) -> &'static str {
        match self {
            StopWords::English => "English",
        }
    }

    pub const fn words(&self) -> &'static [&'static str] {
        match self {
            StopWords::English => ENGLISH,
        }
    }

    /// Tokenizes `self` using the given `model``.
    pub fn into_tokens(
        self,
        model: &Model,
    ) -> impl Iterator<Item = llama_token> + '_ {
        self.words()
            .iter()
            // TODO: there is allocation here that can be avoided by turning the
            // tokenize function into a method returning an iterator, however
            // it's not a big deal since this is only done once.
            .flat_map(|word| model.tokenize(word, false).into_iter())
    }
}

/// A list of common English stopwords from NLTK.
pub const ENGLISH: &[&str] = &[
    "a",
    "about",
    "above",
    "after",
    "again",
    "against",
    "all",
    "am",
    "an",
    "and",
    "any",
    "are",
    "as",
    "at",
    "be",
    "because",
    "been",
    "before",
    "being",
    "below",
    "between",
    "both",
    "but",
    "by",
    "can",
    "did",
    "do",
    "does",
    "doing",
    "don",
    "down",
    "during",
    "each",
    "few",
    "for",
    "from",
    "further",
    "had",
    "has",
    "have",
    "having",
    "he",
    "her",
    "here",
    "hers",
    "herself",
    "him",
    "himself",
    "his",
    "how",
    "i",
    "if",
    "in",
    "into",
    "is",
    "it",
    "its",
    "itself",
    "just",
    "me",
    "more",
    "most",
    "my",
    "myself",
    "no",
    "nor",
    "not",
    "now",
    "of",
    "off",
    "on",
    "once",
    "only",
    "or",
    "other",
    "our",
    "ours",
    "ourselves",
    "out",
    "over",
    "own",
    "s",
    "same",
    "she",
    "should",
    "so",
    "some",
    "such",
    "t",
    "than",
    "that",
    "the",
    "their",
    "theirs",
    "them",
    "themselves",
    "then",
    "there",
    "these",
    "they",
    "this",
    "those",
    "through",
    "to",
    "too",
    "under",
    "until",
    "up",
    "very",
    "was",
    "we",
    "were",
    "what",
    "when",
    "where",
    "which",
    "while",
    "who",
    "whom",
    "why",
    "will",
    "with",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
];
