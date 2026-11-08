//! The borderline reading. A borderline reading takes
//! a collection of recorded review inputs in the census
//! form and states, per set holding at least one, every
//! borderline flag of the collection, the flags standing
//! between the two shapes of their pattern, so which
//! lines of a collection stand between two shapes stands
//! as one list rather than being read out of one set
//! review at a time. It exists for the same recorded
//! inputs as the census, contested, pattern, and files
//! readings, over the same collection loading whole into
//! all five. The loader validates each entry through
//! the shared set entry validation and each member
//! through the change set loader, and the flags come
//! from the set review, so the reading holds no second
//! flagging or judging logic of its own. The obligations
//! this module carries are CK-23 and CK-24 in
//! CONTRACT.md, in the Round 11 section.

use crate::changeset::review_changeset;
use crate::checker::BORDERLINE_PATTERNS;
use crate::json::JsonValue;
use crate::sets::validate_entry;

/// One borderline flag of a borderline reading, holding
/// the name of the file the flag stands in, the line
/// number of the flag, the pattern of the flag, the
/// flagged line as written, and the shape reading the
/// checker computed for the flag.
#[derive(Clone, Debug, PartialEq)]
pub struct BorderlineFlag {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub content: String,
    pub reading: String,
}

/// One section of a borderline reading, holding one set
/// holding at least one borderline flag and the
/// borderline flags of that set in the order the set
/// review raised them.
#[derive(Clone, Debug, PartialEq)]
pub struct BorderlineSection {
    pub label: String,
    pub flags: Vec<BorderlineFlag>,
}

/// A whole borderline reading, holding its sections in
/// the order the sets stand in the collection and the
/// count of the sets of the collection.
#[derive(Clone, Debug, PartialEq)]
pub struct BorderlineReading {
    pub sections: Vec<BorderlineSection>,
    pub sets_read: usize,
}

/// Return the list of defects found in a borderline
/// file. Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home the census, rejudged,
/// contested, pattern, and files loaders all call.
pub fn load_borderline(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["borderline must be a JSON object".to_string()];
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

/// Read a valid borderline file into its reading.
///
/// The borderline flags of each set are gathered in the
/// order the set review raised them, the files in the
/// order they stand in the set and the flags in the
/// order they stand in their file, and a set holding no
/// borderline flag stands no section, because the
/// reading states the borderline flags of a collection.
/// A set the loader refused reads as the flags its
/// members still yield, in the posture the set readings
/// hold, and the command line never renders one.
pub fn borderline_reading_of(value: &JsonValue) -> BorderlineReading {
    let mut sections: Vec<BorderlineSection> = Vec::new();
    let mut sets_read = 0usize;
    if let Some(entries) = value.get("sets").and_then(|sets| sets.as_array()) {
        for entry in entries {
            sets_read += 1;
            let label = entry
                .get("label")
                .and_then(|label| label.as_jstring())
                .and_then(|text| text.to_rust_string())
                .unwrap_or_default();
            let mut flags: Vec<BorderlineFlag> = Vec::new();
            if let Some(member) = entry.get("change_set") {
                for result in review_changeset(member) {
                    for block in &result.blocks {
                        if BORDERLINE_PATTERNS.contains(&block.flag.pattern.as_str()) {
                            flags.push(BorderlineFlag {
                                file: result.name.clone(),
                                line: block.flag.line,
                                pattern: block.flag.pattern.clone(),
                                content: block.flag.content.clone(),
                                reading: block.flag.reading.clone(),
                            });
                        }
                    }
                }
            }
            if !flags.is_empty() {
                sections.push(BorderlineSection { label, flags });
            }
        }
    }
    BorderlineReading {
        sections,
        sets_read,
    }
}
