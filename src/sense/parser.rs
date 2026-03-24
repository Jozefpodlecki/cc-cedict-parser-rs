use crate::sense::{classifier::ClassifierParser, models::*, qualifier::QualifierExtractor, reference::{ExtractResult, ReferenceExtractor}, tag::TagExtractor};

pub struct SenseParser;

impl SenseParser {
    pub fn parse<'a>(senses_str: &'a str) -> (Vec<Sense<'a>>, Vec<Classifier<'a>>, Vec<Reference>) {
        let mut senses: Vec<Sense<'_>> = Vec::new();
        let mut classifiers: Vec<Classifier<'_>> = Vec::new();
        let mut references: Vec<Reference> = Vec::new();

        for sense in senses_str.split('/').filter(|d| !d.is_empty()) {
            let sense_str = sense.trim();
            let mut sense = Sense::default();

            for gloss in sense_str.split(';') {
                let gloss = gloss.trim();

                if let Some(classifier) = ClassifierParser::parse(gloss) {
                    classifiers.extend(classifier);
                    continue;
                }

                let (value, qualifier) = QualifierExtractor::extract(gloss);
                
                if qualifier.is_some() {
                    sense.qualifier = qualifier;
                }

                let (mut value, tags) = TagExtractor::extract(value);

                let result = ReferenceExtractor::extract(value);
                
                match result {
                    ExtractResult::None(original) => { value = original.into() },
                    ExtractResult::Reference { parsed, reference } => {
                        references.push(reference);

                        match parsed {
                            Some(parsed) => value = parsed,
                            None => continue,
                        }
                    },
                };

                if value.is_empty() {
                    println!("empty gloss ? sense: {senses_str}");
                    continue;
                }

                sense.glosses.push(value.into());
                sense.tags.extend(tags);
                
            }

            senses.push(sense);
        }

        (senses, classifiers, references)
    }
    

    //         

    //         if extracted_reference.is_some() {
    //             references.push(extracted_reference.unwrap());

    //             if let Some(value) = reference_value {
    //                 definitions.push(Definition {
    //                     value: value.to_string(),
    //                     tags: vec![],
    //                     qualifier: None,
    //                 });
    //             }

    //             continue;
    //         }

    //         for sense in raw.split(';') {
    //             let (without_qualifier, qualifier) = QualifierExtractor::extract(sense);
    //             let (value, tags) = TagExtractor::extract(without_qualifier);

    //             if value.is_empty() {
    //                 continue;
    //             }

    //             definitions.push(Definition {
    //                 value: value.to_string(),
    //                 tags: tags,
    //                 qualifier,
    //             });
    //         }
    //     }

    //     (definitions, classifiers, references)
    // }
}


mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn should_parse_sense_with_qualifier() -> Result<()> {
        let defs = "very large; huge; tremendous; gigantic/(coll.) very; extremely/";
        let (senses, classifiers, reference) = SenseParser::parse(defs);

        dbg!(&senses);

        assert!(classifiers.is_empty());
        assert_eq!(senses.len(), 2);

        assert_eq!(senses[0].glosses, vec!["very large", "huge", "tremendous", "gigantic"].into_iter().map(Into::into).collect::<Vec<_>>());
        assert!(senses[0].qualifier.is_none());
        assert!(senses[0].tags.is_empty());
        assert_eq!(senses[1].glosses, vec!["very", "extremely"].into_iter().map(Into::into).collect::<Vec<_>>());
        assert_eq!(senses[1].qualifier, Some("colloquially"));
        assert!(senses[1].tags.is_empty());
        assert!(senses[1].tags.is_empty());
        assert!(reference.is_empty());

        Ok(())
    }
    
}