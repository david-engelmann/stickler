//! The models reading. A models reading takes a
//! collection of recorded review inputs in the census
//! form and states, per model standing in the
//! collection, every flag of the collection carrying a
//! verdict of that model together with the one pair of
//! that model standing on it, so the recorded verdicts
//! of a collection stand gathered by the model that
//! recorded them rather than being read out of one set
//! review at a time or picked out of the verdicts
//! reading by hand. It exists for the same recorded
//! inputs as the census, over the same collection
//! loading whole. The loader validates each entry
//! through the shared set entry validation and each
//! member through the change set loader, the flags of
//! each set come from the set review, the verdicts
//! standing on each flag come from that same review's
//! blocks, and the file a flag stands under is the file
//! of that review whose block carries the flag, so the
//! reading holds no second flagging or outcome logic
//! of its own. The obligations this module carries are
//! CK-45 and CK-46 in CONTRACT.md, in the Round 23
//! section.

use crate::changeset::review_changeset;
use crate::json::JsonValue;
use crate::sets::validate_entry;
use crate::verdicts::VerdictEntry;

/// One flag group of a models reading, holding the
/// name of the file the flag stands in, the line number
/// of the flag, the pattern of the flag, the label of
/// the set the flag stands in, and the one recorded
/// verdict of the section's model standing on the flag.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelsFlagGroup {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub set_label: String,
    pub verdict: VerdictEntry,
}

/// One section of a models reading, holding one model
/// standing in the collection and the flag groups of
/// the collection carrying a verdict of that model, one
/// group for each flag of the collection carrying at
/// least one verdict of the model, in collection order.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelsSection {
    pub model: String,
    pub groups: Vec<ModelsFlagGroup>,
}

/// A whole models reading, holding its sections in
/// first appearance order across the collection and the
/// counts of the collection the builder counts, the
/// sets read, the flagged lines, the flags carrying at
/// least one recorded verdict, and the verdicts read,
/// a verdict standing in its flag's group under its own
/// model whatever its choice, so the verdicts read
/// count counts every verdict the review's blocks
/// carry and the flag groups over the sections count to
/// it, one group standing per pair.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelsReading {
    pub sections: Vec<ModelsSection>,
    pub sets_read: usize,
    pub flags_read: usize,
    pub flags_with_verdicts: usize,
    pub verdicts_read: usize,
}

/// Return the list of defects found in a models
/// file. Never fails, whatever it is handed.
///
/// Each entry is validated through the shared set entry
/// validation, the one home every collection loader calls.
pub fn load_models(value: &JsonValue) -> Vec<String> {
    let mut defects = Vec::new();
    if !value.is_object() {
        return vec!["models must be a JSON object".to_string()];
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

/// Read a valid models file into its reading.
///
/// The flags of the collection are gathered in
/// collection order and each pair then stands under
/// the model the pair itself names, a flag carrying
/// verdicts of two models standing as a group in both
/// models' sections with the one pair of each, so one
/// pair stands in exactly one section and a flag
/// carrying no recorded verdict stands in no section.
/// The counts stand over the whole collection, the
/// flags with verdicts count counting each block
/// carrying at least one verdict once at each standing
/// of its entry, however many models its verdicts
/// name, and the verdicts read count counting every
/// verdict the review's blocks carry. A set the loader refused
/// reads as the groups its members still yield, in the
/// posture the set readings hold, and the command line
/// never renders one.
pub fn models_reading_of(value: &JsonValue) -> ModelsReading {
    let mut sections: Vec<ModelsSection> = Vec::new();
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
                        if !block.verdicts.is_empty() {
                            flags_with_verdicts += 1;
                        }
                        for verdict in &block.verdicts {
                            verdicts_read += 1;
                            let index = match sections
                                .iter()
                                .position(|section| section.model == verdict.model)
                            {
                                Some(index) => index,
                                None => {
                                    sections.push(ModelsSection {
                                        model: verdict.model.clone(),
                                        groups: Vec::new(),
                                    });
                                    sections.len() - 1
                                }
                            };
                            sections[index].groups.push(ModelsFlagGroup {
                                file: result.name.clone(),
                                line: block.flag.line,
                                pattern: block.flag.pattern.clone(),
                                set_label: label.clone(),
                                verdict: verdict.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    ModelsReading {
        sections,
        sets_read,
        flags_read,
        flags_with_verdicts,
        verdicts_read,
    }
}
