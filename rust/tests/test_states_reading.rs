//! Pins for CK-41 and CK-42, the states file and
//! reading.
//!
//! The states pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per file verdict state
//! in the order CK-8 states the judgements and standing
//! whether populated or not, a file whose flags carry
//! no recorded verdict standing in no section, the file
//! groups holding the flag groups of their file whole
//! with the pairs in the standing set review form, and
//! the closing counts in their stated relations to the
//! sections and to the verdicts, standings, judgements,
//! and census readings over the same collection. The
//! loader pins name each defect of the file in the
//! forms CK-12 states, one pin per defect. The fencing
//! pins hold the rendering to the one line rule at the
//! library surface, the file name of a file group
//! printing as a field through the one helper at both
//! of its positions, in stated contrast to CK-38, whose
//! section key never prints as a field at all, the
//! reading standing born inside the fencing programme.
//! Every pin runs offline on recorded and constructed
//! entries, and no pin makes a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent at
//! base, against the tree Round 19 closed on, by the
//! absence of the module it names, and within the slice
//! each pin discriminates against the wrong build its
//! own comment names. The golden pin catches the
//! section order mutation MA, the shipped sections
//! standing in the order CK-8 states the judgements and
//! not in alphabetical order, the seating guard
//! mutation MG, the verdictless cache.py line 19
//! seating as an empty group inside the cache.py file
//! group, and the majority derivation mutation MH,
//! ledger.py reading judged clean by majority at one
//! stickler outcome against three clean and so leaving
//! the judged stickler section the golden states it in.
//! It passes under the first set pooling mutation MB
//! and the per set file group mutation MC by design,
//! the shipped collection standing no file in more than
//! one set, and under the stored count mutation MF by
//! design, the stored and summed counts agreeing there
//! at thirty two. The verdicts binding pin catches MG,
//! CK-42 stating the flags with verdicts count in the
//! closed form as the count of the flag groups the
//! sections state, so the seated empty group raises it
//! to eighteen against the verdicts reading's
//! seventeen. The standings binding pin catches MG,
//! the empty group being a flag group the standings
//! reading states nowhere. The census binding pin
//! fails on a build whose counts travel a second
//! flagging path. The sums pin discriminates by content
//! on the rendered sums. It passes under MF, the stored
//! and summed counts agreeing there at thirty two, so
//! the edit moves nothing the pin reads, and MF stands
//! caught by the false counts pin below, and it passes
//! under MB and MC, every pair standing exactly once
//! under the first and the file groups standing whole
//! under the second, and under MG, the empty groups
//! stating no pairs. The one file pin catches MG, the
//! verdictless files of its fixture standing file
//! groups and the files read count rising above one,
//! and the empty collection pin passes under MG by
//! design, the empty collection carrying no flag the
//! deleted guard could seat. The cross set order pin
//! catches MC in its pure form. It passes under MB, its
//! fixture standing judged stickler in both sets. The
//! pooling pin catches MB and MC in pure form, its file
//! reading judged clean on its first set's flags alone
//! and judged stickler over the pool of the whole
//! collection. The both labels pin and the abstention
//! only pin fail on a build that seats a flag's pairs
//! by outcome or drops a group standing under no
//! outcome. The section order pin catches MA in its
//! pure form. The precedence pins catch MH in pure
//! form, and the judgements binding pin catches MH over
//! the shipped collection, ledger.py standing judged
//! stickler there on its line 6 stickler choice. The
//! handed word abstained pin does not catch any builder
//! mutation, the pin hand building its reading and
//! calling only the renderer, so no builder edit can
//! reach it, and it holds at the library surface
//! against a build that drops the pair or seats it by
//! its word there. The loader pins each fail on a
//! loader that accepts the defect they name, and the
//! whole or nothing pin catches the dropped whole or
//! nothing mutation MD, its defect standing in a later
//! entry, the loader pins passing under MD, their
//! defects standing at the head of the value or in the
//! first entry. The fencing pins each fail on a raw
//! print or a handed count in their position, the file
//! group head pin catching the dropped fencing
//! mutation ME in pure form at its position and the
//! other five passing under ME, their positions keeping
//! the helper. The false counts pin catches MF, under
//! which the falsified stored count prints.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::judgements::judgements_reading_of;
use stickler::render::{render_states, render_verdicts};
use stickler::standings::standings_reading_of;
use stickler::states::{
    load_states, states_reading_of, StatesFileGroup, StatesFlagGroup, StatesReading, StatesSection,
};
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
        "stickler-states-{}-{}-{}",
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

fn run_states(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "states.json", text);
    let (code, out, _err) = run(&["states", &path]);
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
/// CK-41 and CK-12 share.
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

/// The verdicts read counts of a states rendering in
/// standing order, the file group closing counts, the
/// section closing counts, and the closing verdicts
/// read count standing last.
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
    let text = std::fs::read_to_string(examples().join("states.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_states(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("states.txt")).unwrap();
    assert_eq!(render_states(&states_reading_of(&value)), golden);
}

/// The shipped collection, read whole. Each of the
/// seven closing counts the states reading shares with
/// the verdicts reading, three sets read, eighteen
/// flags read, seventeen flags with verdicts, thirty
/// two verdicts read, twenty four chose stickler, five
/// chose clean, and three abstained, equals the same
/// named count of the verdicts reading over the same
/// collection, both readings run here, and the files
/// read count stands at the five files carrying
/// verdicts, so a build whose counts travel a second
/// path fails this pin wherever the two organizations
/// of the same pairs disagree. It catches the seating
/// guard mutation MG on the flags with verdicts count,
/// CK-42 stating that count in the closed form as the
/// count of the flag groups the sections state, so the
/// seated empty group raises it to eighteen against
/// the verdicts reading's seventeen.
#[test]
fn the_closings_bind_to_the_verdicts_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("states.json")).unwrap();
    let value = parse(&text).unwrap();
    let states = render_states(&states_reading_of(&value));
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let states_closings = states.split("\n\n").last().unwrap();
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
        assert_eq!(count_after(states_closings, prefix), expected, "{prefix}");
        assert_eq!(
            count_after(states_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
    assert_eq!(count_after(states_closings, "Files read, "), 5);
}

/// The shipped collection, read whole. The file groups
/// of the states reading are exactly the sections of
/// the standings reading over the same collection, file
/// for file, each file group holding the same flag
/// groups in the same order with the same pairs, all
/// three readings run here, so a build that gathers a
/// file's flags by a second path fails this pin
/// wherever that path and the standings reading
/// disagree, and a build that seats a flag carrying no
/// verdicts fails it on the group count of its file.
#[test]
fn the_file_groups_bind_to_the_standings_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("states.json")).unwrap();
    let value = parse(&text).unwrap();
    let states = states_reading_of(&value);
    let standings = standings_reading_of(&value);
    let file_groups: Vec<&StatesFileGroup> = states
        .sections
        .iter()
        .flat_map(|section| section.files.iter())
        .collect();
    assert_eq!(file_groups.len(), standings.sections.len());
    for section in &standings.sections {
        let group = file_groups
            .iter()
            .find(|group| group.file == section.file)
            .unwrap();
        assert_eq!(
            group.groups.len(),
            section.groups.len(),
            "file {}",
            section.file
        );
        for (states_group, standings_group) in group.groups.iter().zip(section.groups.iter()) {
            assert_eq!(states_group.file, standings_group.file);
            assert_eq!(states_group.line, standings_group.line);
            assert_eq!(states_group.pattern, standings_group.pattern);
            assert_eq!(states_group.set_label, standings_group.set_label);
            assert_eq!(states_group.verdicts.len(), standings_group.verdicts.len());
        }
    }
}

/// The closings of the states reading over the shipped
/// collection stand in their relations to the census
/// over the same collection, the flags read count the
/// census total block's flagged lines and the sets read
/// count the count of the sets the census reads.
#[test]
fn the_closings_bind_to_the_census_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("states.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = states_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.sets_read, census.sets.len());
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.flags_read, census.total.borderline);
    assert_eq!(reading.flags_read, 18);
    let rendered = render_states(&reading);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
}

/// The file group closing counts of the shipped
/// rendering sum to their section closing count, the
/// three section closing counts sum to the closing
/// verdicts read count, and the three outcome counts
/// sum to the same, read from the rendered text.
#[test]
fn the_section_counts_sum_to_the_closing_verdicts_read_count() {
    let text = std::fs::read_to_string(examples().join("states.json")).unwrap();
    let value = parse(&text).unwrap();
    let rendered = render_states(&states_reading_of(&value));
    let counts = verdicts_read_counts(&rendered);
    assert_eq!(counts, vec![6, 8, 8, 4, 6, 32, 0, 0, 32]);
    assert_eq!(counts[..5].iter().sum::<usize>(), counts[5]);
    assert_eq!(counts[5] + counts[6] + counts[7], counts[8]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// A constructed one set collection whose verdicts
/// stand only on one file's flags stands that file
/// alone, in the judged clean section its one clean
/// choice earns it, the other two sections stating no
/// files, the other file of the collection, carrying a
/// flag with no verdicts, standing in no section, and
/// the verdictless flag of the standing file standing
/// in no group. It catches the seating guard mutation
/// MG, under which the verdictless file stands a file
/// group and the files read count rises above one.
#[test]
fn a_collection_with_verdicts_on_one_file_stands_that_file_alone() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "app.py",
                FLAGS_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.8}]",
            ),
            entry_json("other.py", ONE_FLAG_DIFF, ""),
        ]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_states(&value), Vec::<String>::new());
    assert_eq!(
        render_states(&states_reading_of(&value)),
        "States\n\
         \n\
         Judged stickler\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Judged clean\n\
         File, app.py\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, clean\n\
         Reason, jev-1.13.0 chose clean with confidence 0.8\n\
         Verdicts read, 1\n\
         Verdicts read, 1\n\
         \n\
         Unjudged\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 1\n\
         Files read, 1\n\
         Flags read, 3\n\
         Flags with verdicts, 1\n\
         Verdicts read, 1\n\
         Chose stickler, 0\n\
         Chose clean, 1\n\
         Abstained, 0\n"
    );
}

/// An empty collection renders the head line, the
/// three sections each stating the no files line and
/// closing at zero, and the closing counts at zero.
#[test]
fn an_empty_collection_renders_three_sections_each_stating_no_files_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_states(&value), Vec::<String>::new());
    assert_eq!(
        render_states(&states_reading_of(&value)),
        "States\n\
         \n\
         Judged stickler\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Judged clean\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Unjudged\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 0\n\
         Files read, 0\n\
         Flags read, 0\n\
         Flags with verdicts, 0\n\
         Verdicts read, 0\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 0\n"
    );
}

/// Over a constructed two set collection where one
/// file stands in both sets, the file stands once, in
/// its states section, its first set's group before its
/// second set's, in collection order. It catches the
/// per set file group mutation MC in its pure form.
#[test]
fn a_file_standing_in_two_sets_stands_once_in_its_states_section_with_its_groups_in_collection_order(
) {
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
    let rendered = render_states(&states_reading_of(&value));
    assert_eq!(rendered.matches("File, app.py").count(), 1, "{rendered}");
    let head = rendered
        .lines()
        .position(|line| line == "Judged stickler")
        .unwrap();
    let file = rendered
        .lines()
        .position(|line| line == "File, app.py")
        .unwrap();
    let clean = rendered
        .lines()
        .position(|line| line == "Judged clean")
        .unwrap();
    assert!(head < file && file < clean, "{rendered}");
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

/// A constructed flag carrying one verdict choosing
/// stickler and one choosing clean stands once, in its
/// file's group, its group holding both pairs in their
/// standing order, so no outcome split seating of the
/// flag stands anywhere in the rendering, and the file
/// stands judged stickler on the stickler choice.
#[test]
fn a_flag_carrying_both_labels_stands_once_in_its_files_group_with_its_pairs_whole() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_states(&states_reading_of(&value));
    assert_eq!(
        rendered
            .matches("app.py, line 4, near placeholder name, set, one set")
            .count(),
        1,
        "{rendered}"
    );
    assert_eq!(
        rendered,
        "States\n\
         \n\
         Judged stickler\n\
         File, app.py\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.9\n\
         Verdict, openai/gpt-6-luna, clean\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.8\n\
         Verdicts read, 2\n\
         Verdicts read, 2\n\
         \n\
         Judged clean\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Unjudged\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Sets read, 1\n\
         Files read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 2\n\
         Chose stickler, 1\n\
         Chose clean, 1\n\
         Abstained, 0\n"
    );
}

/// A constructed flag carrying only abstentions stands
/// in its file's group, its group holding its pairs
/// whole, each printing in the abstention form, so a
/// seating by outcome, under which an all abstention
/// group has no outcome section to stand in, drops it
/// or seats it under a label and fails here, and the
/// file stands unjudged on the abstentions alone.
#[test]
fn a_flag_carrying_only_abstentions_stands_in_its_files_group_with_its_pairs_whole() {
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
        render_states(&states_reading_of(&value)),
        "States\n\
         \n\
         Judged stickler\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Judged clean\n\
         No files\n\
         Verdicts read, 0\n\
         \n\
         Unjudged\n\
         File, app.py\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.3 and abstained\n\
         Verdict, openai/gpt-6-luna, abstained\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.4 and abstained\n\
         Verdicts read, 2\n\
         Verdicts read, 2\n\
         \n\
         Sets read, 1\n\
         Files read, 1\n\
         Flags read, 1\n\
         Flags with verdicts, 1\n\
         Verdicts read, 2\n\
         Chose stickler, 0\n\
         Chose clean, 0\n\
         Abstained, 2\n"
    );
}

/// Over a constructed collection whose files stand
/// judged clean, judged stickler, and unjudged in that
/// first appearance order, the sections stand in the
/// order CK-8 states the judgements, judged stickler
/// first and unjudged last, and not in first appearance
/// order. It catches the section order mutation MA in
/// its pure form.
#[test]
fn the_sections_stand_in_the_ck_8_order_not_in_first_appearance_order() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "windows.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.8}]",
            ),
            entry_json(
                "harvest.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            ),
            entry_json(
                "cache.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.3}]",
            ),
        ]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_states(&states_reading_of(&value));
    let position = |needle: &str| rendered.lines().position(|line| line == needle).unwrap();
    let stickler = position("Judged stickler");
    let clean = position("Judged clean");
    let unjudged = position("Unjudged");
    assert!(stickler < clean && clean < unjudged, "{rendered}");
    let harvest = position("File, harvest.py");
    let windows = position("File, windows.py");
    let cache = position("File, cache.py");
    assert!(stickler < harvest && harvest < clean, "{rendered}");
    assert!(clean < windows && windows < unjudged, "{rendered}");
    assert!(unjudged < cache, "{rendered}");
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
fn handed_group(file: &str, set_label: &str, verdicts: Vec<VerdictEntry>) -> StatesFlagGroup {
    StatesFlagGroup {
        file: file.to_string(),
        line: 4,
        pattern: "near placeholder name".to_string(),
        set_label: set_label.to_string(),
        verdicts,
    }
}

/// Build one file group by hand in the family's form.
fn handed_file_group(file: &str, groups: Vec<StatesFlagGroup>) -> StatesFileGroup {
    StatesFileGroup {
        file: file.to_string(),
        groups,
    }
}

/// Build one section by hand in the family's form.
fn handed_section(state: &'static str, files: Vec<StatesFileGroup>) -> StatesSection {
    StatesSection { state, files }
}

/// A hand built reading whose one flag carries a
/// verdict whose choice stands as the word abstained
/// at a confidence that is no abstention renders that
/// flag's group in its file's group with the pair
/// printing the choice as written, the pair counting
/// in the reading's verdicts read count and in none
/// of its three outcome counts, in the posture CK-42
/// states, because its section is the state of its
/// flag's file and not its own, and the file carrying
/// only such a verdict stands unjudged. A build that
/// seats by the verdict, through the outcome word the
/// verdict carries, drops the pair or seats it by its
/// word and fails here.
#[test]
fn a_handed_word_abstained_verdict_stands_in_its_flags_group() {
    let reading = StatesReading {
        sections: vec![
            handed_section("stickler", vec![]),
            handed_section("clean", vec![]),
            handed_section(
                "unjudged",
                vec![handed_file_group(
                    "app.py",
                    vec![handed_group(
                        "app.py",
                        "one set",
                        vec![handed_entry("jev-1.13.0", "abstained", "0.9")],
                    )],
                )],
            ),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
    assert!(
        rendered.lines().any(|line| line == "File, app.py"),
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
    let unjudged = rendered
        .lines()
        .position(|line| line == "Unjudged")
        .unwrap();
    let file = rendered
        .lines()
        .position(|line| line == "File, app.py")
        .unwrap();
    assert!(unjudged < file, "{rendered}");
    assert_eq!(verdicts_read_counts(&rendered), vec![0, 0, 1, 1, 1]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Files read, "), 1);
    assert_eq!(count_after(closings, "Flags with verdicts, "), 1);
    assert_eq!(count_after(closings, "Chose stickler, "), 0);
    assert_eq!(count_after(closings, "Chose clean, "), 0);
    assert_eq!(count_after(closings, "Abstained, "), 0);
}

/// A constructed file carrying one stickler choice on
/// one flag and only clean choices on another stands
/// judged stickler, in the precedence of CK-41, though
/// its second flag stands judged clean under CK-8. It
/// catches the majority derivation mutation MH in pure
/// form, majority reading the file judged clean at one
/// stickler outcome against two clean.
#[test]
fn a_file_carrying_one_stickler_choice_stands_judged_stickler_though_another_of_its_flags_stands_judged_clean(
) {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            FLAGS_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = states_reading_of(&value);
    assert_eq!(reading.sections[0].state, "stickler");
    assert_eq!(reading.sections[0].files.len(), 1);
    assert_eq!(reading.sections[0].files[0].file, "app.py");
    assert!(reading.sections[1].files.is_empty());
    assert!(reading.sections[2].files.is_empty());
}

/// A constructed file carrying no stickler choice and
/// one clean choice stands judged clean, in the
/// precedence of CK-41, though another of its flags
/// carries only abstentions and so stands unjudged
/// under CK-8.
#[test]
fn a_file_carrying_no_stickler_choice_and_one_clean_choice_stands_judged_clean_though_another_of_its_flags_carries_only_abstentions(
) {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            FLAGS_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.3},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.4}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = states_reading_of(&value);
    assert_eq!(reading.sections[1].state, "clean");
    assert_eq!(reading.sections[1].files.len(), 1);
    assert_eq!(reading.sections[1].files[0].file, "app.py");
    assert!(reading.sections[0].files.is_empty());
    assert!(reading.sections[2].files.is_empty());
}

/// Over a constructed two set collection, a file
/// reading judged clean on its first set's flags alone
/// stands judged stickler over the pool of the whole
/// collection once its second set carries a stickler
/// choice, the state pooling every recorded verdict
/// standing on the file's flags across the collection.
/// It catches the first set pooling mutation MB and
/// the per set file group mutation MC in pure form.
#[test]
fn a_file_pools_its_verdicts_across_sets_for_its_state() {
    let first = (
        "first set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    );
    let alone = parse(&collection_json(&[first.clone()])).unwrap();
    let alone_reading = states_reading_of(&alone);
    assert_eq!(alone_reading.sections[1].files.len(), 1);
    assert_eq!(alone_reading.sections[1].files[0].file, "app.py");
    let text = collection_json(&[
        first,
        (
            "second set",
            change_set_json(&[entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            )]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let reading = states_reading_of(&value);
    assert_eq!(reading.sections[0].files.len(), 1);
    assert_eq!(reading.sections[0].files[0].file, "app.py");
    assert_eq!(reading.sections[0].files[0].groups.len(), 2);
    assert!(reading.sections[1].files.is_empty());
}

/// The shipped collection, read whole. Each file
/// carrying verdicts stands in the states section of
/// the precedence over the judgements of its flags
/// exactly as the judgements reading over the same
/// collection states those judgements, a file standing
/// judged stickler when any of its flags stands in the
/// judged stickler section of the judgements reading,
/// judged clean when none does and at least one stands
/// in its judged clean section, and unjudged when every
/// verdict carrying flag of the file stands in its
/// unjudged section, both readings run here. It
/// catches the majority derivation mutation MH over
/// the shipped collection, ledger.py standing judged
/// stickler on its line 6 flag's judgement there.
#[test]
fn the_states_of_the_example_bind_to_the_judgements_reading() {
    let text = std::fs::read_to_string(examples().join("states.json")).unwrap();
    let value = parse(&text).unwrap();
    let states = states_reading_of(&value);
    let judgements = judgements_reading_of(&value);
    let mut files: Vec<String> = Vec::new();
    for section in &judgements.sections {
        for group in &section.groups {
            if !files.contains(&group.file) {
                files.push(group.file.clone());
            }
        }
    }
    assert_eq!(files.len(), 5);
    for file in &files {
        let expected = if judgements.sections[0]
            .groups
            .iter()
            .any(|group| &group.file == file)
        {
            "stickler"
        } else if judgements.sections[1]
            .groups
            .iter()
            .any(|group| &group.file == file)
        {
            "clean"
        } else {
            "unjudged"
        };
        let standing = states
            .sections
            .iter()
            .find(|section| section.files.iter().any(|group| &group.file == file))
            .unwrap();
        assert_eq!(standing.state, expected, "file {file}");
    }
}

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["states must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_states(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(load_states(&value), vec!["sets must be a list".to_string()]);
}

#[test]
fn an_entry_that_is_not_an_object_is_refused() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_that_is_empty_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_missing_change_set_field_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_states(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the last set of a collection renders
/// the defect list and no reading, whole or nothing,
/// so no partial states reading ever prints.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")])
    );
    let (code, out) = run_states(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "States\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// A file group head file name holding a break and a
/// verdicts read line's own text prints as the empty
/// field, so no closing line is forged. It catches
/// the dropped helper mutation ME in pure form at
/// this position, the head position of this reading
/// printing its key as a field through the one
/// helper, in stated contrast to CK-38.
#[test]
fn a_line_break_in_a_file_group_head_file_name_forges_no_closing_line() {
    let reading = StatesReading {
        sections: vec![
            handed_section(
                "stickler",
                vec![handed_file_group(
                    "evil.py\nVerdicts read, 99",
                    vec![handed_group(
                        "app.py",
                        "one set",
                        vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
                    )],
                )],
            ),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
    assert!(rendered.lines().any(|line| line == "File, "), "{rendered}");
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1, 0, 0, 1]);
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// A file name holding a break and a verdicts read
/// line's own text prints as the empty field in the
/// flag group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_file_forges_no_closing_line() {
    let reading = StatesReading {
        sections: vec![
            handed_section(
                "stickler",
                vec![handed_file_group(
                    "app.py",
                    vec![handed_group(
                        "evil.py\nVerdicts read, 99",
                        "one set",
                        vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
                    )],
                )],
            ),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
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
        vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
    );
    group.pattern = "near placeholder name\nVerdicts read, 99".to_string();
    let reading = StatesReading {
        sections: vec![
            handed_section("stickler", vec![handed_file_group("app.py", vec![group])]),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
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
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1, 0, 0, 1]);
}

/// A set label holding a break and a closing line's
/// own text prints as the empty field in the flag
/// group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_set_label_forges_no_closing_line() {
    let reading = StatesReading {
        sections: vec![
            handed_section(
                "stickler",
                vec![handed_file_group(
                    "app.py",
                    vec![handed_group(
                        "app.py",
                        "one set\nSets read, 99",
                        vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
                    )],
                )],
            ),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
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
    let reading = StatesReading {
        sections: vec![
            handed_section(
                "stickler",
                vec![handed_file_group(
                    "app.py",
                    vec![handed_group(
                        "app.py",
                        "one set",
                        vec![handed_entry(
                            "jev-1.13.0\nVerdict, forged, clean",
                            "stickler",
                            "0.9",
                        )],
                    )],
                )],
            ),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
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
/// counts, its choice standing outside the two labels
/// as written.
#[test]
fn a_line_break_in_a_constructed_reason_is_fenced_whole_in_its_pairs_second_line() {
    let reading = StatesReading {
        sections: vec![
            handed_section(
                "stickler",
                vec![handed_file_group(
                    "app.py",
                    vec![handed_group(
                        "app.py",
                        "one set",
                        vec![handed_entry("jev-1.13.0", "clean\nChose clean, 99", "0.9")],
                    )],
                )],
            ),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_states(&reading);
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
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1, 0, 0, 1]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Chose clean, "), 0);
}

/// The closing counts print as the counts over the
/// sections, so a handed reading whose stored verdicts
/// count disagrees with its sections states the summed
/// verdicts read count, the count of its flag groups
/// as its flags with verdicts count, and the count of
/// its file groups as its files read count, and no
/// handed count can print against the sections. It
/// catches the stored count mutation MF, under which
/// the falsified stored count prints.
#[test]
fn the_closing_counts_print_as_the_counts_over_the_sections() {
    let reading = StatesReading {
        sections: vec![
            handed_section(
                "stickler",
                vec![handed_file_group(
                    "app.py",
                    vec![handed_group(
                        "app.py",
                        "one set",
                        vec![
                            handed_entry("jev-1.13.0", "stickler", "0.9"),
                            handed_entry("openai/gpt-6-luna", "stickler", "0.8"),
                        ],
                    )],
                )],
            ),
            handed_section(
                "clean",
                vec![handed_file_group(
                    "other.py",
                    vec![{
                        let mut group = handed_group(
                            "other.py",
                            "one set",
                            vec![handed_entry("jev-1.13.0", "clean", "0.7")],
                        );
                        group.line = 5;
                        group.pattern = "broad handler".to_string();
                        group
                    }],
                )],
            ),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 2,
        verdicts_read: 99,
    };
    let rendered = render_states(&reading);
    assert_eq!(verdicts_read_counts(&rendered), vec![2, 2, 1, 1, 0, 3]);
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Files read, "), 2);
    assert_eq!(count_after(closings, "Flags with verdicts, "), 2);
    assert_eq!(count_after(closings, "Chose stickler, "), 2);
    assert_eq!(count_after(closings, "Chose clean, "), 1);
}
