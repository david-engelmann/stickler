//! Pins for CK-31 and CK-32, the choices file and
//! reading.
//!
//! The choices pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per outcome in the
//! order CK-28 states the outcome counts, every
//! outcome standing whether any verdict stands under
//! it or not, the flag groups holding only the pairs
//! of their section's outcome, the pairs in the
//! standing set review form, the no verdicts line
//! where no verdict stands under an outcome, and the
//! closing counts in their stated relations to the
//! sections and to the verdicts and census closings
//! over the same collection. The loader pins name
//! each defect of the file in the forms CK-12 states,
//! one pin per defect. The fencing pins hold the
//! rendering to the one line rule at the library
//! surface, the reading standing born inside the
//! fencing programme. Every pin runs offline on
//! recorded and constructed entries, and no pin makes
//! a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent
//! against the tree Round 14 closed on, by the
//! absence of the module it names, and within the
//! slice each pin discriminates against the wrong
//! build its own comment names. The golden pin
//! discriminates by content against any build whose
//! counts, order, sections, or pairs differ. The
//! verdicts binding pin fails on a build whose
//! counts travel a second outcome path, and the
//! census binding pin on a build whose counts travel
//! a second flagging path. The sum pin discriminates
//! by content on the rendered sums. The empty section
//! and empty collection pins fail on a build that
//! stands only the outcomes that raised. The cross
//! set order pin fails on a build that gathers by
//! set completion order or reverses the collection.
//! The split flag pin fails on a build that assigns
//! a flag to one section whole. The abstention beside
//! label pin is the FIX-CK-28 discrimination stated
//! at birth, failing on any build that assigns by
//! the choice word rather than by the predicate and
//! the choice together, and the word abstained pin
//! fails on a build that seats the word in the
//! abstained section or drops it from the builder's
//! count. The same label pin fails on a build that
//! states one pair per flag per section. The loader
//! pins each fail on a loader that accepts the
//! defect they name. The fencing pins each fail on a
//! raw print or a handed count in their position.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::choices::{
    choices_reading_of, load_choices, ChoiceFlagGroup, ChoicesReading, ChoicesSection,
};
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::render::{render_choices, render_verdicts};
use stickler::verdicts::VerdictEntry;
use stickler::verdicts_reading::verdicts_reading_of;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-choices-{}-{}-{}",
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

fn run_choices(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "choices.json", text);
    let (code, out, _err) = run(&["choices", &path]);
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
/// CK-31 and CK-12 share.
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

/// The verdicts read counts of a choices rendering in
/// standing order, the three section closing counts and
/// the closing verdicts read count standing last.
fn verdicts_read_counts(text: &str) -> Vec<usize> {
    text.lines()
        .filter(|line| line.starts_with("Verdicts read, "))
        .map(|line| {
            line.strip_prefix("Verdicts read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect()
}

#[test]
fn the_example_matches_its_golden_through_the_library() {
    let text = std::fs::read_to_string(examples().join("choices.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_choices(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("choices.txt")).unwrap();
    assert_eq!(render_choices(&choices_reading_of(&value)), golden);
}

/// The shipped collection, read whole. Each of the
/// seven closing counts of the choices reading, three
/// sets read, eighteen flags read, seventeen flags
/// with verdicts, thirty two verdicts read, twenty
/// four chose stickler, five chose clean, and three
/// abstained, equals the same named count of the
/// verdicts reading over the same collection, both
/// readings run here, so a build whose counts travel
/// a second outcome path fails this pin wherever the
/// two organizations of the same pairs disagree.
#[test]
fn the_closings_bind_to_the_verdicts_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("choices.json")).unwrap();
    let value = parse(&text).unwrap();
    let choices = render_choices(&choices_reading_of(&value));
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let choices_closings = choices.split("\n\n").last().unwrap();
    let verdicts_closings = verdicts.split("\n\n").last().unwrap();
    for (prefix, expected) in [
        ("Sets read, ", 3),
        ("Flags read, ", 18),
        ("Flags with verdicts, ", 17),
        ("Verdicts read, ", 32),
        ("Chose stickler, ", 24),
        ("Chose clean, ", 5),
        ("Abstained, ", 3),
    ] {
        assert_eq!(count_after(choices_closings, prefix), expected, "{prefix}");
        assert_eq!(
            count_after(choices_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
}

/// The closings of the choices reading over the
/// shipped collection stand in their relations to the
/// census over the same collection, the flags read
/// count the census total block's flagged lines and
/// the sets read count the count of the sets the
/// census reads.
#[test]
fn the_closings_bind_to_the_census_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("choices.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = choices_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.sets_read, census.sets.len());
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.flags_read, census.total.borderline);
    assert_eq!(reading.flags_read, 18);
    let rendered = render_choices(&reading);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
}

/// The three outcome counts of the shipped rendering
/// sum to its closing verdicts read count, and the
/// section closing counts sum to the same, read from
/// the rendered text.
#[test]
fn the_outcome_counts_sum_to_the_verdicts_read_count() {
    let text = std::fs::read_to_string(examples().join("choices.json")).unwrap();
    let value = parse(&text).unwrap();
    let rendered = render_choices(&choices_reading_of(&value));
    let counts = verdicts_read_counts(&rendered);
    assert_eq!(counts, vec![24, 5, 3, 32]);
    assert_eq!(counts[..3].iter().sum::<usize>(), counts[3]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// A constructed one set collection whose verdicts
/// chose stickler only renders the chose clean and
/// abstained sections standing, each stating the no
/// verdicts line and closing at zero.
#[test]
fn a_collection_under_one_choice_stands_the_other_two_sections_empty() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            FLAGS_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.7}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_choices(&value), Vec::<String>::new());
    assert_eq!(
        render_choices(&choices_reading_of(&value)),
        "Choices\n\
         \n\
         Chose stickler\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.9\n\
         app.py, line 5, broad handler, set, one set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.7\n\
         Verdicts read, 2\n\
         \n\
         Chose clean\n\
         No verdicts\n\
         Verdicts read, 0\n\
         \n\
         Abstained\n\
         No verdicts\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 1\n\
         Flags read, 2\n\
         Flags with verdicts, 2\n\
         Verdicts read, 2\n\
         Chose stickler, 2\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}

/// An empty collection renders the head line, the
/// three sections each stating the no verdicts line
/// and closing at zero, and the closing counts at
/// zero.
#[test]
fn an_empty_collection_renders_every_section_empty_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_choices(&value), Vec::<String>::new());
    assert_eq!(
        render_choices(&choices_reading_of(&value)),
        "Choices\n\
         \n\
         Chose stickler\n\
         No verdicts\n\
         Verdicts read, 0\n\
         \n\
         Chose clean\n\
         No verdicts\n\
         Verdicts read, 0\n\
         \n\
         Abstained\n\
         No verdicts\n\
         Verdicts read, 0\n\
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

/// Over a constructed two set collection where one
/// choice carries a pair in each set, the section
/// lists the first set's flag group before the second
/// set's, in collection order.
#[test]
fn a_choice_section_lists_flag_groups_in_collection_order_across_sets() {
    let one = ONE_FLAG_DIFF.replace("app.py", "one.py");
    let two = ONE_FLAG_DIFF.replace("app.py", "two.py");
    let verdicts =
        "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]";
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[entry_json("one.py", &one, verdicts)]),
        ),
        (
            "second set",
            change_set_json(&[entry_json("two.py", &two, verdicts)]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let rendered = render_choices(&choices_reading_of(&value));
    let first = rendered
        .lines()
        .position(|line| line == "one.py, line 4, near placeholder name, set, first set")
        .unwrap();
    let second = rendered
        .lines()
        .position(|line| line == "two.py, line 4, near placeholder name, set, second set")
        .unwrap();
    assert!(first < second, "{rendered}");
}

/// Over a constructed contested flag, one verdict
/// choosing stickler and one choosing clean from the
/// two models, the flag stands in both label
/// sections, its group in each holding only that
/// section's one pair and neither holding the other's.
#[test]
fn a_contested_flag_stands_in_both_label_sections_with_only_its_own_pair_in_each() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_choices(&value), Vec::<String>::new());
    assert_eq!(
        render_choices(&choices_reading_of(&value)),
        "Choices\n\
         \n\
         Chose stickler\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.9\n\
         Verdicts read, 1\n\
         \n\
         Chose clean\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, openai/gpt-6-luna, clean\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.8\n\
         Verdicts read, 1\n\
         \n\
         Abstained\n\
         No verdicts\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 2\n\
         Chose stickler, 1\n\
         Chose clean, 1\n\
         Abstained, 0\n"
    );
}

/// Over a constructed flag carrying an abstained
/// verdict whose choice word is stickler beside a
/// standing clean verdict, the flag stands in the
/// abstained section with the one abstained pair and
/// in the chose clean section with the one clean
/// pair, and stands in the chose stickler section not
/// at all. This is the FIX-CK-28 discrimination
/// stated at birth, and it fails on any build that
/// assigns by the choice word rather than by the
/// predicate and the choice together.
#[test]
fn an_abstention_beside_a_label_seats_by_the_predicate_and_not_the_word() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.32},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.83}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_choices(&value), Vec::<String>::new());
    assert_eq!(
        render_choices(&choices_reading_of(&value)),
        "Choices\n\
         \n\
         Chose stickler\n\
         No verdicts\n\
         Verdicts read, 0\n\
         \n\
         Chose clean\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, openai/gpt-6-luna, clean\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.83\n\
         Verdicts read, 1\n\
         \n\
         Abstained\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained\n\
         Verdicts read, 1\n\
         \n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 2\n\
         Chose stickler, 0\n\
         Chose clean, 1\n\
         Abstained, 1\n"
    );
}

/// A verdict entry carrying the word abstained as its
/// choice at a confidence of nine tenths, no
/// abstention under CK-7, built into a reading through
/// the builder over a hand built collection, stands in
/// no section, its pair printing nowhere, and counts
/// in the builder's verdicts read count and in none
/// of the three outcome counts. The change set loader
/// refuses the word by name, so the pin reads the
/// builder's surface and not the loader's, in the
/// posture CK-32 states for the handed shape. It
/// fails on a build that seats the word in the
/// abstained section or drops it from the count.
#[test]
fn the_word_abstained_at_no_abstention_confidence_stands_in_no_section() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"abstained\",\"confidence\":0.9}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = choices_reading_of(&value);
    assert_eq!(reading.verdicts_read, 1);
    assert_eq!(reading.flags_read, 1);
    assert_eq!(reading.flags_with_verdicts, 1);
    assert_eq!(reading.sections.len(), 3);
    assert!(reading
        .sections
        .iter()
        .all(|section| section.groups.is_empty()));
    let rendered = render_choices(&reading);
    assert!(
        !rendered.lines().any(|line| line.starts_with("Verdict, ")),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line.starts_with("Reason, ")),
        "{rendered}"
    );
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Chose stickler, "), 0);
    assert_eq!(count_after(closings, "Chose clean, "), 0);
    assert_eq!(count_after(closings, "Abstained, "), 0);
    assert_eq!(count_after(closings, "Verdicts read, "), 0);
}

/// Over a constructed flag carrying two verdicts from
/// the two models both choosing stickler, the chose
/// stickler section holds one flag group for that
/// flag with both pairs standing in verdict order.
#[test]
fn two_verdicts_choosing_the_same_label_stand_as_two_pairs_in_one_group() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"stickler\",\"confidence\":0.85}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_choices(&value), Vec::<String>::new());
    let reading = choices_reading_of(&value);
    assert_eq!(reading.sections[0].outcome, "stickler");
    assert_eq!(reading.sections[0].groups.len(), 1);
    let group = &reading.sections[0].groups[0];
    assert_eq!(group.verdicts.len(), 2);
    assert_eq!(group.verdicts[0].model, "jev-1.13.0");
    assert_eq!(group.verdicts[1].model, "openai/gpt-6-luna");
    assert!(reading.sections[1].groups.is_empty());
    assert!(reading.sections[2].groups.is_empty());
    let rendered = render_choices(&reading);
    assert_eq!(verdicts_read_counts(&rendered), vec![2, 0, 0, 2]);
}

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["choices must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_choices(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets must be a list".to_string()]
    );
}

#[test]
fn an_entry_that_is_not_an_object_is_refused() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_that_is_empty_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_missing_change_set_field_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_choices(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the last set of a collection renders
/// the defect list and no reading, whole or nothing,
/// so no partial choices reading ever prints.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")])
    );
    let (code, out) = run_choices(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Choices\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// Build one verdict entry by hand in the family's
/// form, its confidence parsed from JSON text so it
/// stands in the seat's number form.
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

/// Build one flag group by hand in the family's form.
fn handed_group(file: &str, set_label: &str, verdicts: Vec<VerdictEntry>) -> ChoiceFlagGroup {
    ChoiceFlagGroup {
        file: file.to_string(),
        line: 4,
        pattern: "near placeholder name".to_string(),
        set_label: set_label.to_string(),
        verdicts,
    }
}

/// A file name holding a break and a verdicts read
/// line's own text prints as the empty field, so no
/// flag group line or closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_file_forges_no_closing_line() {
    let reading = ChoicesReading {
        sections: vec![ChoicesSection {
            outcome: "stickler".to_string(),
            groups: vec![handed_group(
                "evil.py\nVerdicts read, 99",
                "one set",
                vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        flags_with_verdicts: 1,
        verdicts_read: 1,
    };
    let rendered = render_choices(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == ", line 4, near placeholder name, set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Verdicts read, 1"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// A set label holding a break and a closing line's
/// own text prints as the empty field in the flag
/// group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_set_label_forges_no_closing_line() {
    let reading = ChoicesReading {
        sections: vec![ChoicesSection {
            outcome: "stickler".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set\nSets read, 99",
                vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        flags_with_verdicts: 1,
        verdicts_read: 1,
    };
    let rendered = render_choices(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, line 4, near placeholder name, set, "),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Sets read, 99"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Sets read, 1"),
        "{rendered}"
    );
}

/// A model holding a break and a verdict line's own
/// text prints as the empty field in its pair's first
/// line, and the constructed reason built from it
/// prints fenced whole in the second line, so no
/// verdict line is forged.
#[test]
fn a_line_break_in_a_model_is_fenced_in_its_pairs_first_line() {
    let reading = ChoicesReading {
        sections: vec![ChoicesSection {
            outcome: "stickler".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                vec![handed_entry(
                    "jev-1.13.0\nVerdict, forged, clean",
                    "stickler",
                    "0.9",
                )],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        flags_with_verdicts: 1,
        verdicts_read: 1,
    };
    let rendered = render_choices(&reading);
    assert!(
        rendered.lines().any(|line| line == "Verdict, , stickler"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Reason, "),
        "{rendered}"
    );
    assert!(!rendered.contains("forged"), "{rendered}");
}

/// A choice holding a break, carried into the
/// constructed reason of its pair, prints fenced in
/// the pair's first line and the reason fenced whole
/// in the second line, so no closing line is forged
/// by either.
#[test]
fn a_line_break_in_a_constructed_reason_is_fenced_whole_in_its_pairs_second_line() {
    let reading = ChoicesReading {
        sections: vec![ChoicesSection {
            outcome: "clean".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                vec![handed_entry("jev-1.13.0", "clean\nChose clean, 99", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        flags_with_verdicts: 1,
        verdicts_read: 1,
    };
    let rendered = render_choices(&reading);
    assert!(
        rendered.lines().any(|line| line == "Verdict, jev-1.13.0, "),
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
        rendered.lines().any(|line| line == "Chose clean, 1"),
        "{rendered}"
    );
}

/// A section outcome word holding a break and a
/// structural word prints fenced in its head, so the
/// head mints no closing line, and the section's
/// pairs count in the verdicts read count and in
/// none of the three outcome counts, the posture
/// CK-32 states for a handed section outside the
/// three outcome words.
#[test]
fn a_line_break_in_a_section_outcome_word_is_fenced_in_its_head() {
    let reading = ChoicesReading {
        sections: vec![ChoicesSection {
            outcome: "stickler\nVerdicts read, 99".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        flags_with_verdicts: 1,
        verdicts_read: 1,
    };
    let rendered = render_choices(&reading);
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "Verdict, jev-1.13.0, stickler"),
        "{rendered}"
    );
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Verdicts read, "), 1);
    assert_eq!(count_after(closings, "Chose stickler, "), 0);
    assert_eq!(count_after(closings, "Chose clean, "), 0);
    assert_eq!(count_after(closings, "Abstained, "), 0);
}

/// The closing verdicts read count prints as the sum
/// over the sections the rendering states, so a
/// handed reading whose stored verdicts count
/// disagrees with its sections states the summed
/// count and no handed count can print against the
/// sections.
#[test]
fn the_closing_verdicts_count_prints_as_the_sum_over_the_sections() {
    let reading = ChoicesReading {
        sections: vec![
            ChoicesSection {
                outcome: "stickler".to_string(),
                groups: vec![handed_group(
                    "app.py",
                    "one set",
                    vec![
                        handed_entry("jev-1.13.0", "stickler", "0.9"),
                        handed_entry("openai/gpt-6-luna", "stickler", "0.8"),
                    ],
                )],
            },
            ChoicesSection {
                outcome: "clean".to_string(),
                groups: vec![handed_group(
                    "app.py",
                    "one set",
                    vec![handed_entry("jev-1.13.0", "clean", "0.7")],
                )],
            },
        ],
        sets_read: 1,
        flags_read: 1,
        flags_with_verdicts: 2,
        verdicts_read: 99,
    };
    let rendered = render_choices(&reading);
    assert_eq!(verdicts_read_counts(&rendered), vec![2, 1, 3]);
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Chose stickler, "), 2);
    assert_eq!(count_after(closings, "Chose clean, "), 1);
}
