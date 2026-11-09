//! The outcomes reading. An outcomes reading takes a
//! collection of recorded review inputs in the census
//! form and states, per judgement of CK-8, every flag of
//! the collection standing under that judgement, so which
//! flags stand judged stickler, which stand judged clean,
//! and which stand unjudged stands as one list rather than
//! being summed out of the census tally or read out of one
//! set review at a time. It exists for the same recorded
//! inputs as the census, contested, pattern, files, and
//! borderline readings, over the same collection loading
//! whole into all six. The loader validates each entry
//! through the shared set entry validation and each member
//! through the change set loader, the flags come from the
//! set review, and the judgement of each flag comes from
//! the one outcome helper of CK-8, so the reading holds no
//! second flagging or judging logic of its own. The
//! obligations this module carries are CK-25 and CK-26 in
//! CONTRACT.md, in the Round 12 section.

use crate::changeset::{review_changeset, FileResult};
use crate::json::JsonValue;
use crate::sets::validate_entry;
use crate::verdicts::line_outcome;

/// The judgements of CK-8 in the order CK-8 states them,
/// the order the sections of the reading stand in.
pub const JUDGEMENTS: [&str; 3] = ["stickler", "clean", "unjudged"];

/// One flag of an outcomes reading, holding the name of
/// the file the flag stands in, the line number of the
/// flag, the pattern of the flag, and the label of the
/// set the flag stands in.
#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeFlag {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
}

/// One section of an outcomes reading, holding one
/// judgement of CK-8 and the flags of the collection
/// standing under it in collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct OutcomeSection {
    pub judgement: &'static str,
    pub flags: Vec<OutcomeFlag>,
}

/// A whole outcomes reading, holding its three sections
/// in the order CK-8 states the judgements and the count
/// of the sets of the collection.
#[derive(Clone, Debug, PartialEq)]
pub struct OutcomesReading {
    pub sections: Vec<OutcomeSection>,
    pub sets_read: usize,
}

/// Gather the flags of one set from its set review
/// results, in the order the files stand in the review
/// and the flags stand in each file, each holding the
/// judgement the one outcome helper of CK-8 reads over
/// the verdicts standing on its flag.
fn flags_of(results: &[FileResult], set_label: &str) -> Vec<(&'static str, OutcomeFlag)> {
    let mut gathered = Vec::new();
    for result in results {
        for block in &result.blocks {
            gathered.push((
                line_outcome(&block.verdicts),
                OutcomeFlag {
                    file: result.name.clone(),
                    line: block.flag.line,
                    pattern: block.flag.pattern.clone(),
                    set_label: set_label.to_string(),
                },
            ));
        }
    }
    gathered
}

/// Return the list of defects found in an outcomes file.
/// Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_outcomes(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["outcomes must be a JSON object".to_string()];
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

/// Read a valid outcomes file into its reading.
///
/// The flags of the collection are gathered in
/// collection order and then stand under their
/// judgement in that order, so a flag stands in exactly
/// one section, the section of its judgement, and every
/// judgement stands a section whether any flag stands
/// under it or not. A set the loader refused reads as
/// the flags its members still yield, in the posture
/// the set readings hold, and the command line never
/// renders one.
pub fn outcomes_reading_of(value: &JsonValue) -> OutcomesReading {
    let mut gathered: Vec<(&'static str, OutcomeFlag)> = Vec::new();
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
                gathered.extend(flags_of(&review_changeset(member), &label));
            }
        }
    }
    let sections = JUDGEMENTS
        .iter()
        .map(|judgement| OutcomeSection {
            judgement,
            flags: gathered
                .iter()
                .filter(|(held, _)| held == judgement)
                .map(|(_, flag)| flag.clone())
                .collect(),
        })
        .collect();
    OutcomesReading {
        sections,
        sets_read,
    }
}
