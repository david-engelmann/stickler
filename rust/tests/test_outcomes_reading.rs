//! Pins for CK-25 and CK-26, the outcomes file and
//! reading.
//!
//! The outcomes pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per judgement in the
//! order CK-8 states them and standing whether a flag
//! stands under them or not, the flag lines in
//! collection order, the whole or nothing refusal, and
//! the closing counts in their stated relations to the
//! sections and to the census closings over the same
//! collection. The census pins hold the reading over
//! the shipped census collection against its golden
//! and against the census totals it is bound to. The
//! fencing pins hold the rendering to the one line
//! rule at the library surface, the reading standing
//! born inside the fencing programme. Every pin runs
//! offline on recorded and constructed entries, and
//! no pin makes a live call.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::outcomes::{
    load_outcomes, outcomes_reading_of, OutcomeFlag, OutcomeSection, OutcomesReading,
};
use stickler::render::{render_outcomes, render_outcomes_defects};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-outcomes-{}-{}-{}",
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

fn run_outcomes(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "outcomes.json", text);
    let (code, out, _err) = run(&["outcomes", &path]);
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
/// CK-25 and CK-12 share.
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

/// A diff holding no borderline lines and no judgements.
const CLEAN_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -1,1 +1,2 @@\n+    logger.debug(total)\n";

const STICKLER_ON_LINE_4: &str = "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]";

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_outcomes(&value),
        vec!["outcomes must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_outcomes(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(
        load_outcomes(&value),
        vec!["sets must be a list".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_outcomes(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_outcomes(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_outcomes(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

#[test]
fn the_empty_collection_renders_every_section_and_its_closings() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_outcomes(&value), Vec::<String>::new());
    assert_eq!(
        render_outcomes(&outcomes_reading_of(&value)),
        "Outcomes\n\
         \n\
         Judged stickler\n\
         No flagged lines\n\
         Flags read, 0\n\
         \n\
         Judged clean\n\
         No flagged lines\n\
         Flags read, 0\n\
         \n\
         Unjudged\n\
         No flagged lines\n\
         Flags read, 0\n\
         \n\
         Sets read, 0\n\
         Flags read, 0\n\
         Judged stickler, 0\n\
         Judged clean, 0\n\
         Unjudged, 0\n"
    );
}

#[test]
fn the_sections_stand_per_judgement_in_collection_order() {
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[entry_json(
                "app.py",
                FLAGS_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.9}]",
            )]),
        ),
        (
            "second set",
            change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")]),
        ),
    ]);
    let value = parse(&text).unwrap();
    assert_eq!(load_outcomes(&value), Vec::<String>::new());
    let reading = outcomes_reading_of(&value);
    assert_eq!(reading.sets_read, 2);
    let judgements: Vec<&str> = reading
        .sections
        .iter()
        .map(|section| section.judgement)
        .collect();
    assert_eq!(judgements, vec!["stickler", "clean", "unjudged"]);
    assert_eq!(
        render_outcomes(&reading),
        "Outcomes\n\
         \n\
         Judged stickler\n\
         app.py, line 4, near placeholder name, set, first set\n\
         Flags read, 1\n\
         \n\
         Judged clean\n\
         app.py, line 5, broad handler, set, first set\n\
         Flags read, 1\n\
         \n\
         Unjudged\n\
         app.py, line 4, near placeholder name, set, second set\n\
         Flags read, 1\n\
         \n\
         Sets read, 2\n\
         Flags read, 3\n\
         Judged stickler, 1\n\
         Judged clean, 1\n\
         Unjudged, 1\n"
    );
}

#[test]
fn a_judgement_holding_no_flag_states_the_no_flagged_lines_line() {
    let text = collection_json(&[(
        "only set",
        change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, STICKLER_ON_LINE_4)]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_outcomes(&outcomes_reading_of(&value));
    assert_eq!(
        rendered.matches("No flagged lines").count(),
        2,
        "{rendered}"
    );
    assert!(rendered.contains("Judged clean\nNo flagged lines\nFlags read, 0\n"));
    assert!(rendered.contains("Unjudged\nNo flagged lines\nFlags read, 0\n"));
}

#[test]
fn an_abstained_verdict_leaves_its_flag_unjudged() {
    let text = collection_json(&[(
        "only set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.32}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = outcomes_reading_of(&value);
    assert_eq!(reading.sections[0].flags.len(), 0);
    assert_eq!(reading.sections[2].flags.len(), 1);
    assert_eq!(reading.sections[2].flags[0].file, "app.py");
}

#[test]
fn a_contested_flag_stands_judged_stickler() {
    let text = collection_json(&[(
        "only set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.81},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.77}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = outcomes_reading_of(&value);
    assert_eq!(reading.sections[0].flags.len(), 1);
    assert_eq!(reading.sections[1].flags.len(), 0);
}

#[test]
fn the_closings_stand_in_their_relations_to_the_census() {
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[entry_json("app.py", FLAGS_DIFF, STICKLER_ON_LINE_4)]),
        ),
        (
            "quiet set",
            change_set_json(&[entry_json("clean.py", CLEAN_DIFF, "")]),
        ),
        (
            "third set",
            change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let reading = outcomes_reading_of(&value);
    let census = census_of(&value);
    let flags: usize = reading.sections.iter().map(|s| s.flags.len()).sum();
    assert_eq!(flags, census.total.borderline);
    assert_eq!(reading.sections[0].flags.len(), census.total.stickler);
    assert_eq!(reading.sections[1].flags.len(), census.total.clean);
    assert_eq!(reading.sections[2].flags.len(), census.total.unjudged);
    let rendered = render_outcomes(&reading);
    assert!(rendered.contains("Sets read, 3\n"), "{rendered}");
    assert!(rendered.contains("Flags read, 3\n"), "{rendered}");
    assert!(rendered.contains("Judged stickler, 1\n"), "{rendered}");
    assert!(rendered.contains("Unjudged, 2\n"), "{rendered}");
}

#[test]
fn the_reading_is_deterministic() {
    let text = collection_json(&[(
        "only set",
        change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(
        render_outcomes(&outcomes_reading_of(&value)),
        render_outcomes(&outcomes_reading_of(&value))
    );
}

#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let (code, out) = run_outcomes(&dir, "{\"sets\": [{\"change_set\": {\"files\": []}}]}");
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Outcomes\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[0].label is missing\n"
    );
}

#[test]
fn a_malformed_outcomes_rendering_names_its_defects() {
    let rendered = render_outcomes_defects(&["sets is missing".to_string()]);
    assert_eq!(
        rendered,
        "Outcomes\nVerdict, invalid\nDefects, 1\nDefect, sets is missing\n"
    );
}

#[test]
fn outcomes_on_a_missing_file_is_malformed_with_exit_2() {
    let dir = temp_dir("gone");
    let missing = dir.join("gone.json").to_string_lossy().into_owned();
    let (code, out, _err) = run(&["outcomes", &missing]);
    assert_eq!(code, 2);
    assert!(out.contains("Defect, the outcomes file could not be read\n"));
}

#[test]
fn outcomes_on_a_non_json_file_is_malformed_with_exit_2() {
    let dir = temp_dir("nonjson");
    let (code, out) = run_outcomes(&dir, "not json");
    assert_eq!(code, 2);
    assert!(out.contains("Defect, the file does not hold a JSON outcomes collection\n"));
}

/// The census collection, read whole. The reading over
/// examples/outcomes.json holds fourteen flags judged
/// stickler, two judged clean, and two unjudged, the
/// closing counts of the census golden over the same
/// collection, and every section stands over the
/// shipped collection.
#[test]
fn the_census_collection_reads_with_its_sections_and_closings() {
    let text = std::fs::read_to_string(examples().join("outcomes.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_outcomes(&value), Vec::<String>::new());
    let reading = outcomes_reading_of(&value);
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.sections.len(), 3);
    assert_eq!(reading.sections[0].flags.len(), 14);
    assert_eq!(reading.sections[1].flags.len(), 2);
    assert_eq!(reading.sections[2].flags.len(), 2);
    let census = census_of(&value);
    assert_eq!(reading.sections[0].flags.len(), census.total.stickler);
    assert_eq!(reading.sections[1].flags.len(), census.total.clean);
    assert_eq!(reading.sections[2].flags.len(), census.total.unjudged);
    let rendered = render_outcomes(&reading);
    assert!(rendered.contains("Sets read, 3\n"), "{rendered}");
    assert!(rendered.contains("Flags read, 18\n"), "{rendered}");
    assert!(rendered.contains("Judged stickler, 14\n"), "{rendered}");
    assert!(rendered.contains("Judged clean, 2\n"), "{rendered}");
    assert!(rendered.contains("Unjudged, 2\n"), "{rendered}");
}

#[test]
fn the_example_matches_its_golden_through_the_command() {
    let path = examples().join("outcomes.json");
    let (code, out, _err) = run(&["outcomes", path.to_str().unwrap()]);
    assert_eq!(code, 0);
    let golden = std::fs::read_to_string(examples().join("outcomes.txt")).unwrap();
    assert_eq!(out, golden);
}

#[test]
fn the_example_matches_its_golden_through_the_library() {
    let text = std::fs::read_to_string(examples().join("outcomes.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_outcomes(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("outcomes.txt")).unwrap();
    assert_eq!(render_outcomes(&outcomes_reading_of(&value)), golden);
}

/// The fencing of the outcomes reading at the library
/// surface, over a reading built by hand. A file name
/// holding a line break and a flags read line's own
/// text, a pattern holding a break and a section head's
/// own text, and a set label holding a break and a sets
/// read line's own text each print as the empty field,
/// so no flag line, section head, or closing line is
/// forged. A handed reading the loaders refused prints
/// its fields raw nowhere, as the collection readings
/// before it stood closed from their first writing.
#[test]
fn a_line_break_in_a_hand_built_reading_forges_no_flag_head_or_closing_line() {
    let reading = OutcomesReading {
        sections: vec![
            OutcomeSection {
                judgement: "stickler",
                flags: vec![OutcomeFlag {
                    file: "evil.py\nFlags read, 99".to_string(),
                    line: 4,
                    pattern: "near placeholder name\nJudged clean".to_string(),
                    set_label: "one set\nSets read, 99".to_string(),
                }],
            },
            OutcomeSection {
                judgement: "clean",
                flags: Vec::new(),
            },
            OutcomeSection {
                judgement: "unjudged",
                flags: Vec::new(),
            },
        ],
        sets_read: 1,
    };
    let rendered = render_outcomes(&reading);
    assert!(
        rendered.lines().any(|line| line == ", line 4, , set, "),
        "{rendered}"
    );
    assert_eq!(
        rendered
            .lines()
            .filter(|line| *line == "Judged clean")
            .count(),
        1,
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Sets read, 1"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Flags read, 1"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Flags read, 99"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Sets read, 99"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}
