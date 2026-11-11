//! The standings reading. A standings reading takes
//! a collection of recorded review inputs in the census
//! form and states, per file standing in the collection,
//! every verdict pair of the collection standing on a
//! flag of that file together with the flag it stands
//! on, so the recorded verdicts of a collection stand
//! gathered by the file of the flag they stand on rather
//! than being read out of one set review at a time or
//! picked out of the verdicts reading by hand. It exists
//! for the same recorded inputs as the census, over the
//! same collection loading whole. The loader validates
//! each entry through the shared set entry validation
//! and each member through the change set loader, the
//! flags of each set come from the set review, the
//! verdicts standing on each flag come from that same
//! review's blocks, and the file a flag stands under is
//! the file of that review whose block carries the flag,
//! so the reading holds no second flagging or outcome
//! logic of its own. The obligations this module carries
//! are CK-39 and CK-40 in CONTRACT.md, in the Round 19
//! section.

use crate::changeset::review_changeset;
use crate::json::JsonValue;
use crate::sets::validate_entry;
use crate::verdicts::VerdictEntry;

/// One flag group of a standings reading, holding the
/// name of the file the flag stands in, the line number
/// of the flag, the pattern of the flag, the label of
/// the set the flag stands in, and all the recorded
/// verdicts standing on the flag, in the order they
/// stand in the entry.
#[derive(Clone, Debug, PartialEq)]
pub struct StandingFlagGroup {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
    pub verdicts: Vec<VerdictEntry>,
}

/// One section of a standings reading, holding one file
/// standing in the collection and the flag groups of
/// the collection standing on that file, one group for
/// each flag of the collection standing on the file and
/// carrying at least one recorded verdict, in
/// collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct StandingsSection {
    pub file: String,
    pub groups: Vec<StandingFlagGroup>,
}

/// A whole standings reading, holding its sections in
/// first appearance order across the collection and the
/// counts of the collection the builder counts, the
/// sets read, the flagged lines, and the verdicts read,
/// a verdict standing in its flag's group whole
/// whatever its choice, so the verdicts read count
/// counts every verdict the review's blocks carry.
#[derive(Clone, Debug, PartialEq)]
pub struct StandingsReading {
    pub sections: Vec<StandingsSection>,
    pub sets_read: usize,
    pub flags_read: usize,
    pub verdicts_read: usize,
}

/// Return the list of defects found in a standings
/// file. Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_standings(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["standings must be a JSON object".to_string()];
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

/// Read a valid standings file into its reading.
///
/// The flags of the collection are gathered in
/// collection order and then stand under the file they
/// stand on in that order, the pairs of a flag standing
/// whole in the flag's one group, so one flag stands in
/// exactly one section, and a file whose flags carry no
/// recorded verdicts stands no section. A flag carrying
/// no verdicts stands in no flag group. The counts
/// stand over the whole collection, the verdicts read
/// count counting every verdict the review's blocks
/// carry. A set the loader refused reads as the groups
/// its members still yield, in the posture the set
/// readings hold, and the command line never renders
/// one.
pub fn standings_reading_of(value: &JsonValue) -> StandingsReading {
    let mut sections: Vec<StandingsSection> = Vec::new();
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
                        let index = match sections
                            .iter()
                            .position(|section| section.file == result.name)
                        {
                            Some(index) => index,
                            None => {
                                sections.push(StandingsSection {
                                    file: result.name.clone(),
                                    groups: Vec::new(),
                                });
                                sections.len() - 1
                            }
                        };
                        if block.verdicts.is_empty() {
                            continue;
                        }
                        verdicts_read += block.verdicts.len();
                        sections[index].groups.push(StandingFlagGroup {
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
    sections.retain(|section| !section.groups.is_empty());
    StandingsReading {
        sections,
        sets_read,
        flags_read,
        verdicts_read,
    }
}
