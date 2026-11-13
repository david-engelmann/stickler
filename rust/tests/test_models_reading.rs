//! Pins for CK-45 and CK-46, the models file and
//! reading.
//!
//! The models pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per model in first
//! appearance order, a flag carrying no recorded
//! verdict standing in no section, each flag group
//! holding the one pair of its section's model where
//! the standing pair readings hold the flag's pairs
//! whole, the pairs in the standing set review form,
//! and the closing counts in their stated relations
//! to the sections and to the verdicts reading and
//! the census over the same collection. The loader
//! pins name each defect of the file in the forms
//! CK-12 states, one pin per defect. The fencing pins
//! hold the rendering to the one line rule at the
//! library surface, the six handed fields of this
//! reading, the section head model, the flag line's
//! file name, pattern, and set label, the model in
//! the pair's first line, and the constructed reason
//! in its second line, each printing through the one
//! helper, the reading standing born inside the
//! fencing programme. Every pin runs offline on
//! recorded and constructed entries, and no pin
//! makes a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent
//! at base, against the tree Round 22 closed on, by
//! the absence of the module it names, and within
//! the slice each pin discriminates against the
//! wrong build its own comment names. The golden
//! pin catches the per set gathering mutation MB,
//! both shipped sections losing their constructed
//! set pairs and closing at twelve and ten against
//! the golden's seventeen and fifteen, the model and
//! set keying mutation MC, the shipped collection
//! standing four sections under it against the
//! golden's two, and the whole group seating
//! mutation MG, the shipped ledger.py line 6 and
//! cache.py line 15 groups carrying both labels
//! whole under their first verdict's model against
//! the golden's split. It passes under the section
//! sort mutation MA by design, the shipped
//! collection standing jev-1.13.0 before
//! openai/gpt-6-luna in first appearance order and
//! in alphabetical order alike, and under the
//! stored count mutation MF by design, the stored
//! and summed counts agreeing there at thirty two.
//! The closings binding pin passes under MG, its
//! closings reading the same seven counts the
//! verdicts reading states under either seating.
//! The flag groups binding pin catches MG over the
//! same collection. The sums pin passes under MA,
//! MC, and MG, every pair still standing exactly
//! once under each, so its sums hold, and it passes
//! under MF, its sums reading the sections and not
//! the stored closing. The one model pin states the
//! seating of a collection whose verdicts name one
//! model alone. The cross set order pin catches MB
//! and MC in pure form, its model's groups standing
//! in one section across its two sets. The two
//! models pin catches MG in pure form, its flag
//! standing in both sections with the one pair of
//! each. The no verdict pin fails on a build that
//! stands a verdictless flag as a group. The
//! section order pin catches MA in its pure form,
//! its fixture parting first appearance from
//! alphabetical order. The abstention only pin
//! fails on a build that seats a flag's pairs by
//! outcome or drops an all abstention pair, which
//! has no outcome section to stand in. The handed
//! word abstained pin does not catch any builder
//! mutation, the pin hand building its reading and
//! calling only the renderer, so no builder edit
//! can reach it, and it holds at the library
//! surface against a build that drops the pair or
//! seats it by its word there. The verdicts
//! collection pin carries the cross binding figures
//! in full, seventy two and fifty eight, and the
//! section outcome splits beside them. The loader
//! pins each fail on a loader that accepts the
//! defect they name, and the whole or nothing pin
//! catches the dropped whole or nothing mutation
//! MD in pure form alone, its defective entry
//! standing at index 1 behind a sound first entry,
//! and the other loader pins pass under MD, their
//! defects standing at the head of the value or in
//! the first entry. The fencing pins each fail on a
//! raw print in their position, the section head
//! pin catching the dropped helper mutation ME in
//! pure form at its position and the other five
//! passing under ME, their positions keeping the
//! helper. The closing counts pin catches MF,
//! under which the closing verdicts read count
//! prints from a stored count rather than the sum
//! over the sections the rendering states.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::models::{
    load_models, models_reading_of, ModelsFlagGroup, ModelsReading, ModelsSection,
};
use stickler::render::{render_models, render_verdicts};
use stickler::verdicts::{is_abstention, verdict_outcome, VerdictEntry};
use stickler::verdicts_reading::verdicts_reading_of;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-models-{}-{}-{}",
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

fn run_models(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "models.json", text);
    let (code, out, _err) = run(&["models", &path]);
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
/// CK-45 and CK-12 share.
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

/// The verdicts read counts of a models rendering in
/// standing order, the section closing counts and the
/// closing verdicts read count standing last.
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
    let text = std::fs::read_to_string(examples().join("models.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_models(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("models.txt")).unwrap();
    assert_eq!(render_models(&models_reading_of(&value)), golden);
}

/// The shipped collection, read whole. Each of the
/// seven closing counts the models reading shares
/// with the verdicts reading, three sets read,
/// eighteen flags read, seventeen flags with
/// verdicts, thirty two verdicts read, twenty four
/// chose stickler, five chose clean, and three
/// abstained, equals the same named count of the
/// verdicts reading over the same collection, both
/// readings run here, so a build whose counts travel
/// a second path fails this pin wherever the two
/// organizations of the same pairs disagree.
#[test]
fn the_closings_bind_to_the_verdicts_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("models.json")).unwrap();
    let value = parse(&text).unwrap();
    let models = render_models(&models_reading_of(&value));
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let models_closings = models.split("\n\n").last().unwrap();
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
        assert_eq!(count_after(models_closings, prefix), expected, "{prefix}");
        assert_eq!(
            count_after(models_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
}

/// The shipped collection, read whole. The flag
/// groups of the models reading are exactly the
/// pairs of the verdicts reading regrouped by the
/// model each pair names, one group per pair, each
/// group holding the one pair of its section's model
/// where the verdicts reading holds the flag's pairs
/// whole, in the same collection order within each
/// section, both readings run here, so a build that
/// seats a flag's pairs whole under one model fails
/// this pin wherever a flag carries two models.
#[test]
fn the_flag_groups_bind_to_the_verdicts_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("models.json")).unwrap();
    let value = parse(&text).unwrap();
    let models = models_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let mut expected: Vec<(String, String, usize, String, String)> = Vec::new();
    for section in &verdicts.sections {
        for group in &section.groups {
            for entry in &group.verdicts {
                expected.push((
                    entry.model.clone(),
                    group.file.clone(),
                    group.line,
                    group.pattern.clone(),
                    section.label.clone(),
                ));
            }
        }
    }
    let mut stated: Vec<(String, String, usize, String, String)> = Vec::new();
    for section in &models.sections {
        for group in &section.groups {
            assert_eq!(group.verdict.model, section.model);
            stated.push((
                section.model.clone(),
                group.file.clone(),
                group.line,
                group.pattern.clone(),
                group.set_label.clone(),
            ));
        }
    }
    assert_eq!(stated.len(), 32);
    let mut sorted_expected = expected.clone();
    let mut sorted_stated = stated.clone();
    sorted_expected.sort();
    sorted_stated.sort();
    assert_eq!(sorted_stated, sorted_expected);
    for section in &models.sections {
        let in_order: Vec<_> = expected
            .iter()
            .filter(|held| held.0 == section.model)
            .cloned()
            .collect();
        let mine: Vec<_> = stated
            .iter()
            .filter(|held| held.0 == section.model)
            .cloned()
            .collect();
        assert_eq!(mine, in_order, "section {}", section.model);
    }
    let groups_over_sections: usize = models.sections.iter().map(|s| s.groups.len()).sum();
    assert_eq!(groups_over_sections, 32);
}

/// The closings of the models reading over the
/// shipped collection stand in their relations to
/// the census over the same collection, the flags
/// read count the census total block's flagged lines
/// and the sets read count the count of the sets the
/// census reads, read from the builder's counts and
/// from the rendered text alike.
#[test]
fn the_closings_bind_to_the_census_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("models.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = models_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.sets_read, census.sets.len());
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.flags_read, census.total.borderline);
    assert_eq!(reading.flags_read, 18);
    let rendered = render_models(&reading);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
}

/// The two section closing counts of the shipped
/// rendering sum to its closing verdicts read count,
/// and the three outcome counts sum to the same,
/// the sums computed with the builder built counts
/// beside the rendered text.
#[test]
fn the_section_counts_sum_to_the_closing_verdicts_read_count() {
    let text = std::fs::read_to_string(examples().join("models.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = models_reading_of(&value);
    let built: usize = reading.sections.iter().map(|s| s.groups.len()).sum();
    assert_eq!(built, reading.verdicts_read);
    assert_eq!(reading.verdicts_read, 32);
    let rendered = render_models(&reading);
    let counts = verdicts_read_counts(&rendered);
    assert_eq!(counts, vec![17, 15, 32]);
    assert_eq!(counts[..2].iter().sum::<usize>(), counts[2]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// A constructed one set collection whose verdicts
/// all name one model stands that model alone, the
/// other file of the collection, carrying flags with
/// no verdicts, standing in no section, and its flags
/// counting in the closing flags read count alone.
#[test]
fn a_collection_with_verdicts_of_one_model_stands_that_model_alone() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.8}]",
            ),
            entry_json("other.py", FLAGS_DIFF, ""),
        ]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_models(&value), Vec::<String>::new());
    assert_eq!(
        render_models(&models_reading_of(&value)),
        "Models\n\
         \n\
         Model, jev-1.13.0\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, clean\n\
         Reason, jev-1.13.0 chose clean with confidence 0.8\n\
         Verdicts read, 1\n\
         \n\
         Sets read, 1\n\
         Flags read, 3\n\
         Flags with verdicts, 1\n\
         Verdicts read, 1\n\
         Chose stickler, 0\n\
         Chose clean, 1\n\
         Abstained, 0\n"
    );
}

/// An empty collection renders the head line and the
/// seven closing counts at zero, and no section
/// stands.
#[test]
fn an_empty_collection_renders_no_section_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_models(&value), Vec::<String>::new());
    assert_eq!(
        render_models(&models_reading_of(&value)),
        "Models\n\
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
/// model carries verdicts in both sets, the model
/// stands one section, its first set's group before
/// its second set's, in collection order. It
/// catches the per set gathering mutation and the
/// model and set keying mutation in pure form.
#[test]
fn a_model_section_lists_groups_in_collection_order_across_sets() {
    let verdicts =
        "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]";
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, verdicts)]),
        ),
        (
            "second set",
            change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, verdicts)]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let rendered = render_models(&models_reading_of(&value));
    assert_eq!(
        rendered.matches("Model, jev-1.13.0").count(),
        1,
        "{rendered}"
    );
    let first = rendered
        .lines()
        .position(|line| line == "app.py, line 4, near placeholder name, set, first set")
        .unwrap();
    let second = rendered
        .lines()
        .position(|line| line == "app.py, line 4, near placeholder name, set, second set")
        .unwrap();
    assert!(first < second, "{rendered}");
}

/// A constructed flag carrying verdicts of both
/// models stands as a group in both sections, each
/// group holding the one pair of its section's
/// model, and the flag groups over the sections
/// count to the verdicts read count and not to the
/// flags with verdicts count. It catches the whole
/// group seating mutation in pure form.
#[test]
fn a_flag_carrying_two_models_stands_in_both_sections_with_the_one_pair_of_each() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(
        render_models(&models_reading_of(&value)),
        "Models\n\
         \n\
         Model, jev-1.13.0\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.9\n\
         Verdicts read, 1\n\
         \n\
         Model, openai/gpt-6-luna\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, openai/gpt-6-luna, clean\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.8\n\
         Verdicts read, 1\n\
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

/// A constructed collection whose flags carry no
/// recorded verdicts renders the empty form, no
/// model standing in it, while the closing flags
/// read count counts its flags.
#[test]
fn a_flag_carrying_no_verdicts_stands_in_no_section() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", FLAGS_DIFF, "")]),
    )]);
    let value = parse(&text).unwrap();
    let reading = models_reading_of(&value);
    assert!(reading.sections.is_empty());
    assert_eq!(
        render_models(&reading),
        "Models\n\
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

/// Over a constructed collection whose first pair
/// names openai/gpt-6-luna and whose second names
/// jev-1.13.0, the sections stand in that first
/// appearance order, and not in alphabetical order,
/// which would stand jev-1.13.0 first.
#[test]
fn the_sections_stand_in_first_appearance_order_not_alphabetical_order() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "windows.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            ),
            entry_json(
                "harvest.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            ),
        ]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_models(&models_reading_of(&value));
    let luna = rendered
        .lines()
        .position(|line| line == "Model, openai/gpt-6-luna")
        .unwrap();
    let jev = rendered
        .lines()
        .position(|line| line == "Model, jev-1.13.0")
        .unwrap();
    assert!(luna < jev, "{rendered}");
}

/// A constructed flag carrying only abstentions
/// stands as a group in the section of each of its
/// models, its pairs printing in the abstained form
/// of the pair's first line, so a seating by outcome,
/// under which an all abstention pair has no outcome
/// section to stand in, drops it or seats it under a
/// label and fails here.
#[test]
fn a_flag_carrying_only_abstentions_stands_in_the_section_of_each_of_its_models_with_its_pairs_in_the_abstained_form(
) {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.3},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.4}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(
        render_models(&models_reading_of(&value)),
        "Models\n\
         \n\
         Model, jev-1.13.0\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.3 and abstained\n\
         Verdicts read, 1\n\
         \n\
         Model, openai/gpt-6-luna\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, openai/gpt-6-luna, abstained\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.4 and abstained\n\
         Verdicts read, 1\n\
         \n\
         Sets read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 2\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 2\n"
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
fn handed_group(file: &str, set_label: &str, verdict: VerdictEntry) -> ModelsFlagGroup {
    ModelsFlagGroup {
        file: file.to_string(),
        line: 4,
        pattern: "near placeholder name".to_string(),
        set_label: set_label.to_string(),
        verdict,
    }
}

/// A hand built reading whose one flag carries a
/// verdict whose choice stands as the word abstained
/// at a confidence that is no abstention renders that
/// flag's group in its model's section with the pair
/// printing the choice as written, the pair counting
/// in the reading's verdicts read count and in none
/// of its three outcome counts, in the posture
/// FIX-CK-28 states for handed choices outside the
/// two labels, because its section is its pair's
/// model and not its own. A build that seats by the
/// verdict, through the outcome word the verdict
/// carries, drops the pair or seats it by its word
/// and fails here.
#[test]
fn a_handed_word_abstained_verdict_stands_in_its_models_section() {
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "jev-1.13.0".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                handed_entry("jev-1.13.0", "abstained", "0.9"),
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
    assert!(
        rendered.lines().any(|line| line == "Model, jev-1.13.0"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, line 4, near placeholder name, set, one set"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "Verdict, jev-1.13.0, abstained"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "Reason, jev-1.13.0 chose abstained with confidence 0.9"),
        "{rendered}"
    );
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Flags with verdicts, "), 1);
    assert_eq!(count_after(closings, "Chose stickler, "), 0);
    assert_eq!(count_after(closings, "Chose clean, "), 0);
    assert_eq!(count_after(closings, "Abstained, "), 0);
}

/// Over the constructed verdicts collection, the
/// cross binding material of the verdicts reading,
/// the models reading stands as the transcription of
/// the design states, the jev-1.13.0 section carrying
/// seventy two flag groups and closing at Verdicts
/// read, 72, of outcomes thirty seven chose stickler,
/// thirty one chose clean, and four abstained, the
/// openai/gpt-6-luna section carrying fifty eight
/// flag groups and closing at Verdicts read, 58, of
/// outcomes fifty three chose stickler, one chose
/// clean, and four abstained, and the closings
/// standing at the verdicts reading's own figures
/// over the same collection.
#[test]
fn the_models_over_the_verdicts_collection_bind_to_the_verdicts_reading() {
    let text = std::fs::read_to_string(examples().join("verdicts_collection.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_models(&value), Vec::<String>::new());
    let reading = models_reading_of(&value);
    assert_eq!(reading.sections.len(), 2);
    assert_eq!(reading.sections[0].model, "jev-1.13.0");
    assert_eq!(reading.sections[0].groups.len(), 72);
    assert_eq!(reading.sections[1].model, "openai/gpt-6-luna");
    assert_eq!(reading.sections[1].groups.len(), 58);
    let splits: Vec<(usize, usize, usize)> = reading
        .sections
        .iter()
        .map(|section| {
            let mut counts = (0, 0, 0);
            for group in &section.groups {
                if is_abstention(&group.verdict) {
                    counts.2 += 1;
                } else {
                    match verdict_outcome(&group.verdict) {
                        "stickler" => counts.0 += 1,
                        "clean" => counts.1 += 1,
                        _ => {}
                    }
                }
            }
            counts
        })
        .collect();
    assert_eq!(splits, vec![(37, 31, 4), (53, 1, 4)]);
    let rendered = render_models(&reading);
    assert_eq!(verdicts_read_counts(&rendered), vec![72, 58, 130]);
    let closings = rendered.split("\n\n").last().unwrap();
    for (prefix, expected) in [
        ("Sets read, ", 10),
        ("Flags read, ", 159),
        ("Flags with verdicts, ", 103),
        ("Verdicts read, ", 130),
        ("Chose stickler, ", 90),
        ("Chose clean, ", 32),
        ("Abstained, ", 8),
    ] {
        assert_eq!(count_after(closings, prefix), expected, "{prefix}");
    }
}

#[test]
fn the_loader_refuses_a_file_that_is_not_an_object() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["models must be a JSON object".to_string()]
    );
}

#[test]
fn the_loader_refuses_a_file_missing_its_sets() {
    let value = parse("{}").unwrap();
    assert_eq!(load_models(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn the_loader_refuses_a_sets_value_that_is_not_a_list() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(load_models(&value), vec!["sets must be a list".to_string()]);
}

#[test]
fn the_loader_refuses_an_entry_that_is_not_an_object() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn the_loader_refuses_an_entry_with_an_empty_label() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn the_loader_refuses_a_label_holding_a_line_break() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn the_loader_refuses_a_label_holding_an_unpaired_surrogate() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn the_loader_refuses_an_entry_missing_its_change_set() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_renders_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_models(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the second set of a collection
/// renders the defect list and no reading, whole or
/// nothing, so no partial models reading ever
/// prints. It catches the dropped whole or nothing
/// mutation MD in pure form alone, its defective
/// entry standing at index 1 behind a sound first
/// entry.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")])
    );
    let (code, out) = run_models(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Models\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// A section head model holding a break and a
/// verdicts read line's own text prints as the empty
/// field, so no closing line is forged. It catches
/// the dropped helper mutation ME in pure form at
/// this position, the head position of this reading
/// printing its key as a field through the one
/// helper, in the posture CK-30 states for its
/// section heads.
#[test]
fn a_line_break_in_a_section_head_model_forges_no_closing_line() {
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "evil-model\nVerdicts read, 99".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                handed_entry("jev-1.13.0", "stickler", "0.9"),
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
    assert!(rendered.lines().any(|line| line == "Model, "), "{rendered}");
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1]);
    assert!(!rendered.contains("evil-model"), "{rendered}");
}

/// A file name holding a break and a verdicts read
/// line's own text prints as the empty field in the
/// flag group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_file_forges_no_closing_line() {
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "jev-1.13.0".to_string(),
            groups: vec![handed_group(
                "evil.py\nVerdicts read, 99",
                "one set",
                handed_entry("jev-1.13.0", "stickler", "0.9"),
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
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

/// A pattern holding a break and a verdicts read
/// line's own text prints as the empty field in the
/// flag group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_pattern_forges_no_closing_line() {
    let mut group = handed_group(
        "app.py",
        "one set",
        handed_entry("jev-1.13.0", "stickler", "0.9"),
    );
    group.pattern = "near placeholder name\nVerdicts read, 99".to_string();
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "jev-1.13.0".to_string(),
            groups: vec![group],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, line 4, , set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1]);
}

/// A set label holding a break and a closing line's
/// own text prints as the empty field in the flag
/// group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_set_label_forges_no_closing_line() {
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "jev-1.13.0".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set\nSets read, 99",
                handed_entry("jev-1.13.0", "stickler", "0.9"),
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
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
/// text prints as the empty field in its pair's
/// first line, and the constructed reason built from
/// it prints fenced whole in the second line, so no
/// verdict line is forged.
#[test]
fn a_line_break_in_a_model_is_fenced_in_its_pairs_first_line() {
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "jev-1.13.0".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                handed_entry("jev-1.13.0\nVerdict, forged, clean", "stickler", "0.9"),
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
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
/// by either, and the pair counts in the verdicts
/// read count and in none of the three outcome
/// counts, its choice standing outside the two
/// labels as written.
#[test]
fn a_line_break_in_a_constructed_reason_is_fenced_whole_in_its_pairs_second_line() {
    let reading = ModelsReading {
        sections: vec![ModelsSection {
            model: "jev-1.13.0".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                handed_entry("jev-1.13.0", "clean\nChose clean, 99", "0.9"),
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_models(&reading);
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
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Chose clean, "), 0);
}

/// The closing verdicts read count prints as the
/// sum over the sections the rendering states, so a
/// handed reading whose stored verdicts count
/// disagrees with its sections states the summed
/// count and no handed count can print against the
/// sections. It catches the stored count mutation
/// MF, under which the falsified stored count
/// prints.
#[test]
fn the_closing_verdicts_count_prints_as_the_sum_over_the_sections() {
    let reading = ModelsReading {
        sections: vec![
            ModelsSection {
                model: "jev-1.13.0".to_string(),
                groups: vec![
                    handed_group(
                        "app.py",
                        "one set",
                        handed_entry("jev-1.13.0", "stickler", "0.9"),
                    ),
                    {
                        let mut group = handed_group(
                            "other.py",
                            "one set",
                            handed_entry("jev-1.13.0", "clean", "0.7"),
                        );
                        group.line = 5;
                        group.pattern = "broad handler".to_string();
                        group
                    },
                ],
            },
            ModelsSection {
                model: "openai/gpt-6-luna".to_string(),
                groups: vec![handed_group(
                    "app.py",
                    "one set",
                    handed_entry("openai/gpt-6-luna", "stickler", "0.8"),
                )],
            },
        ],
        sets_read: 1,
        flags_read: 2,
        verdicts_read: 99,
    };
    let rendered = render_models(&reading);
    assert_eq!(verdicts_read_counts(&rendered), vec![2, 1, 3]);
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Flags with verdicts, "), 2);
    assert_eq!(count_after(closings, "Chose stickler, "), 2);
    assert_eq!(count_after(closings, "Chose clean, "), 1);
}
