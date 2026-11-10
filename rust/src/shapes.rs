//! The shapes reading. A shapes reading takes a
//! collection of recorded review inputs in the census
//! form and states, per shape of the checker, every
//! judgement of the collection standing under that
//! shape, so the judgements of a collection stand
//! grouped by the shape that raised them rather than
//! being read out of one set review at a time or
//! summed out of the census tallies. It exists for the
//! same recorded inputs as the census, over the same
//! collection loading whole. The loader validates each
//! entry
//! through the shared set entry validation and each
//! member through the change set loader, the
//! judgements of each set come from the set review,
//! and the standing number of each judgement counts
//! over the judgements of its file in the order the
//! set review heads them, so the reading holds no
//! second checking, flagging, or judging logic of
//! its own. The obligations this module carries are
//! CK-29 and CK-30 in CONTRACT.md, in the Round 14
//! section.

use crate::changeset::review_changeset;
use crate::checker::SHAPES;
use crate::json::JsonValue;
use crate::sets::validate_entry;

/// One judgement line of a shapes reading, holding the
/// name of the file the judgement stands in, the
/// standing number of the judgement among the
/// judgements of its file, the category of the
/// judgement, and the label of the set the judgement
/// stands in.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeJudgement {
    pub file: String,
    pub number: usize,
    pub category: String,
    pub set_label: String,
}

/// One section of a shapes reading, holding one shape
/// name of the checker and the judgement lines of the
/// collection standing under that shape, in collection
/// order.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapesSection {
    pub shape: String,
    pub judgements: Vec<ShapeJudgement>,
}

/// A whole shapes reading, holding its sections in the
/// order the shapes are stated across CK-3 and CK-5
/// and the count of the sets of the collection.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapesReading {
    pub sections: Vec<ShapesSection>,
    pub sets_read: usize,
}

/// Return the list of defects found in a shapes file.
/// Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_shapes(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["shapes must be a JSON object".to_string()];
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

/// Read a valid shapes file into its reading.
///
/// The judgements of the collection are gathered in
/// collection order and then stand under their shape
/// in that order, so a judgement stands in exactly
/// one section, the section of its shape, and every
/// shape stands a section whether any judgement
/// stands under it or not. A set the loader refused
/// reads as the judgements its members still yield,
/// in the posture the set readings hold, and the
/// command line never renders one.
pub fn shapes_reading_of(value: &JsonValue) -> ShapesReading {
    let mut gathered: Vec<(&'static str, ShapeJudgement)> = Vec::new();
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
                            *shape,
                            ShapeJudgement {
                                file: result.name.clone(),
                                number: index + 1,
                                category: objection.category.clone(),
                                set_label: label.clone(),
                            },
                        ));
                    }
                }
            }
        }
    }
    let sections = SHAPES
        .iter()
        .map(|shape| ShapesSection {
            shape: shape.to_string(),
            judgements: gathered
                .iter()
                .filter(|(held, _)| held == shape)
                .map(|(_, judgement)| judgement.clone())
                .collect(),
        })
        .collect();
    ShapesReading {
        sections,
        sets_read,
    }
}
