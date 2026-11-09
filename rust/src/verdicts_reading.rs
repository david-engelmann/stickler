//! The verdicts reading. A verdicts reading takes a
//! collection of recorded review inputs in the census
//! form and states, per set, every flag of the set
//! carrying at least one recorded verdict together with
//! the verdict pairs standing on it, in the standing
//! set review form, so the recorded verdicts of a
//! collection stand as one list rather than being read
//! out of one set review at a time or summed out of the
//! census and outcomes tallies. It exists for the same
//! recorded inputs as the census, contested, pattern,
//! files, borderline, and outcomes readings, over the
//! same collection loading whole into all seven. The
//! loader validates each entry through the shared set
//! entry validation and each member through the change
//! set loader, the flags of each set come from the set
//! review, and the verdicts standing on each flag come
//! from that same review's blocks, so the reading holds
//! no second flagging, judging, or outcome logic of
//! its own. The obligations this module carries are
//! CK-27 and CK-28 in CONTRACT.md, in the Round 13
//! section.

use crate::changeset::{review_changeset, FileResult};
use crate::json::JsonValue;
use crate::sets::validate_entry;
use crate::verdicts::VerdictEntry;

/// One flag group of a verdicts reading, holding the
/// name of the file the flag stands in, the line number
/// of the flag, the pattern of the flag, and the
/// recorded verdicts standing on the flag in the order
/// they stand in the entry.
#[derive(Clone, Debug, PartialEq)]
pub struct VerdictFlagGroup {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub verdicts: Vec<VerdictEntry>,
}

/// One section of a verdicts reading, holding the label
/// of one set of the collection and the flag groups of
/// the set, one group for each flag of the set carrying
/// at least one recorded verdict, in collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct VerdictsSection {
    pub label: String,
    pub groups: Vec<VerdictFlagGroup>,
}

/// A whole verdicts reading, holding its sections in
/// collection order and the count of the flagged lines
/// of the whole collection, the flags carrying no
/// recorded verdict standing in that count and in no
/// group.
#[derive(Clone, Debug, PartialEq)]
pub struct VerdictsReading {
    pub sections: Vec<VerdictsSection>,
    pub flags_read: usize,
}

/// Gather the flag groups of one set from its set
/// review results, in the order the files stand in the
/// review and the flags stand in each file, holding
/// beside the groups the count of the flags of the set,
/// carrying verdicts or not.
fn groups_of(results: &[FileResult]) -> (Vec<VerdictFlagGroup>, usize) {
    let mut groups = Vec::new();
    let mut flags = 0usize;
    for result in results {
        for block in &result.blocks {
            flags += 1;
            if !block.verdicts.is_empty() {
                groups.push(VerdictFlagGroup {
                    file: result.name.clone(),
                    line: block.flag.line,
                    pattern: block.flag.pattern.clone(),
                    verdicts: block.verdicts.clone(),
                });
            }
        }
    }
    (groups, flags)
}

/// Return the list of defects found in a verdicts file.
/// Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home the census, rejudged,
/// contested, pattern, files, borderline, and outcomes
/// loaders all call.
pub fn load_verdicts(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["verdicts must be a JSON object".to_string()];
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

/// Read a valid verdicts file into its reading.
///
/// One section stands for each set of the collection in
/// collection order, holding the flag groups of the
/// set, and the count of the flagged lines stands over
/// the whole collection. A set the loader refused reads
/// as the groups its members still yield, in the
/// posture the set readings hold, and the command line
/// never renders one.
pub fn verdicts_reading_of(value: &JsonValue) -> VerdictsReading {
    let mut sections = Vec::new();
    let mut flags_read = 0usize;
    if let Some(entries) = value.get("sets").and_then(|sets| sets.as_array()) {
        for entry in entries {
            let label = entry
                .get("label")
                .and_then(|label| label.as_jstring())
                .and_then(|text| text.to_rust_string())
                .unwrap_or_default();
            let mut groups = Vec::new();
            if let Some(member) = entry.get("change_set") {
                let (found, flags) = groups_of(&review_changeset(member));
                groups = found;
                flags_read += flags;
            }
            sections.push(VerdictsSection { label, groups });
        }
    }
    VerdictsReading {
        sections,
        flags_read,
    }
}
