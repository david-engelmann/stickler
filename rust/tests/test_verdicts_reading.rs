//! Pins for CK-27 and CK-28, the verdicts file and
//! reading.
//!
//! The verdicts pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per set in collection
//! order, the flag groups holding only the flags that
//! carry a recorded verdict, the pairs in the standing
//! set review form, the empty form where no flag of a
//! set carries one, and the closing counts in their
//! stated relations to the sections and to the census
//! and outcomes closings over the same collection. The
//! census pins hold the reading over the shipped
//! verdicts collection against its golden and against
//! the census and outcomes totals it is bound to. The
//! loader pins name each defect of the file in the
//! forms CK-12 states, one pin per defect. The fencing
//! pins hold the rendering to the one line rule at the
//! library surface, the reading standing born inside
//! the fencing programme. Every pin runs offline on
//! recorded and constructed entries, and no pin makes
//! a live call.
//!
//! The discrimination split of this file stands as
//! follows. Every pin fails against the tree Round 12
//! closed on, the library and loader pins by the
//! absence of the module they name, and the command
//! pins by that absence and by the base binary
//! answering a verdicts invocation with the usage
//! text. Within the slice, the abstention boundary
//! pin discriminates against a second abstention
//! logic in the new module, the binding pins against
//! a second flagging or outcome logic, and each
//! fencing pin against a raw print in its position.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::outcomes::outcomes_reading_of;
use stickler::render::{render_verdicts, render_verdicts_defects};
use stickler::verdicts::{line_outcome, VerdictEntry};
use stickler::verdicts_reading::{
    load_verdicts, verdicts_reading_of, VerdictFlagGroup, VerdictsReading, VerdictsSection,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-verdicts-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    path.to_string_lossy().into_owned()
}

fn run(args: &[&str]) -> (i32, String, String) {
    let owned: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    let result = cli_main(&owned);
    (result.code, result.stdout, result.stderr)
}

fn run_verdicts(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "verdicts.json", text);
    let (code, out, _err) = run(&["verdicts", &path]);
    (code, out)
}

/// Escape a string as JSON string content.
fn jstr(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// One change set entry as JSON text, verdicts appended raw.
fn entry_json(file: &str, diff: &str, verdicts: &str) -> String {
    let verdict_part = if verdicts.is_empty() {
        String::new()
    } else {
        format!(",\"verdicts\":{verdicts}")
    };
    format!(
        "{{\"file\":\"{}\",\"diff\":\"{}\"{}}}",
        jstr(file),
        jstr(diff),
        verdict_part
    )
}

/// One collection text over the given set entries, each a
/// pair of a label and a change set JSON text, in the form
/// CK-27 and CK-12 share.
fn collection_json(sets: &[(&str, String)]) -> String {
    let parts: Vec<String> = sets
        .iter()
        .map(|(label, change_set)| {
            format!(
                "{{\"label\":\"{}\",\"change_set\":{}}}",
                jstr(label),
                change_set
            )
        })
        .collect();
    format!("{{\"sets\":[{}]}}", parts.join(","))
}

fn change_set_json(entries: &[String]) -> String {
    format!("{{\"files\":[{}]}}", entries.join(","))
}

/// A diff holding two borderline lines and no judgements,
/// the near placeholder name at line 4 and the broad
/// handler at line 5.
const FLAGS_DIFF: &str =
    "--- a/app.py\n+++ b/app.py\n@@ -1,1 +1,3 @@\n+data = load_entries()\n+    except BaseException:\n";

/// A diff holding one borderline line and no judgements,
/// the near placeholder name at line 4.
const ONE_FLAG_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -1,1 +1,2 @@\n+data = load_entries()\n";

#[test]
fn the_example_matches_its_golden_through_the_command() {
    let path = examples().join("verdicts.json");
    let (code, out, _err) = run(&["verdicts", path.to_str().unwrap()]);
    assert_eq!(code, 0);
    let golden = std::fs::read_to_string(examples().join("verdicts.txt")).unwrap();
    assert_eq!(out, golden);
}

#[test]
fn the_example_matches_its_golden_through_the_library() {
    let text = std::fs::read_to_string(examples().join("verdicts.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_verdicts(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("verdicts.txt")).unwrap();
    assert_eq!(render_verdicts(&verdicts_reading_of(&value)), golden);
}

/// The count standing after a prefix on its own line of a
/// rendering, read as a number.
fn count_after(text: &str, prefix: &str) -> usize {
    text.lines()
        .find(|line| line.starts_with(prefix))
        .unwrap_or_else(|| panic!("no line opens with {prefix}"))
        .strip_prefix(prefix)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

/// The shipped collection, read whole. The round 2 set
/// carries no verdict, the pilot set carries twenty two
/// verdicts on twelve flags, and the constructed round
/// 4 example carries ten on five, and the closings read
/// three sets, eighteen flags, seventeen flags with
/// verdicts, and thirty two verdicts, of outcomes
/// twenty four chose stickler, five chose clean, and
/// three abstained, the three outcome counts summing to
/// the verdicts read count.
#[test]
fn the_shipped_collection_closes_three_ways() {
    let text = std::fs::read_to_string(examples().join("verdicts.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = verdicts_reading_of(&value);
    assert_eq!(reading.sections.len(), 3);
    assert!(reading.sections[0].groups.is_empty());
    let pairs: Vec<usize> = reading
        .sections
        .iter()
        .map(|section| {
            section
                .groups
                .iter()
                .map(|group| group.verdicts.len())
                .sum::<usize>()
        })
        .collect();
    assert_eq!(pairs, vec![0, 22, 10]);
    let rendered = render_verdicts(&reading);
    for line in [
        "Verdicts read, 0\n",
        "Verdicts read, 22\n",
        "Verdicts read, 10\n",
        "Sets read, 3\n",
        "Flags read, 18\n",
        "Flags with verdicts, 17\n",
        "Verdicts read, 32\n",
        "Chose stickler, 24\n",
        "Chose clean, 5\n",
        "Abstained, 3\n",
    ] {
        assert!(rendered.contains(line), "the reading states {line}");
    }
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// The closings of the verdicts reading over the
/// shipped collection stand in their relations to the
/// census and the outcomes reading over the same
/// collection, the flags read count the census total
/// block's flagged lines, the flags with verdicts
/// count the outcomes judged counts summed with the
/// one unjudged flag of the collection carrying a
/// recorded verdict, the second unjudged flag carrying
/// none.
#[test]
fn the_closings_bind_to_the_census_and_outcomes_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("verdicts.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = verdicts_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.flags_read, census.total.borderline);
    assert_eq!(reading.flags_read, 18);
    assert_eq!(reading.sections.len(), census.sets.len());
    let outcomes = outcomes_reading_of(&value);
    assert_eq!(outcomes.sections[0].flags.len(), 14);
    assert_eq!(outcomes.sections[1].flags.len(), 2);
    assert_eq!(outcomes.sections[2].flags.len(), 2);
    let groups: Vec<&VerdictFlagGroup> = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .collect();
    assert_eq!(groups.len(), 17);
    let unjudged_carrying = groups
        .iter()
        .filter(|group| line_outcome(&group.verdicts) == "unjudged")
        .count();
    assert_eq!(unjudged_carrying, 1);
    assert_eq!(
        groups.len(),
        outcomes.sections[0].flags.len() + outcomes.sections[1].flags.len() + unjudged_carrying
    );
}

#[test]
fn the_empty_form_stands_for_a_set_carrying_no_verdicts() {
    let text = collection_json(&[(
        "only set",
        change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_verdicts(&value), Vec::<String>::new());
    assert_eq!(
        render_verdicts(&verdicts_reading_of(&value)),
        "Verdicts\n\
         \n\
         Set, only set\n\
         Verdicts, none recorded\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 1\n\
         Flags read, 2\n\
         Flags with verdicts, 0\n\
         Verdicts read, 0\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}

#[test]
fn the_empty_collection_renders_its_head_and_zero_closings() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_verdicts(&value), Vec::<String>::new());
    assert_eq!(
        render_verdicts(&verdicts_reading_of(&value)),
        "Verdicts\n\
         \n\
         Sets read, 0\n\
         Flags read, 0\n\
         Flags with verdicts, 0\n\
         Verdicts read, 0\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}

/// The abstention boundary of CK-7 stands at exactly
/// one half in the new reading, read through the one
/// outcome helper of the seat. The verdict at exactly
/// one half stands under its choice, the verdicts just
/// below one half and at zero stand as abstained, and
/// a contested flag states both of its pairs in the
/// order the verdicts stand on it. The whole rendering
/// stands byte for byte, so a second abstention logic
/// in the new module fails this pin wherever it
/// disagrees with the seat.
#[test]
fn the_abstention_boundary_stands_at_one_half() {
    let text = collection_json(&[(
        "boundary set",
        change_set_json(&[
            entry_json(
                "app.py",
                FLAGS_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.5},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.25}]",
            ),
            entry_json(
                "util.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"stickler\",\"confidence\":0}]",
            ),
            entry_json(
                "core.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.81},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.77}]",
            ),
        ]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_verdicts(&value), Vec::<String>::new());
    assert_eq!(
        render_verdicts(&verdicts_reading_of(&value)),
        "Verdicts\n\
         \n\
         Set, boundary set\n\
         app.py, line 4, near placeholder name, set, boundary set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.5\n\
         app.py, line 5, broad handler, set, boundary set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose clean with confidence 0.25 and abstained\n\
         util.py, line 4, near placeholder name, set, boundary set\n\
         Verdict, openai/gpt-6-luna, abstained\n\
         Reason, openai/gpt-6-luna chose stickler with confidence 0 and abstained\n\
         core.py, line 4, near placeholder name, set, boundary set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.81\n\
         Verdict, openai/gpt-6-luna, clean\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.77\n\
         Verdicts read, 5\n\
         \n\
         Sets read, 1\n\
         Flags read, 4\n\
         Flags with verdicts, 4\n\
         Verdicts read, 5\n\
         Chose stickler, 2\n\
         Chose clean, 1\n\
         Abstained, 2\n"
    );
}

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["verdicts must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_verdicts(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets must be a list".to_string()]
    );
}

#[test]
fn an_entry_that_is_not_an_object_is_refused() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_that_is_empty_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_missing_change_set_field_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_verdicts(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the last set of a collection renders
/// the defect list and no reading, whole or nothing,
/// so no partial verdicts reading ever prints.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")])
    );
    let (code, out) = run_verdicts(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Verdicts\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

#[test]
fn a_malformed_verdicts_rendering_names_its_defects() {
    let rendered = render_verdicts_defects(&["sets is missing".to_string()]);
    assert_eq!(
        rendered,
        "Verdicts\nVerdict, invalid\nDefects, 1\nDefect, sets is missing\n"
    );
}

#[test]
fn verdicts_on_a_missing_file_is_malformed_with_exit_2() {
    let dir = temp_dir("gone");
    let missing = dir.join("gone.json").to_string_lossy().into_owned();
    let (code, out, _err) = run(&["verdicts", &missing]);
    assert_eq!(code, 2);
    assert!(out.contains("Defect, the verdicts file could not be read\n"));
}

#[test]
fn verdicts_on_a_non_json_file_is_malformed_with_exit_2() {
    let dir = temp_dir("nonjson");
    let (code, out) = run_verdicts(&dir, "not json");
    assert_eq!(code, 2);
    assert!(out.contains("Defect, the file does not hold a JSON verdicts collection\n"));
}

/// Build one recorded verdict entry by hand, its
/// confidence read from the JSON text it is given.
fn handed_entry(model: &str, choice: &str, confidence: &str) -> VerdictEntry {
    let value = parse(&format!("{{\"confidence\": {confidence}}}")).unwrap();
    VerdictEntry {
        line: 4,
        pattern: "near placeholder name".to_string(),
        model: model.to_string(),
        choice: choice.to_string(),
        confidence: value
            .get("confidence")
            .and_then(|held| held.as_number())
            .unwrap()
            .clone(),
    }
}

/// The fencing of the verdicts reading at the library
/// surface, over readings built by hand. A set label
/// holding a break and a closing line's own text
/// prints as the empty field in the section head and
/// in the flag lines alike, so no section head or
/// closing line is forged.
#[test]
fn a_line_break_in_a_section_label_forges_no_closing_line() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set\nSets read, 99".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "app.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                verdicts: vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            }],
        }],
        flags_read: 1,
    };
    let rendered = render_verdicts(&reading);
    assert!(rendered.lines().any(|line| line == "Set, "), "{rendered}");
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, line 4, near placeholder name, set, "),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Sets read, 1"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Sets read, 99"),
        "{rendered}"
    );
}

/// A file name holding a break and a verdicts read
/// line's own text, and a pattern holding a break and
/// a chose line's own text, each print as the empty
/// field, so no flag line or closing line is forged.
#[test]
fn a_line_break_in_a_flag_file_or_pattern_forges_no_flag_or_closing_line() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "evil.py\nVerdicts read, 99".to_string(),
                line: 4,
                pattern: "near placeholder name\nChose stickler, 99".to_string(),
                verdicts: vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            }],
        }],
        flags_read: 1,
    };
    let rendered = render_verdicts(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == ", line 4, , set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Chose stickler, 99"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Chose stickler, 1"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// A pair model holding a break prints as the empty
/// field in the verdict line, and the constructed
/// reason, built from the model, carries the break and
/// is fenced whole in the reason line, exactly as the
/// set review fences it.
#[test]
fn a_line_break_in_a_pair_model_is_fenced_in_the_verdict_and_reason_lines_alike() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "app.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                verdicts: vec![handed_entry(
                    "jev-1.13.0\nChose clean, 99",
                    "stickler",
                    "0.9",
                )],
            }],
        }],
        flags_read: 1,
    };
    let rendered = render_verdicts(&reading);
    assert!(
        rendered.lines().any(|line| line == "Verdict, , stickler"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Reason, "),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Chose clean, 99"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Chose stickler, 1"),
        "{rendered}"
    );
}

/// A handed verdict whose choice stands outside the
/// two labels the loader admits, here holding a break
/// and a chose line's own text, prints its pair fenced
/// and counts in the verdicts read count and in none
/// of the three outcome counts, the posture CK-28
/// states for it, so the rendering states it byte for
/// byte.
#[test]
fn a_handed_choice_holding_a_break_prints_fenced_and_counts_in_no_outcome() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "app.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                verdicts: vec![handed_entry("jev-1.13.0", "clean\nChose clean, 99", "0.9")],
            }],
        }],
        flags_read: 1,
    };
    let rendered = render_verdicts(&reading);
    assert_eq!(
        rendered,
        "Verdicts\n\nSet, one set\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, \n\
         Reason, \n\
         Verdicts read, 1\n\n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 1\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}

/// A handed verdict whose choice is the word
/// abstained at a confidence that is no abstention
/// under CK-7 prints its pair with the choice standing
/// as written and counts in the verdicts read count
/// and in none of the three outcome counts, under
/// FIX-CK-28, in the posture CK-28 states for a choice
/// outside the two labels the loader admits. The
/// discrimination split of the two pins of this repair
/// stands as follows, pin by pin. This pin fails
/// against the unrepaired tree by assertion on the
/// abstained closing count, the unrepaired rendering
/// counting the verdict there because its abstention
/// branch stood on the outcome word the helper
/// returns and that helper returns the choice standing
/// as written for a verdict that does not abstain. The
/// control pin below it passes on the unrepaired tree
/// by design, a true abstention counting in the
/// abstained closing under the unrepaired branch and
/// the repaired one alike, and it fails if the repair
/// ever stops counting a true abstention.
#[test]
fn a_handed_abstained_word_choice_at_no_abstention_confidence_counts_in_no_outcome() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "app.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                verdicts: vec![handed_entry("jev-1.13.0", "abstained", "0.9")],
            }],
        }],
        flags_read: 1,
    };
    assert_eq!(
        render_verdicts(&reading),
        "Verdicts\n\
         \n\
         Set, one set\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose abstained with confidence 0.9\n\
         Verdicts read, 1\n\
         \n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 1\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}

/// A handed verdict that is a true abstention under
/// CK-7, its confidence below one half, prints its
/// pair in the abstention form and counts in the
/// abstained closing, the control pin of the repair
/// above, passing on the unrepaired tree by design as
/// its split states.
#[test]
fn a_handed_true_abstention_still_counts_in_the_abstained_closing() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "app.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                verdicts: vec![handed_entry("jev-1.13.0", "stickler", "0.3")],
            }],
        }],
        flags_read: 1,
    };
    assert_eq!(
        render_verdicts(&reading),
        "Verdicts\n\
         \n\
         Set, one set\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.3 and abstained\n\
         Verdicts read, 1\n\
         \n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 1\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 1\n"
    );
}

/// A handed reading stating a flag group carrying no
/// verdicts renders its flag line and no pairs, and
/// the flag counts in flags read and not in flags with
/// verdicts, the posture CK-28 states for it.
#[test]
fn a_handed_flag_carrying_no_verdicts_renders_its_line_and_no_pairs() {
    let reading = VerdictsReading {
        sections: vec![VerdictsSection {
            label: "one set".to_string(),
            groups: vec![VerdictFlagGroup {
                file: "app.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                verdicts: Vec::new(),
            }],
        }],
        flags_read: 1,
    };
    assert_eq!(
        render_verdicts(&reading),
        "Verdicts\n\
         \n\
         Set, one set\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 0\n\
         Verdicts read, 0\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}
