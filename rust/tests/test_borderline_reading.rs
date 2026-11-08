//! Pins for CK-23 and CK-24, the borderline file and
//! reading.
//!
//! The borderline pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per set holding at least
//! one borderline flag in collection order, a set
//! holding none standing no section, the flag lines in
//! set review order, the whole or nothing refusal, and
//! the closing counts in their stated relations to the
//! sections and to the census, pattern, and files
//! closings over the same collection. The census pins
//! hold the reading over the shipped census collection
//! against its golden and against the closings it is
//! bound to. The fencing pins hold the rendering to the
//! one line rule at the library surface under
//! FIX-CK-23. Every pin runs offline on recorded and
//! constructed entries, and no pin makes a live call.

use std::path::{Path, PathBuf};

use stickler::borderline::{
    borderline_reading_of, load_borderline, BorderlineFlag, BorderlineReading, BorderlineSection,
};
use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::files::files_reading_of;
use stickler::json::parse;
use stickler::pattern::pattern_reading_of;
use stickler::render::{render_borderline, render_borderline_defects};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-borderline-{}-{}-{}",
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

fn run_borderline(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "borderline.json", text);
    let (code, out, _err) = run(&["borderline", &path]);
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
/// CK-23 and CK-12 share.
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

/// A diff holding one borderline line and no judgements,
/// the shared member default at line 4.
const DEFAULT_DIFF: &str =
    "--- a/app.py\n+++ b/app.py\n@@ -1,1 +1,2 @@\n+def plan(start, cache=([],)):\n";

/// A diff holding one judgement and no borderline lines.
const JUDGED_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -1,1 +1,2 @@\n+    print(total)\n";

/// A diff holding no borderline lines and no judgements.
const CLEAN_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -1,1 +1,2 @@\n+    logger.debug(total)\n";

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_borderline(&value),
        vec!["borderline must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_borderline(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(
        load_borderline(&value),
        vec!["sets must be a list".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_borderline(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_borderline(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_borderline(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

#[test]
fn the_empty_collection_renders_its_closings_alone() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_borderline(&value), Vec::<String>::new());
    assert_eq!(
        render_borderline(&borderline_reading_of(&value)),
        "Borderline\n\
         \n\
         Sets standing, 0\n\
         Borderline flags, 0\n\
         Sets read, 0\n"
    );
}

#[test]
fn a_set_holding_no_borderline_flag_stands_no_section() {
    let text = collection_json(&[
        (
            "quiet set",
            change_set_json(&[
                entry_json("judged.py", JUDGED_DIFF, ""),
                entry_json("clean.py", CLEAN_DIFF, ""),
            ]),
        ),
        (
            "loud set",
            change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let reading = borderline_reading_of(&value);
    assert_eq!(reading.sets_read, 2);
    assert_eq!(reading.sections.len(), 1);
    assert_eq!(reading.sections[0].label, "loud set");
    let rendered = render_borderline(&reading);
    assert_eq!(
        rendered,
        "Borderline\n\
         \n\
         Set, loud set\n\
         Borderline, 2\n\
         app.py, line 4, near placeholder name, data = load_entries(), drawn name entries\n\
         app.py, line 5, broad handler,     except BaseException:, handler names BaseException\n\
         \n\
         Sets standing, 1\n\
         Borderline flags, 2\n\
         Sets read, 2\n"
    );
    assert!(!rendered.contains("quiet set"), "{rendered}");
    assert!(!rendered.contains("judged.py"), "{rendered}");
}

#[test]
fn the_sections_and_flags_stand_in_collection_and_review_order() {
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[
                entry_json("zeta.py", ONE_FLAG_DIFF, ""),
                entry_json("mid.py", ONE_FLAG_DIFF, ""),
            ]),
        ),
        (
            "second set",
            change_set_json(&[entry_json("app.py", DEFAULT_DIFF, "")]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let reading = borderline_reading_of(&value);
    let labels: Vec<&str> = reading
        .sections
        .iter()
        .map(|section| section.label.as_str())
        .collect();
    assert_eq!(labels, vec!["first set", "second set"]);
    assert_eq!(
        reading.sections[0].flags,
        vec![
            BorderlineFlag {
                file: "zeta.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                content: "data = load_entries()".to_string(),
                reading: "drawn name entries".to_string(),
            },
            BorderlineFlag {
                file: "mid.py".to_string(),
                line: 4,
                pattern: "near placeholder name".to_string(),
                content: "data = load_entries()".to_string(),
                reading: "drawn name entries".to_string(),
            },
        ]
    );
    assert_eq!(
        reading.sections[1].flags,
        vec![BorderlineFlag {
            file: "app.py".to_string(),
            line: 4,
            pattern: "shared member default".to_string(),
            content: "def plan(start, cache=([],)):".to_string(),
            reading: "default holds a list literal".to_string(),
        }]
    );
}

#[test]
fn the_closings_stand_in_their_relations_over_a_hand_built_collection() {
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")]),
        ),
        (
            "quiet set",
            change_set_json(&[entry_json("app.py", JUDGED_DIFF, "")]),
        ),
        (
            "third set",
            change_set_json(&[entry_json("app.py", DEFAULT_DIFF, "")]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let reading = borderline_reading_of(&value);
    let flags: usize = reading.sections.iter().map(|s| s.flags.len()).sum();
    assert_eq!(flags, 3);
    assert_eq!(reading.sections.len(), 2);
    let census = census_of(&value);
    assert_eq!(flags, census.total.borderline);
    assert_eq!(flags, pattern_reading_of(&value).flags_read);
    assert_eq!(flags, files_reading_of(&value).flags_read);
    let standing = census
        .sets
        .iter()
        .filter(|set| set.tally.borderline > 0)
        .count();
    assert_eq!(reading.sections.len(), standing);
    let rendered = render_borderline(&reading);
    assert!(rendered.contains("Sets standing, 2\n"), "{rendered}");
    assert!(rendered.contains("Borderline flags, 3\n"), "{rendered}");
    assert!(rendered.contains("Sets read, 3\n"), "{rendered}");
}

#[test]
fn the_reading_is_deterministic() {
    let text = collection_json(&[(
        "only set",
        change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(
        render_borderline(&borderline_reading_of(&value)),
        render_borderline(&borderline_reading_of(&value))
    );
}

#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let (code, out) = run_borderline(&dir, "{\"sets\": [{\"change_set\": {\"files\": []}}]}");
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Borderline\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[0].label is missing\n"
    );
}

#[test]
fn a_malformed_borderline_rendering_names_its_defects() {
    let rendered = render_borderline_defects(&["sets is missing".to_string()]);
    assert_eq!(
        rendered,
        "Borderline\nVerdict, invalid\nDefects, 1\nDefect, sets is missing\n"
    );
}

#[test]
fn borderline_on_a_missing_file_is_malformed_with_exit_2() {
    let dir = temp_dir("gone");
    let missing = dir.join("gone.json").to_string_lossy().into_owned();
    let (code, out, _err) = run(&["borderline", &missing]);
    assert_eq!(code, 2);
    assert!(out.contains("Defect, the borderline file could not be read\n"));
}

#[test]
fn borderline_on_a_non_json_file_is_malformed_with_exit_2() {
    let dir = temp_dir("nonjson");
    let (code, out) = run_borderline(&dir, "not json");
    assert_eq!(code, 2);
    assert!(out.contains("Defect, the file does not hold a JSON borderline collection\n"));
}

/// The census collection, read whole. The reading over
/// examples/census.json holds two sections, the pilot
/// set holding twelve borderline flags and the
/// constructed round 4 example holding six, the round 2
/// set holding none and standing no section, and the
/// closings stand in their relations to the census,
/// pattern, and files closings over the same collection.
#[test]
fn the_census_collection_reads_with_its_sections_and_closings() {
    let text = std::fs::read_to_string(examples().join("census.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_borderline(&value), Vec::<String>::new());
    let reading = borderline_reading_of(&value);
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.sections.len(), 2);
    assert_eq!(reading.sections[0].label, "pilot set");
    assert_eq!(reading.sections[0].flags.len(), 12);
    assert_eq!(reading.sections[1].label, "constructed round 4 example");
    assert_eq!(reading.sections[1].flags.len(), 6);
    let flags: usize = reading.sections.iter().map(|s| s.flags.len()).sum();
    assert_eq!(flags, 18);
    assert_eq!(flags, census_of(&value).total.borderline);
    assert_eq!(flags, pattern_reading_of(&value).flags_read);
    assert_eq!(flags, files_reading_of(&value).flags_read);
    let rendered = render_borderline(&reading);
    assert!(rendered.contains("Sets standing, 2\n"), "{rendered}");
    assert!(rendered.contains("Borderline flags, 18\n"), "{rendered}");
    assert!(rendered.contains("Sets read, 3\n"), "{rendered}");
    assert!(!rendered.contains("round 2 set"), "{rendered}");
}

#[test]
fn the_example_matches_its_golden_through_the_command() {
    let path = examples().join("borderline.json");
    let (code, out, _err) = run(&["borderline", path.to_str().unwrap()]);
    assert_eq!(code, 0);
    let golden = std::fs::read_to_string(examples().join("borderline.txt")).unwrap();
    assert_eq!(out, golden);
}

#[test]
fn the_example_matches_its_golden_through_the_library() {
    let text = std::fs::read_to_string(examples().join("borderline.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_borderline(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("borderline.txt")).unwrap();
    assert_eq!(render_borderline(&borderline_reading_of(&value)), golden);
}

/// The fencing of FIX-CK-23 on the borderline reading,
/// over a reading built by hand. A set label holding a
/// line break and a closing count line's own text, a
/// file name holding a break, a pattern holding a
/// break, and the flagged line and the shape reading
/// each holding a break and a closing line's own text
/// each print as the empty field, so no section line,
/// flag line, or closing line is forged. Before the
/// closure a handed reading printed each raw, as the
/// set review rendering printed its judgement, flag,
/// and verdict fields before FIX-CK-23.
#[test]
fn a_line_break_in_a_hand_built_reading_forges_no_section_flag_or_closing_line() {
    let reading = BorderlineReading {
        sections: vec![BorderlineSection {
            label: "one set\nSets read, 99".to_string(),
            flags: vec![BorderlineFlag {
                file: "evil.py\nBorderline, 99".to_string(),
                line: 4,
                pattern: "near placeholder name\nSets standing, 99".to_string(),
                content: "content\nBorderline flags, 99".to_string(),
                reading: "reading\nSets read, 99".to_string(),
            }],
        }],
        sets_read: 1,
    };
    let rendered = render_borderline(&reading);
    assert!(rendered.lines().any(|line| line == "Set, "), "{rendered}");
    assert!(
        rendered.lines().any(|line| line == ", line 4, , , "),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Sets standing, 1"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Borderline flags, 1"),
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
    assert!(
        !rendered.lines().any(|line| line == "Borderline, 99"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Sets standing, 99"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Borderline flags, 99"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}
