//! The flags reading. A flags reading takes a
//! collection of recorded review inputs in the census
//! form and states, per file verdict state, every flag
//! of the collection standing on a file standing in
//! that state as one flag line, so the flagged lines
//! of a collection stand gathered by the state of the
//! file they stand on rather than being read out of
//! one file section at a time or picked out of the
//! states reading by hand. It exists for the same
//! recorded inputs as the census, over the same
//! collection loading whole. The loader validates each
//! entry through the shared set entry validation and
//! each member through the change set loader, the flags
//! of each set come from the set review, the verdicts
//! standing on each flag come from that same review's
//! blocks, and the file a flag stands under is the file
//! of that review whose block carries the flag, so the
//! reading holds no second flagging or outcome logic
//! of its own. The obligations this module carries are
//! CK-43 and CK-44 in CONTRACT.md, in the Round 21
//! section.

use crate::changeset::review_changeset;
use crate::json::JsonValue;
use crate::outcomes::JUDGEMENTS;
use crate::sets::validate_entry;
use crate::verdicts::{is_contested, line_outcome, VerdictEntry};

/// One flag line of a flags reading, holding the name
/// of the file the flag stands in, the line number of
/// the flag, the pattern of the flag, the label of the
/// set the flag stands in, and whether the flag stands
/// contested under CK-9.
#[derive(Clone, Debug, PartialEq)]
pub struct FlagsLine {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
    pub contested: bool,
}

/// One section of a flags reading, holding one file
/// verdict state and the flag lines of the collection
/// standing on files standing in that state, in
/// collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct FlagsSection {
    pub state: &'static str,
    pub lines: Vec<FlagsLine>,
}

/// A whole flags reading, holding its three sections
/// in the order CK-8 states the judgements and the
/// counts of the collection the builder counts, the
/// sets read and the flagged lines of the whole
/// collection, a flagged line standing counted in the
/// flags read count whether the file it stands on
/// carries a state or carries none.
#[derive(Clone, Debug, PartialEq)]
pub struct FlagsReading {
    pub sections: Vec<FlagsSection>,
    pub sets_read: usize,
    pub flags_read: usize,
}

/// Return the list of defects found in a flags
/// file. Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_flags(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["flags must be a JSON object".to_string()];
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

/// Read a valid flags file into its reading.
///
/// The flag lines of the collection are gathered in
/// collection order, and each line then stands in the
/// one section of the state of the file it stands on,
/// the state the one outcome helper of CK-8 states
/// over the pool of every recorded verdict standing
/// on the file's flags across the collection under
/// CK-41, so one flag stands in exactly one section
/// and a file whose flags carry no recorded verdicts
/// stands in no section, though the flags read count
/// counts its flags. Every flag standing on a file
/// carrying a state stands as a line, whether it
/// carries a recorded verdict or carries none. The
/// counts stand over the whole collection. A set the
/// loader refused reads as the lines its members still
/// yield, in the posture the set readings hold, and the
/// command line never renders one.
pub fn flags_reading_of(value: &JsonValue) -> FlagsReading {
    let mut lines: Vec<FlagsLine> = Vec::new();
    let mut pools: Vec<(String, Vec<VerdictEntry>)> = Vec::new();
    let mut sets_read = 0usize;
    let mut flags_read = 0usize;
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
                    match pools.iter_mut().find(|(file, _)| file == &result.name) {
                        Some((_, pool)) => pool.extend(
                            result
                                .blocks
                                .iter()
                                .flat_map(|block| block.verdicts.iter().cloned()),
                        ),
                        None => pools.push((
                            result.name.clone(),
                            result
                                .blocks
                                .iter()
                                .flat_map(|block| block.verdicts.iter().cloned())
                                .collect(),
                        )),
                    }
                    for block in &result.blocks {
                        flags_read += 1;
                        lines.push(FlagsLine {
                            file: result.name.clone(),
                            line: block.flag.line,
                            pattern: block.flag.pattern.clone(),
                            set_label: label.clone(),
                            contested: is_contested(&block.verdicts),
                        });
                    }
                }
            }
        }
    }
    let state_of = |file: &str| -> Option<&'static str> {
        pools
            .iter()
            .find(|(name, _)| name == file)
            .filter(|(_, pool)| !pool.is_empty())
            .map(|(_, pool)| line_outcome(pool))
    };
    let mut sections = Vec::new();
    for state in JUDGEMENTS {
        let section_lines = lines
            .iter()
            .filter(|line| state_of(&line.file) == Some(state))
            .cloned()
            .collect();
        sections.push(FlagsSection {
            state,
            lines: section_lines,
        });
    }
    FlagsReading {
        sections,
        sets_read,
        flags_read,
    }
}
