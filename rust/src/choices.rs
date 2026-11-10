//! The choices reading. A choices reading takes a
//! collection of recorded review inputs in the census
//! form and states, per outcome of the seat, every
//! verdict pair of the collection standing under that
//! outcome together with the flag it stands on, so the
//! recorded verdicts of a collection stand gathered by
//! the choice they record rather than being read out
//! of one set review at a time or summed out of the
//! verdicts reading's closing counts. It exists for
//! the same recorded inputs as the census, over the
//! same collection loading whole. The loader validates
//! each entry through the shared set entry validation
//! and each member through the change set loader, the
//! flags of each set come from the set review, the
//! verdicts standing on each flag come from that same
//! review's blocks, and each verdict stands under the
//! outcome the abstention predicate of CK-7 and its
//! choice together name, so the reading holds no
//! second flagging, judging, or outcome logic of its
//! own. The obligations this module carries are CK-31
//! and CK-32 in CONTRACT.md, in the Round 15 section.

use crate::changeset::review_changeset;
use crate::json::JsonValue;
use crate::sets::validate_entry;
use crate::verdicts::{is_abstention, VerdictEntry};

/// The outcome words of the seat, in the order CK-28
/// states the outcome counts, the words the sections
/// of a choices reading stand under.
const OUTCOMES: [&str; 3] = ["stickler", "clean", "abstained"];

/// One flag group of a choices reading, holding the
/// name of the file the flag stands in, the line number
/// of the flag, the pattern of the flag, the label of
/// the set the flag stands in, and the recorded
/// verdicts standing on the flag that stand under the
/// outcome of the section holding the group, in the
/// order they stand in the entry.
#[derive(Clone, Debug, PartialEq)]
pub struct ChoiceFlagGroup {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
    pub verdicts: Vec<VerdictEntry>,
}

/// One section of a choices reading, holding one
/// outcome word of the seat and the flag groups of
/// the collection standing under that outcome, one
/// group for each flag of the collection carrying at
/// least one verdict standing under it, in collection
/// order.
#[derive(Clone, Debug, PartialEq)]
pub struct ChoicesSection {
    pub outcome: String,
    pub groups: Vec<ChoiceFlagGroup>,
}

/// A whole choices reading, holding its sections in
/// the order CK-28 states the outcome counts and the
/// counts of the collection the builder counts, the
/// sets read, the flagged lines, the flags carrying at
/// least one recorded verdict, and the verdicts read,
/// a verdict standing in no section standing in the
/// verdicts read count and in none of the three
/// outcome counts, in the posture CK-32 states for a
/// choice outside the two labels.
#[derive(Clone, Debug, PartialEq)]
pub struct ChoicesReading {
    pub sections: Vec<ChoicesSection>,
    pub sets_read: usize,
    pub flags_read: usize,
    pub flags_with_verdicts: usize,
    pub verdicts_read: usize,
}

/// The index of the section a verdict stands under,
/// under the rule FIX-CK-28 states. The abstained
/// section takes the verdict exactly where the
/// abstention predicate of CK-7 holds for it, and a
/// label section takes it where its choice stands as
/// that label and the predicate does not hold, so no
/// verdict is seated by its choice word alone. A
/// verdict whose choice stands outside the two labels
/// stands in no section.
fn section_index(entry: &VerdictEntry) -> Option<usize> {
    if is_abstention(entry) {
        Some(2)
    } else {
        match entry.choice.as_str() {
            "stickler" => Some(0),
            "clean" => Some(1),
            _ => None,
        }
    }
}

/// Return the list of defects found in a choices file.
/// Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_choices(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["choices must be a JSON object".to_string()];
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

/// Read a valid choices file into its reading.
///
/// The verdicts of the collection are gathered in
/// collection order and then stand under their outcome
/// in that order, under the rule FIX-CK-28 states, so
/// one flag stands in as many sections as its verdicts'
/// outcomes, its group in each section holding only the
/// pairs of that section's outcome, and every outcome
/// stands a section whether any verdict of the
/// collection stands under it or not. The counts stand
/// over the whole collection, the verdicts read count
/// counting every verdict the review's blocks carry,
/// including one standing in no section. A set the
/// loader refused reads as the groups its members
/// still yield, in the posture the set readings hold,
/// and the command line never renders one.
pub fn choices_reading_of(value: &JsonValue) -> ChoicesReading {
    let mut gathered: [Vec<ChoiceFlagGroup>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut sets_read = 0usize;
    let mut flags_read = 0usize;
    let mut flags_with_verdicts = 0usize;
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
                        flags_with_verdicts += 1;
                        verdicts_read += block.verdicts.len();
                        let mut held: [Vec<VerdictEntry>; 3] = [Vec::new(), Vec::new(), Vec::new()];
                        for verdict in &block.verdicts {
                            if let Some(index) = section_index(verdict) {
                                held[index].push(verdict.clone());
                            }
                        }
                        for (index, verdicts) in held.into_iter().enumerate() {
                            if !verdicts.is_empty() {
                                gathered[index].push(ChoiceFlagGroup {
                                    file: result.name.clone(),
                                    line: block.flag.line,
                                    pattern: block.flag.pattern.clone(),
                                    set_label: label.clone(),
                                    verdicts,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    let sections = OUTCOMES
        .iter()
        .zip(gathered)
        .map(|(outcome, groups)| ChoicesSection {
            outcome: outcome.to_string(),
            groups,
        })
        .collect();
    ChoicesReading {
        sections,
        sets_read,
        flags_read,
        flags_with_verdicts,
        verdicts_read,
    }
}
