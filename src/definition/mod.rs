use crate::definition::{classifier::ClassifierParser, qualifier::QualifierExtractor, reference::ReferenceExtractor, tag::TagExtractor};

mod qualifier;
mod tag;
mod reference;
mod classifier;

#[derive(Debug)]
pub struct Definition<'a> {
    pub value: String,
    pub tags: Vec<&'a str>, 
    pub qualifier: Option<&'a str>,
}

#[derive(Debug)]
pub struct Reference<'a> {
    pub traditional: &'a str,
    pub simplified: Option<&'a str>,
    pub pinyin: Option<Vec<&'a str>>,
}

#[derive(Debug)]
pub struct Classifier<'a> {
    pub traditional: &'a str,
    pub simplified: Option<&'a str>,
    pub pinyin: &'a str,
}

pub struct DefinitionParser;

impl DefinitionParser {
    pub fn parse<'a>(defs: &'a str) -> (Vec<Definition<'a>>, Vec<Classifier<'a>>, Vec<Reference<'a>>) {
        let mut definitions = Vec::new();
        let mut classifiers = Vec::new();
        let mut references = Vec::new();

        for raw in defs.split('/').filter(|d| !d.is_empty()) {
            let raw = raw.trim();

            if let Some(classifier) = ClassifierParser::parse(raw) {
                classifiers.extend(classifier);
                continue;
            }

            let (without_reference, reference_value, extracted_reference) = ReferenceExtractor::extract(raw);

            if extracted_reference.is_some() {
                references.push(extracted_reference.unwrap());

                if let Some(value) = reference_value {
                    definitions.push(Definition {
                        value: value.to_string(),
                        tags: vec![],
                        qualifier: None,
                    });
                }

                continue;
            }

            for sense in raw.split(';') {
                let (without_qualifier, qualifier) = QualifierExtractor::extract(sense);
                let (value, tags) = TagExtractor::extract(without_qualifier);

                if value.is_empty() {
                    continue;
                }

                definitions.push(Definition {
                    value: value.to_string(),
                    tags: tags,
                    qualifier,
                });
            }
        }

        (definitions, classifiers, references)
    }
}


mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn should_parse_definitions_with_qualifier() -> Result<()> {
        let defs = "very large; huge; tremendous; gigantic/(coll.) very; extremely/";
        let (definitions, classifiers, reference) = DefinitionParser::parse(defs);

        assert!(classifiers.is_empty());
        assert_eq!(definitions.len(), 6);

        assert_eq!(definitions[0].value, "very large");
        assert_eq!(definitions[1].value, "huge");
        // assert!(definitions[0].qualifier.is_none());
        // assert!(definitions[0].tags.is_empty());

        // assert_eq!(
        //     definitions[1].value,
        //     "very; extremely"
        // );
        // assert_eq!(definitions[1].qualifier, Some("colloquially"));
        // assert!(definitions[1].tags.is_empty());

        Ok(())
    }
    
}