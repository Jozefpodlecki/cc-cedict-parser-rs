#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A *sense* represents a single distinct meaning of a lexeme (dictionary entry).
///
/// A word (lexeme) may have multiple senses when it is polysemous.
/// Each sense groups together one or more glosses that describe the same meaning.
///
/// Glosses are alternative translations or paraphrases of that meaning (e.g. synonyms
/// or near-synonyms in the target language), not separate meanings.
///
/// Tags describe usage constraints or domain information (e.g. "literary", "Taiwanese"),
/// and a qualifier provides additional contextual restriction when present.
///
/// In contrast to glosses, senses are the unit used to distinguish different meanings
/// of the same lexeme.
///
/// Example:
/// - "to clamp down; to suppress" → one sense with two glosses
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct Sense<'a> {
    pub glosses: Vec<Box<str>>,  
    pub tags: Vec<&'a str>, 
    pub qualifier: Option<&'a str>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct OwnedSense {
    pub glosses: Vec<Box<str>>,  
    pub tags: Vec<Box<str>>, 
    pub qualifier: Option<Box<str>>,
}

impl<'a> Sense<'a> {
    pub fn to_owned(&self) -> OwnedSense {
        OwnedSense {
            glosses: self.glosses.clone(),
            tags: self.tags.iter().map(|t| (*t).into()).collect(),
            qualifier: self.qualifier.map(|q| q.into()),
        }
    }
}

/// The kind of relationship between a lexeme and another lexical item.
///
/// References in CC-CEDICT link an entry to another entry via a specific relation:
/// - Abbreviation: a shortened form of a full expression
/// - Variant: an alternate or literary/orthographic form of the same word
/// - AlsoWritten: another way the same word is written
/// - See: a pointer to a related entry for further information
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ReferenceKind {
    Abbreviation,
    Variant,
    AlsoWritten,
    See,
}

impl From<ReferenceKind> for i32 {
    fn from(value: ReferenceKind) -> Self {
        value as i32
    }
}

impl From<i32> for ReferenceKind {
    fn from(value: i32) -> Self {
        match value {
            0 => ReferenceKind::Abbreviation,
            1 => ReferenceKind::Variant,
            2 => ReferenceKind::AlsoWritten,
            3 => ReferenceKind::See,
            _ => unreachable!("Unhandled variant")
        }
    }
}
/// A reference to another lexical item in the dictionary.
///
/// A reference captures the headwords and pronunciation of a related entry.
/// It does not itself define meaning, but links to another entry that does.
///
/// References are typically derived from constructs such as:
/// - "abbr. for"
/// - "variant of"
/// - "also written"
/// - "see"
///
/// These indicate semantic or orthographic relationships between lexemes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Reference {
    pub kind: ReferenceKind,
    pub traditional: Box<str>,
    pub simplified: Option<Box<str>>,
    pub pinyin: Option<Vec<Box<str>>>,
}

/// A classifier (measure word) associated with the entry.
///
/// Classifiers are used in Chinese to quantify nouns and are typically required
/// when counting or specifying objects.
///
/// Each classifier includes:
/// - a traditional form,
/// - an optional simplified form,
/// - and its pinyin pronunciation.
///
/// Example:
/// - 本 (běn) as a classifier for books
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Classifier<'a> {
    pub traditional: &'a str,
    pub simplified: Option<&'a str>,
    pub pinyin: &'a str,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct OwnedClassifier {
    pub traditional: Box<str>,
    pub simplified: Option<Box<str>>,
    pub pinyin: Box<str>,
}

impl<'a> Classifier<'a> {
    pub fn to_owned(&self) -> OwnedClassifier {
        OwnedClassifier {
            traditional: self.traditional.into(),
            simplified: self.simplified.map(Into::into),
            pinyin: self.pinyin.into(),
        }
    }
}
