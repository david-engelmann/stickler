//! The grounds reading. A grounds reading takes a
//! collection of recorded review inputs in the census
//! form and states, per borderline pattern of the
//! checker, every verdict pair of the collection
//! standing on a flag of that pattern together with
//! the flag it stands on, so the recorded verdicts of
//! a collection stand gathered by the pattern of the
//! flag they stand on rather than being read out of
//! one set review at a time or picked out of the
//! verdicts reading by hand. It exists for the same
//! recorded inputs as the census, over the same
//! collection loading whole. The loader validates
//! each entry through the shared set entry validation
//! and each member through the change set loader, the
//! flags of each set come from the set review, the
//! verdicts standing on each flag come from that same
//! review's blocks, and the pattern of a flag is the
//! pattern the checker stated on the flag when it
//! raised it, so the reading holds no second flagging,
//! judging, or outcome logic of its own. The
//! obligations this module carries are CK-35 and
//! CK-36 in CONTRACT.md, in the Round 17 section.

use crate::changeset::review_changeset;
use crate::checker::BORDERLINE_PATTERNS;
use crate::json::JsonValue;
use crate::sets::validate_entry;
use crate::verdicts::VerdictEntry;

/// One flag group of a grounds reading, holding the
/// name of the file the flag stands in, the line number
/// of the flag, the pattern of the flag, the label of
/// the set the flag stands in, and all the recorded
/// verdicts standing on the flag, in the order they
/// stand in the entry.
#[derive(Clone, Debug, PartialEq)]
pub struct GroundFlagGroup {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
    pub verdicts: Vec<VerdictEntry>,
}

/// One section of a grounds reading, holding one
/// pattern name of the checker and the flag groups of
/// the collection standing under that pattern, one
/// group for each flag of the collection standing
/// under the pattern and carrying at least one
/// recorded verdict, in collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct GroundsSection {
    pub pattern: String,
    pub groups: Vec<GroundFlagGroup>,
}

/// A whole grounds reading, holding its sections in
/// the order CK-7 states the patterns and the counts
/// of the collection the builder counts, the sets read,
/// the flagged lines, and the verdicts read, a verdict
/// standing in its flag's section whole whatever its
/// choice, so the verdicts read count counts every
/// verdict the review's blocks carry.
#[derive(Clone, Debug, PartialEq)]
pub struct GroundsReading {
    pub sections: Vec<GroundsSection>,
    pub sets_read: usize,
    pub flags_read: usize,
    pub verdicts_read: usize,
}

/// Return the list of defects found in a grounds file.
/// Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_grounds(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["grounds must be a JSON object".to_string()];
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

/// Read a valid grounds file into its reading.
///
/// The flags of the collection are gathered in
/// collection order and then stand under the pattern
/// stated on the flag in that order, the pairs of a
/// flag standing whole in the flag's one section, so
/// one flag stands in exactly one section, and every
/// pattern stands a section whether any verdict of
/// the collection stands under it or not. A flag
/// carrying no verdicts stands in no section. The
/// counts stand over the whole collection, the
/// verdicts read count counting every verdict the
/// review's blocks carry. A set the loader refused
/// reads as the groups its members still yield, in
/// the posture the set readings hold, and the command
/// line never renders one.
pub fn grounds_reading_of(value: &JsonValue) -> GroundsReading {
    let mut gathered: [Vec<GroundFlagGroup>; 3] = [Vec::new(), Vec::new(), Vec::new()];
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
                        let Some(index) = BORDERLINE_PATTERNS
                            .iter()
                            .position(|pattern| *pattern == block.flag.pattern)
                        else {
                            continue;
                        };
                        gathered[index].push(GroundFlagGroup {
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
    let sections = BORDERLINE_PATTERNS
        .iter()
        .zip(gathered)
        .map(|(pattern, groups)| GroundsSection {
            pattern: pattern.to_string(),
            groups,
        })
        .collect();
    GroundsReading {
        sections,
        sets_read,
        flags_read,
        verdicts_read,
    }
}
