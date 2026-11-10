//! The judgements reading. A judgements reading takes
//! a collection of recorded review inputs in the census
//! form and states, per judgement of CK-8, every verdict
//! pair of the collection standing on a flag of that
//! judgement together with the flag it stands on, so the
//! recorded verdicts of a collection stand gathered by
//! the judgement of the flag they stand on rather than
//! being read out of one set review at a time or picked
//! out of the verdicts reading by hand. It exists for
//! the same recorded inputs as the census, over the same
//! collection loading whole. The loader validates each
//! entry through the shared set entry validation and
//! each member through the change set loader, the flags
//! of each set come from the set review, the verdicts
//! standing on each flag come from that same review's
//! blocks, and the judgement of a flag is the judgement
//! that review states for it under CK-8, so the reading
//! holds no second flagging, judging, or outcome logic
//! of its own. The obligations this module carries are
//! CK-37 and CK-38 in CONTRACT.md, in the Round 18
//! section.

use crate::changeset::review_changeset;
use crate::json::JsonValue;
use crate::outcomes::JUDGEMENTS;
use crate::sets::validate_entry;
use crate::verdicts::{line_outcome, VerdictEntry};

/// One flag group of a judgements reading, holding the
/// name of the file the flag stands in, the line number
/// of the flag, the pattern of the flag, the label of
/// the set the flag stands in, and all the recorded
/// verdicts standing on the flag, in the order they
/// stand in the entry.
#[derive(Clone, Debug, PartialEq)]
pub struct JudgementFlagGroup {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
    pub verdicts: Vec<VerdictEntry>,
}

/// One section of a judgements reading, holding one
/// judgement of CK-8 and the flag groups of the
/// collection standing under that judgement, one group
/// for each flag of the collection standing under the
/// judgement and carrying at least one recorded
/// verdict, in collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct JudgementsSection {
    pub judgement: &'static str,
    pub groups: Vec<JudgementFlagGroup>,
}

/// A whole judgements reading, holding its sections in
/// the order CK-8 states the judgements and the counts
/// of the collection the builder counts, the sets read,
/// the flagged lines, and the verdicts read, a verdict
/// standing in its flag's section whole whatever its
/// choice, so the verdicts read count counts every
/// verdict the review's blocks carry.
#[derive(Clone, Debug, PartialEq)]
pub struct JudgementsReading {
    pub sections: Vec<JudgementsSection>,
    pub sets_read: usize,
    pub flags_read: usize,
    pub verdicts_read: usize,
}

/// Return the list of defects found in a judgements
/// file. Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_judgements(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["judgements must be a JSON object".to_string()];
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

/// Read a valid judgements file into its reading.
///
/// The flags of the collection are gathered in
/// collection order and then stand under the judgement
/// the one outcome helper of CK-8 states for them in
/// that order, the pairs of a flag standing whole in
/// the flag's one section, so one flag stands in
/// exactly one section, and every judgement stands a
/// section whether any verdict of the collection
/// stands under it or not. A flag carrying no verdicts
/// stands in no section. The counts stand over the
/// whole collection, the verdicts read count counting
/// every verdict the review's blocks carry. A set the
/// loader refused reads as the groups its members
/// still yield, in the posture the set readings hold,
/// and the command line never renders one.
pub fn judgements_reading_of(value: &JsonValue) -> JudgementsReading {
    let mut gathered: [Vec<JudgementFlagGroup>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut sets_read = 0usize;
    let mut flags_read = 0usize;
    let mut verdicts_read = 0usize;
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
                    for block in &result.blocks {
                        flags_read += 1;
                        if block.verdicts.is_empty() {
                            continue;
                        }
                        verdicts_read += block.verdicts.len();
                        let Some(index) = JUDGEMENTS
                            .iter()
                            .position(|judgement| *judgement == line_outcome(&block.verdicts))
                        else {
                            continue;
                        };
                        gathered[index].push(JudgementFlagGroup {
                            file: result.name.clone(),
                            line: block.flag.line,
                            pattern: block.flag.pattern.clone(),
                            set_label: label.clone(),
                            verdicts: block.verdicts.clone(),
                        });
                    }
                }
            }
        }
    }
    let sections = JUDGEMENTS
        .iter()
        .zip(gathered)
        .map(|(judgement, groups)| JudgementsSection { judgement, groups })
        .collect();
    JudgementsReading {
        sections,
        sets_read,
        flags_read,
        verdicts_read,
    }
}
