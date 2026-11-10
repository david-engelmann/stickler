//! The categories reading. A categories reading takes
//! a collection of recorded review inputs in the census
//! form and states, per category of the checker, every
//! judgement of the collection standing under that
//! category, so the judgements of a collection stand
//! grouped by the category their objections carry
//! rather than being read out of one set review at a
//! time or summed out of the census tallies. It exists
//! for the same recorded inputs as the census, over the
//! same collection loading whole. The loader validates
//! each entry through the shared set entry validation
//! and each member through the change set loader, the
//! judgements of each set come from the set review,
//! and the standing number of each judgement counts
//! over the judgements of its file in the order the
//! set review heads them, so the reading holds no
//! second checking, flagging, or judging logic of
//! its own. The obligations this module carries are
//! CK-33 and CK-34 in CONTRACT.md, in the Round 16
//! section.

use crate::changeset::review_changeset;
use crate::json::JsonValue;
use crate::sets::validate_entry;

/// The category words of the review form, in the order
/// the formats section states the vocabulary, naming,
/// structure, and smell. The words stand as a constant
/// of this module because the checker states no
/// constant for them, as CK-34 records.
const CATEGORIES: [&str; 3] = ["naming", "structure", "smell"];

/// One judgement line of a categories reading, holding
/// the name of the file the judgement stands in, the
/// standing number of the judgement among the
/// judgements of its file, the shape name of the
/// judgement, and the label of the set the judgement
/// stands in.
#[derive(Clone, Debug, PartialEq)]
pub struct CategoryJudgement {
    pub file: String,
    pub number: usize,
    pub shape: String,
    pub set_label: String,
}

/// One section of a categories reading, holding one
/// category name and the judgement lines of the
/// collection standing under that category, in
/// collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct CategoriesSection {
    pub category: String,
    pub judgements: Vec<CategoryJudgement>,
}

/// A whole categories reading, holding its sections in
/// the order the formats section states the category
/// vocabulary and the count of the sets of the
/// collection.
#[derive(Clone, Debug, PartialEq)]
pub struct CategoriesReading {
    pub sections: Vec<CategoriesSection>,
    pub sets_read: usize,
}

/// Return the list of defects found in a categories
/// file. Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_categories(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["categories must be a JSON object".to_string()];
    }
    let Some(sets) = value.get("sets") else {
        return vec!["sets is missing".to_string()];
    };
    let Some(entries) = sets.as_array() else {
        return vec!["sets must be a list".to_string()];
    };
    for (index, entry) in entries.iter().enumerate() {
        validate_entry(entry, index, &mut defects);
    }
    defects
}

/// Read a valid categories file into its reading.
///
/// The judgements of the collection are gathered in
/// collection order and then stand under their
/// category in that order, so a judgement stands in
/// exactly one section, the section of the category
/// its objection carries, and every category stands
/// a section whether any judgement stands under it
/// or not. A set the loader refused reads as the
/// judgements its members still yield, in the posture
/// the set readings hold, and the command line never
/// renders one.
pub fn categories_reading_of(value: &JsonValue) -> CategoriesReading {
    let mut gathered: Vec<(String, CategoryJudgement)> = Vec::new();
    let mut sets_read = 0usize;
    if let Some(entries) = value.get("sets").and_then(|sets| sets.as_array()) {
        for entry in entries {
            sets_read += 1;
            let label = entry
                .get("label")
                .and_then(|label| label.as_jstring())
                .and_then(|text| text.to_rust_string())
                .unwrap_or_default();
            if let Some(member) = entry.get("change_set") {
                for result in review_changeset(member) {
                    for (index, (shape, objection)) in result.judgements.iter().enumerate() {
                        gathered.push((
                            objection.category.clone(),
                            CategoryJudgement {
                                file: result.name.clone(),
                                number: index + 1,
                                shape: shape.to_string(),
                                set_label: label.clone(),
                            },
                        ));
                    }
                }
            }
        }
    }
    let sections = CATEGORIES
        .iter()
        .map(|category| CategoriesSection {
            category: category.to_string(),
            judgements: gathered
                .iter()
                .filter(|(held, _)| held == category)
                .map(|(_, judgement)| judgement.clone())
                .collect(),
        })
        .collect();
    CategoriesReading {
        sections,
        sets_read,
    }
}
