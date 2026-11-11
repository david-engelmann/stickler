//! Pins for CK-39 and CK-40, the standings file and
//! reading.
//!
//! The standings pins build their collections by
//! hand, so each one fails under the negation of the
//! rule it names, the sections standing per file in
//! first appearance order across the collection, a
//! file whose flags carry no recorded verdict
//! standing no section, the flag groups holding all
//! the pairs of their flag whole, the pairs in the
//! standing set review form, and the closing counts
//! in their stated relations to the sections and to
//! the verdicts, files, and census closings over the
//! same collection. The loader pins name each defect
//! of the file in the forms CK-12 states, one pin per
//! defect. The fencing pins hold the rendering to the
//! one line rule at the library surface, the file
//! name of a section printing as a field through the
//! one helper, in stated contrast to CK-38, whose
//! section key never prints as a field at all, the
//! reading standing born inside the fencing
//! programme. Every pin runs offline on recorded and
//! constructed entries, and no pin makes a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent
//! at base, against the tree Round 18 closed on, by
//! the absence of the module it names, and within the
//! slice each pin discriminates against the wrong
//! build its own comment names. The golden pin
//! discriminates by content against any build whose
//! counts, order, sections, groups, or pairs differ,
//! and it catches the per outcome seating mutation MB
//! on the shipped collection, whose ledger.py line 6
//! and cache.py line 15 groups carry both labels
//! whole in their files' sections, and it catches the
//! verdictless flags mutation MG, the verdictless
//! cache.py line 19 seating as an empty group in the
//! cache.py section, and it catches the section order
//! mutation MA, the shipped order, harvest.py,
//! windows.py, sorting.py, ledger.py, and cache.py,
//! not being alphabetical. It passes under the file
//! and set keying mutation MC by design, the shipped
//! collection standing no file in more than one set,
//! and under the stored count mutation MF by design,
//! the stored and summed counts agreeing there at 32.
//! The verdicts binding pin catches MB, the flags
//! with verdicts count counting seatings and rising
//! above the verdicts reading's count. It passes
//! under MG, the rendering counting flags with
//! verdicts only over the groups carrying verdicts,
//! so a seated empty group moves no closing count,
//! and MG stands caught by the golden pin, the
//! files binding pin, and the one file pin. The
//! files binding pin catches MA, the sections
//! leaving the files reading's order, MB, the
//! sections stating seatings and not the files
//! reading's flags, and MG, the empty group being
//! a flag the binding excludes. It passes under
//! MC, its shipped fixture standing every file in
//! one set only, so the file and set keying moves
//! nothing it reads, and MC stands caught by the
//! cross set order pin in this target and by the
//! sections binding pin of the fixtures file over
//! the constructed collection. The census binding pin fails on a build
//! whose counts travel a second flagging path. The
//! sums pin discriminates by content on the rendered
//! sums. It does not catch MF over the shipped
//! collection, the stored and summed counts agreeing
//! there at 32, so the edit moves nothing the pin
//! reads, and MF stands caught by the false counts
//! pin below, and it passes under MB and MG, every
//! pair standing exactly once under the first and the
//! empty groups stating no pairs under the second.
//! The one file pin catches MG, the verdictless files
//! of its fixture standing sections and the files
//! read count rising above one, and the empty
//! collection pin fails on a build that stands a
//! section the collection leaves empty, passing
//! under MG by design, the empty collection carrying
//! no flag the deleted guard could seat. The cross
//! set order pin catches MC in its pure form. The
//! both labels pin catches MB in its pure form,
//! against any outcome split seating of a flag. The
//! abstention only pin fails on a build that seats a
//! flag's pairs by outcome or drops an all abstention
//! group, which has no outcome section to stand in.
//! The first appearance order pin catches MA in its
//! pure form. The handed word abstained pin does not
//! catch any builder mutation, the pin hand building
//! its reading and calling only the renderer, so no
//! builder edit can reach it, and it holds at the
//! library surface against a build that drops the
//! pair or seats it by its word there. The loader
//! pins each fail on a loader that accepts the defect
//! they name, and the whole or nothing pin catches
//! the dropped whole or nothing mutation MD, its
//! defect standing in a later entry, the loader pins
//! passing under MD, their defects standing at the
//! head of the value or in the first entry. The
//! fencing pins each fail on a raw print or a handed
//! count in their position, the section head pin
//! catching the dropped fencing mutation ME in pure
//! form at its position and the other five passing
//! under ME, their positions keeping the helper.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::files::files_reading_of;
use stickler::json::parse;
use stickler::render::{render_standings, render_verdicts};
use stickler::standings::{
    load_standings, standings_reading_of, StandingFlagGroup, StandingsReading, StandingsSection,
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
        "stickler-standings-{}-{}-{}",
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

fn run_standings(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "standings.json", text);
    let (code, out, _err) = run(&["standings", &path]);
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
/// CK-39 and CK-12 share.
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

/// The verdicts read counts of a standings rendering in
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
    let text = std::fs::read_to_string(examples().join("standings.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_standings(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("standings.txt")).unwrap();
    assert_eq!(render_standings(&standings_reading_of(&value)), golden);
}

/// The shipped collection, read whole. Each of the
/// seven closing counts the standings reading shares
/// with the verdicts reading, three sets read,
/// eighteen flags read, seventeen flags with
/// verdicts, thirty two verdicts read, twenty four
/// chose stickler, five chose clean, and three
/// abstained, equals the same named count of the
/// verdicts reading over the same collection, both
/// readings run here, and the files read count stands
/// at the five files carrying verdicts, so a build
/// whose counts travel a second path fails this pin
/// wherever the two organizations of the same pairs
/// disagree.
#[test]
fn the_closings_bind_to_the_verdicts_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("standings.json")).unwrap();
    let value = parse(&text).unwrap();
    let standings = render_standings(&standings_reading_of(&value));
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let standings_closings = standings.split("\n\n").last().unwrap();
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
        assert_eq!(
            count_after(standings_closings, prefix),
            expected,
            "{prefix}"
        );
        assert_eq!(
            count_after(standings_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
    assert_eq!(count_after(standings_closings, "Files read, "), 5);
}

/// The shipped collection, read whole. The sections
/// of the standings reading are exactly the files the
/// files reading names that carry at least one
/// recorded verdict, in the same order, the flag
/// groups of each section are exactly the flags that
/// reading names under the file that carry verdicts,
/// and each section's closing count equals the count
/// of the pairs standing on those flags in the
/// verdicts reading, all three readings run here, so
/// a build that gathers a file's flags by a second
/// path fails this pin wherever that path and the
/// files reading disagree, and a build that seats a
/// flag carrying no verdicts fails it on the group
/// count of its section.
#[test]
fn the_sections_bind_to_the_files_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("standings.json")).unwrap();
    let value = parse(&text).unwrap();
    let standings = standings_reading_of(&value);
    let files = files_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let pair_count = |set_label: &str, file: &str, line: usize, pattern: &str| -> Option<usize> {
        verdicts
            .sections
            .iter()
            .filter(|section| section.label == set_label)
            .flat_map(|section| section.groups.iter())
            .find(|group| group.file == file && group.line == line && group.pattern == pattern)
            .map(|group| group.verdicts.len())
    };
    let expected_sections: Vec<_> = files
        .sections
        .iter()
        .filter(|section| {
            section.flags.iter().any(|flag| {
                pair_count(&flag.set_label, &section.file, flag.line, &flag.pattern).is_some()
            })
        })
        .collect();
    assert_eq!(standings.sections.len(), expected_sections.len());
    for (standings_section, files_section) in
        standings.sections.iter().zip(expected_sections.iter())
    {
        assert_eq!(standings_section.file, files_section.file);
        let expected: Vec<_> = files_section
            .flags
            .iter()
            .filter(|flag| {
                pair_count(
                    &flag.set_label,
                    &files_section.file,
                    flag.line,
                    &flag.pattern,
                )
                .is_some()
            })
            .collect();
        assert_eq!(
            standings_section.groups.len(),
            expected.len(),
            "section {}",
            standings_section.file
        );
        for (group, flag) in standings_section.groups.iter().zip(expected.iter()) {
            assert_eq!(
                group.file, files_section.file,
                "section {}",
                standings_section.file
            );
            assert_eq!(group.line, flag.line, "section {}", standings_section.file);
            assert_eq!(
                group.pattern, flag.pattern,
                "section {}",
                standings_section.file
            );
            assert_eq!(
                group.set_label, flag.set_label,
                "section {}",
                standings_section.file
            );
        }
        let stated: usize = standings_section
            .groups
            .iter()
            .map(|group| group.verdicts.len())
            .sum();
        let counted: usize = expected
            .iter()
            .map(|flag| {
                pair_count(
                    &flag.set_label,
                    &files_section.file,
                    flag.line,
                    &flag.pattern,
                )
                .unwrap()
            })
            .sum();
        assert_eq!(stated, counted, "section {}", standings_section.file);
    }
}

/// The closings of the standings reading over the
/// shipped collection stand in their relations to the
/// census over the same collection, the flags read
/// count the census total block's flagged lines and
/// the sets read count the count of the sets the
/// census reads.
#[test]
fn the_closings_bind_to_the_census_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("standings.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = standings_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.sets_read, census.sets.len());
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.flags_read, census.total.borderline);
    assert_eq!(reading.flags_read, 18);
    let rendered = render_standings(&reading);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
}

/// The five section closing counts of the shipped
/// rendering sum to its closing verdicts read count,
/// and the three outcome counts sum to the same, read
/// from the rendered text.
#[test]
fn the_section_counts_sum_to_the_closing_verdicts_read_count() {
    let text = std::fs::read_to_string(examples().join("standings.json")).unwrap();
    let value = parse(&text).unwrap();
    let rendered = render_standings(&standings_reading_of(&value));
    let counts = verdicts_read_counts(&rendered);
    assert_eq!(counts, vec![6, 8, 8, 4, 6, 32]);
    assert_eq!(counts[..5].iter().sum::<usize>(), counts[5]);
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
/// alone, the other file of the collection, carrying
/// a flag with no verdicts, standing no section, and
/// the verdictless flag of the standing file standing
/// in no group. It catches the seating guard mutation
/// MG, under which the verdictless file stands a
/// section and the files read count rises above one.
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
    assert_eq!(load_standings(&value), Vec::<String>::new());
    assert_eq!(
        render_standings(&standings_reading_of(&value)),
        "Standings\n\
         \n\
         File, app.py\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, clean\n\
         Reason, jev-1.13.0 chose clean with confidence 0.8\n\
         Verdicts read, 1\n\
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

/// An empty collection renders the head line and the
/// eight closing counts at zero, and no section
/// stands.
#[test]
fn an_empty_collection_renders_no_section_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_standings(&value), Vec::<String>::new());
    assert_eq!(
        render_standings(&standings_reading_of(&value)),
        "Standings\n\
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
/// file stands in both sets, the file stands one
/// section, its first set's group before its second
/// set's, in collection order.
#[test]
fn a_file_section_lists_groups_in_collection_order_across_sets() {
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
    let rendered = render_standings(&standings_reading_of(&value));
    assert_eq!(rendered.matches("File, app.py").count(), 1, "{rendered}");
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
/// stickler and one choosing clean stands once, in
/// its file's section, its group holding both pairs
/// in their standing order, so no outcome split
/// seating of the flag stands anywhere in the
/// rendering.
#[test]
fn a_flag_carrying_both_labels_stands_once_in_its_files_section_with_its_pairs_whole() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_standings(&standings_reading_of(&value));
    assert_eq!(
        rendered
            .matches("app.py, line 4, near placeholder name, set, one set")
            .count(),
        1,
        "{rendered}"
    );
    assert_eq!(
        rendered,
        "Standings\n\
         \n\
         File, app.py\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, stickler\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.9\n\
         Verdict, openai/gpt-6-luna, clean\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.8\n\
         Verdicts read, 2\n\
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
/// in its file's section, its group holding its pairs
/// whole, each printing in the abstention form, so a
/// seating by outcome, under which an all abstention
/// group has no outcome section to stand in, drops it
/// or seats it under a label and fails here.
#[test]
fn a_flag_carrying_only_abstentions_stands_in_its_files_section_with_its_pairs_whole() {
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
        render_standings(&standings_reading_of(&value)),
        "Standings\n\
         \n\
         File, app.py\n\
         app.py, line 4, near placeholder name, set, one set\n\
         Verdict, jev-1.13.0, abstained\n\
         Reason, jev-1.13.0 chose stickler with confidence 0.3 and abstained\n\
         Verdict, openai/gpt-6-luna, abstained\n\
         Reason, openai/gpt-6-luna chose clean with confidence 0.4 and abstained\n\
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

/// Over a constructed collection whose files stand in
/// the order windows.py, harvest.py, and cache.py in
/// their set, the sections stand in that first
/// appearance order, and not in alphabetical order,
/// which would stand cache.py first and windows.py
/// last.
#[test]
fn the_sections_stand_in_first_appearance_order_not_alphabetical_order() {
    let verdicts =
        "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]";
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json("windows.py", ONE_FLAG_DIFF, verdicts),
            entry_json("harvest.py", ONE_FLAG_DIFF, verdicts),
            entry_json("cache.py", ONE_FLAG_DIFF, verdicts),
        ]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_standings(&standings_reading_of(&value));
    let windows = rendered
        .lines()
        .position(|line| line == "File, windows.py")
        .unwrap();
    let harvest = rendered
        .lines()
        .position(|line| line == "File, harvest.py")
        .unwrap();
    let cache = rendered
        .lines()
        .position(|line| line == "File, cache.py")
        .unwrap();
    assert!(windows < harvest && harvest < cache, "{rendered}");
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
fn handed_group(file: &str, set_label: &str, verdicts: Vec<VerdictEntry>) -> StandingFlagGroup {
    StandingFlagGroup {
        file: file.to_string(),
        line: 4,
        pattern: "near placeholder name".to_string(),
        set_label: set_label.to_string(),
        verdicts,
    }
}

/// A hand built reading whose one flag carries a
/// verdict whose choice stands as the word abstained
/// at a confidence that is no abstention renders that
/// flag's group in its file's section with the pair
/// printing the choice as written, the pair counting
/// in the reading's verdicts read count and in none
/// of its three outcome counts, in the posture CK-40
/// states, because its section is its flag's file and
/// not its own. A build that seats by the verdict,
/// through the outcome word the verdict carries,
/// drops the pair or seats it by its word and fails
/// here.
#[test]
fn a_handed_word_abstained_verdict_stands_in_its_flags_group() {
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "app.py".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                vec![handed_entry("jev-1.13.0", "abstained", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
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
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Files read, "), 1);
    assert_eq!(count_after(closings, "Flags with verdicts, "), 1);
    assert_eq!(count_after(closings, "Chose stickler, "), 0);
    assert_eq!(count_after(closings, "Chose clean, "), 0);
    assert_eq!(count_after(closings, "Abstained, "), 0);
}

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["standings must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_standings(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets must be a list".to_string()]
    );
}

#[test]
fn an_entry_that_is_not_an_object_is_refused() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_that_is_empty_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_missing_change_set_field_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_standings(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the last set of a collection renders
/// the defect list and no reading, whole or nothing,
/// so no partial standings reading ever prints.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")])
    );
    let (code, out) = run_standings(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Standings\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// A section head file name holding a break and a
/// verdicts read line's own text prints as the empty
/// field, so no closing line is forged. It catches
/// the dropped helper mutation ME in pure form at
/// this position, the head position of this reading
/// printing its key as a field through the one
/// helper, in stated contrast to CK-38.
#[test]
fn a_line_break_in_a_section_head_file_name_forges_no_closing_line() {
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "evil.py\nVerdicts read, 99".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
    assert!(rendered.lines().any(|line| line == "File, "), "{rendered}");
    assert!(
        !rendered.lines().any(|line| line == "Verdicts read, 99"),
        "{rendered}"
    );
    assert_eq!(verdicts_read_counts(&rendered), vec![1, 1]);
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// A file name holding a break and a verdicts read
/// line's own text prints as the empty field in the
/// flag group line, so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_group_file_forges_no_closing_line() {
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "app.py".to_string(),
            groups: vec![handed_group(
                "evil.py\nVerdicts read, 99",
                "one set",
                vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
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
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "app.py".to_string(),
            groups: vec![group],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
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
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "app.py".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set\nSets read, 99",
                vec![handed_entry("jev-1.13.0", "stickler", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
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
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "app.py".to_string(),
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
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
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
    let reading = StandingsReading {
        sections: vec![StandingsSection {
            file: "app.py".to_string(),
            groups: vec![handed_group(
                "app.py",
                "one set",
                vec![handed_entry("jev-1.13.0", "clean\nChose clean, 99", "0.9")],
            )],
        }],
        sets_read: 1,
        flags_read: 1,
        verdicts_read: 1,
    };
    let rendered = render_standings(&reading);
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

/// The closing verdicts read count prints as the sum
/// over the sections the rendering states, so a
/// handed reading whose stored verdicts count
/// disagrees with its sections states the summed
/// count and no handed count can print against the
/// sections. It catches the stored count mutation MF,
/// under which the falsified stored count prints.
#[test]
fn the_closing_verdicts_count_prints_as_the_sum_over_the_sections() {
    let reading = StandingsReading {
        sections: vec![
            StandingsSection {
                file: "app.py".to_string(),
                groups: vec![handed_group(
                    "app.py",
                    "one set",
                    vec![
                        handed_entry("jev-1.13.0", "stickler", "0.9"),
                        handed_entry("openai/gpt-6-luna", "stickler", "0.8"),
                    ],
                )],
            },
            StandingsSection {
                file: "other.py".to_string(),
                groups: vec![{
                    let mut group = handed_group(
                        "other.py",
                        "one set",
                        vec![handed_entry("jev-1.13.0", "clean", "0.7")],
                    );
                    group.line = 5;
                    group.pattern = "broad handler".to_string();
                    group
                }],
            },
        ],
        sets_read: 1,
        flags_read: 2,
        verdicts_read: 99,
    };
    let rendered = render_standings(&reading);
    assert_eq!(verdicts_read_counts(&rendered), vec![2, 1, 3]);
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
