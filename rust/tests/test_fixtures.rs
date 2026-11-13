//! Pins for the constructed fixtures of Round 12, the large
//! change set, the large collection, and the large diff under
//! examples, stated in the Round 12 contract preamble.
//!
//! The large change set is pinned by dimension pins read off the
//! fixture itself, by the tallies its own set review states, and
//! by the tie at the head of its worst first ordering. The large
//! collection is pinned by the closing relations, the borderline
//! reading over the collection reading the same borderline count
//! as the census total block over the collection, every census
//! total standing as the sum of its set blocks, and the set
//! review over the large set alone stating the same flagged
//! count as the census block of the large set. The large diff is
//! pinned by dimension pins and by the per shape counts of the
//! check, standing as literals counted from the construction.
//! The large verdicts collection of Round 13 is pinned by
//! dimension pins read off the fixture itself under the
//! parameters the preamble of CK-27 states, by the four shape
//! requirements that preamble names, and by the closing
//! relations of the verdicts reading against the census and
//! the outcomes reading over the same collection, the
//! outcomes relation holding the constructed literal three
//! for the collection's unjudged flags carrying verdicts.
//! The constructed choices collection of Round 15 is pinned
//! by dimension pins read off the fixture itself under the
//! parameters the preamble of the Round 15 section states,
//! by the six shape requirements that preamble names,
//! counted from the choices reading and the verdicts
//! reading over the collection, and by the closing
//! relations of the choices reading against the verdicts
//! reading and the census over the same collection and
//! against its own section counts.
//! The constructed categories collection of Round 16 is
//! pinned by dimension pins read off the fixture itself
//! under the parameters the preamble of the Round 16
//! section states, by the six shape requirements that
//! preamble names, counted from the categories reading
//! over the collection, and by the closing relations of
//! the categories reading against the shapes reading
//! regrouped by category and the census over the same
//! collection and against its own section counts. The
//! dimension and requirement pins fail against the
//! tree Round 15 closed on by the absence of the
//! fixture they read, and each one can fail against a
//! miscounted construction on the counts it reads. The
//! three relation pins are compile absent against that
//! tree by the absence of the categories module they
//! name.
//! The constructed grounds collection of Round 17 is
//! pinned by dimension pins read off the fixture itself
//! under the parameters the preamble of the Round 17
//! section states, by the six shape requirements that
//! preamble names, counted from the grounds reading
//! and the pattern reading over the collection, and by
//! the closing relations of the grounds reading
//! against the verdicts reading and the pattern
//! reading over the same collection and against its
//! own section counts. The whole of this file is
//! compile absent against the tree Round 16 closed on
//! by the grounds module it imports, so no pin of this
//! round stands at base to fail by the fixture's
//! absence alone, and the dimension and requirement
//! pins are additionally able to fail against a
//! miscounted construction on the counts they read,
//! and the three relation pins are compile absent by
//! the same absence.
//! The constructed judgements collection of Round 18
//! is pinned by dimension pins read off the fixture
//! itself under the parameters the preamble of the
//! Round 18 section states, by the six shape
//! requirements that preamble names, counted from the
//! judgements reading and the outcomes reading over
//! the collection, and by the closing relations of
//! the judgements reading against the verdicts
//! reading and the outcomes reading over the same
//! collection and against its own section counts. The
//! fixture file stands absent against the tree Round
//! 17 closed on, so the dimension and requirement
//! pins fail there by the fixture's absence read at
//! run time, and the three relation pins are compile
//! absent by the judgements module's absence in their
//! judgements half, and the dimension and requirement
//! pins are additionally able to fail against a
//! miscounted construction on the counts they read.
//! The constructed standings collection of Round 19
//! is pinned by dimension pins read off the fixture
//! itself under the parameters the preamble of the
//! Round 19 section states, by the six shape
//! requirements that preamble names, counted from the
//! standings reading and the files reading over the
//! collection, and by the closing relations of the
//! standings reading against the verdicts reading and
//! the files reading over the same collection and
//! against its own section counts. The fixture file
//! stands absent against the tree Round 18 closed on,
//! so the dimension and requirement pins fail there
//! by the fixture's absence read at run time, and the
//! three relation pins are compile absent by the
//! standings module's absence in their standings
//! half, and the dimension and requirement pins are
//! additionally able to fail against a miscounted
//! construction on the counts they read.
//! The constructed states collection of Round 20 is
//! pinned by dimension pins read off the fixture
//! itself under the parameters the preamble of the
//! Round 20 section states, by the seven shape
//! requirements that preamble names, counted from the
//! states reading over the collection, and by the
//! closing relations of the states reading against
//! the verdicts reading, the standings reading, and
//! the judgements reading over the same collection
//! and against its own section counts. The fixture
//! file stands absent against the tree Round 19
//! closed on, so the dimension and requirement pins
//! fail there by the fixture's absence read at run
//! time, and the four relation pins are compile
//! absent by the states module's absence in their
//! states half, and the dimension and requirement
//! pins are additionally able to fail against a
//! miscounted construction on the counts they read.
//! The constructed flags collection of Round 21 is
//! pinned by dimension pins read off the fixture
//! itself under the parameters the preamble of the
//! Round 21 section states, by the eight shape
//! requirements that preamble names, counted from
//! the flags reading, the verdicts reading, and
//! the files reading over the collection, and by
//! the closing relations of the flags reading
//! against the census, the files reading, and the
//! states reading over the same collection and
//! against its own section counts. The fixture
//! file stands absent against the tree Round 20
//! closed on, so the dimension and requirement
//! pins fail there by the fixture's absence read
//! at run time, and the four relation pins are
//! compile absent by the flags module's absence
//! in their flags half, and the dimension and
//! requirement pins are additionally able to fail
//! against a miscounted construction on the
//! counts they read.
//! Every pin runs offline on the constructed fixtures, and no
//! pin makes a live call.

use std::path::PathBuf;

use stickler::borderline::{borderline_reading_of, load_borderline};
use stickler::categories::{categories_reading_of, load_categories};
use stickler::census::{census_of, load_census};
use stickler::changeset::load_changeset;
use stickler::checker::check_diff_detailed;
use stickler::choices::{choices_reading_of, load_choices};
use stickler::cli::cli_main;
use stickler::files::files_reading_of;
use stickler::flags::{flags_reading_of, load_flags};
use stickler::grounds::{grounds_reading_of, load_grounds};
use stickler::json::{parse, JsonValue};
use stickler::judgements::{judgements_reading_of, load_judgements};
use stickler::outcomes::outcomes_reading_of;
use stickler::pattern::pattern_reading_of;
use stickler::render::{
    render_borderline, render_categories, render_choices, render_flags, render_grounds,
    render_judgements, render_shapes, render_standings, render_states, render_verdicts,
};
use stickler::shapes::{load_shapes, shapes_reading_of};
use stickler::standings::{load_standings, standings_reading_of};
use stickler::states::{load_states, states_reading_of};
use stickler::verdicts::{is_abstention, is_contested, line_outcome, verdict_outcome};
use stickler::verdicts_reading::{load_verdicts, verdicts_reading_of};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn read_example(name: &str) -> String {
    std::fs::read_to_string(examples().join(name)).unwrap()
}

fn parsed_example(name: &str) -> JsonValue {
    parse(&read_example(name)).unwrap()
}

fn run(args: &[&str]) -> (i32, String, String) {
    let owned: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    let result = cli_main(&owned);
    (result.code, result.stdout, result.stderr)
}

fn run_on_example(command: &str, name: &str) -> (i32, String) {
    let path = examples().join(name);
    let (code, out, _err) = run(&[command, path.to_str().unwrap()]);
    (code, out)
}

/// The number of added lines in one file diff, the lines that
/// open with a plus sign other than the file header line.
fn added_lines(diff: &str) -> usize {
    diff.lines()
        .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
        .count()
}

/// The number of hunks in one file diff.
fn hunks(diff: &str) -> usize {
    diff.lines().filter(|line| line.starts_with("@@")).count()
}

/// The file entries of a parsed change set value.
fn file_entries(value: &JsonValue) -> Vec<(String, String)> {
    value
        .get("files")
        .and_then(|files| files.as_array())
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry
                    .get("file")
                    .and_then(|file| file.as_jstring())
                    .unwrap()
                    .to_rust_string()
                    .unwrap(),
                entry
                    .get("diff")
                    .and_then(|diff| diff.as_jstring())
                    .unwrap()
                    .to_rust_string()
                    .unwrap(),
            )
        })
        .collect()
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

#[test]
fn large_set_dimensions_hold() {
    let value = parsed_example("large_set.json");
    assert!(load_changeset(&value).is_empty());
    let entries = file_entries(&value);
    assert_eq!(entries.len(), 24);
    let added: Vec<usize> = entries.iter().map(|(_, diff)| added_lines(diff)).collect();
    assert_eq!(
        added,
        vec![
            22, 21, 20, 19, 18, 18, 17, 17, 16, 16, 16, 15, 15, 15, 16, 14, 20, 16, 19, 18, 15, 14,
            16, 17
        ]
    );
    assert_eq!(added.iter().sum::<usize>(), 410);
    assert!(added.iter().all(|count| (1..=40).contains(count)));
    let hunk_counts: Vec<usize> = entries.iter().map(|(_, diff)| hunks(diff)).collect();
    assert_eq!(
        hunk_counts,
        vec![3, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 2, 3, 2, 2, 2, 3, 2, 3, 2, 3, 2, 2, 3]
    );
    assert!(hunk_counts.iter().all(|count| (2..=6).contains(count)));
}

#[test]
fn large_set_review_states_the_constructed_tallies() {
    let (code, out) = run_on_example("review", "large_set.json");
    assert_eq!(code, 1);
    for line in [
        "Judgements, 56",
        "Shape, bare except, 12",
        "Shape, print call, 13",
        "Shape, mutable default, 9",
        "Shape, placeholder name, 11",
        "Shape, call default, 9",
        "Shape, class attribute, 2",
        "Borderline, 16",
        "Verdict lines, stickler 3, clean 2",
        "Unjudged, 11",
        "Contested, 1",
    ] {
        assert!(out.contains(line), "the review states {line}");
    }
    let worst: Vec<&str> = out
        .lines()
        .filter(|line| line.starts_with("Worst, "))
        .collect();
    assert_eq!(worst.len(), 21);
    assert_eq!(
        worst[..4],
        [
            "Worst, anchor.py, 5",
            "Worst, beacon.py, 5",
            "Worst, cobble.py, 5",
            "Worst, dagger.py, 4"
        ]
    );
}

#[test]
fn large_collection_embeds_both_sets_whole() {
    let collection = parsed_example("large_collection.json");
    let large = parsed_example("large_set.json");
    let census = parsed_example("census.json");
    assert!(load_census(&collection).is_empty());
    assert!(load_borderline(&collection).is_empty());
    let sets = collection
        .get("sets")
        .and_then(|sets| sets.as_array())
        .unwrap();
    assert_eq!(sets.len(), 2);
    let labels: Vec<String> = sets
        .iter()
        .map(|set| {
            set.get("label")
                .and_then(|label| label.as_jstring())
                .unwrap()
                .to_rust_string()
                .unwrap()
        })
        .collect();
    assert_eq!(labels, ["constructed large set", "pilot set"]);
    let embedded_large = sets[0].get("change_set").unwrap();
    assert_eq!(embedded_large, &large);
    let census_sets = census.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(&sets[1], &census_sets[1]);
}

#[test]
fn large_collection_census_totals_are_the_sums_of_its_set_blocks() {
    let collection = parsed_example("large_collection.json");
    let census = census_of(&collection);
    assert_eq!(census.sets.len(), 2);
    let total = &census.total;
    let sum = |pick: &dyn Fn(&stickler::census::Tally) -> usize| {
        census
            .sets
            .iter()
            .map(|set| pick(&set.tally))
            .sum::<usize>()
    };
    assert_eq!(total.files, sum(&|tally| tally.files));
    assert_eq!(total.judgements, sum(&|tally| tally.judgements));
    assert_eq!(total.borderline, sum(&|tally| tally.borderline));
    assert_eq!(total.stickler, sum(&|tally| tally.stickler));
    assert_eq!(total.clean, sum(&|tally| tally.clean));
    assert_eq!(total.unjudged, sum(&|tally| tally.unjudged));
    assert_eq!(total.contested, sum(&|tally| tally.contested));
    for index in 0..6 {
        assert_eq!(
            total.shapes[index],
            census
                .sets
                .iter()
                .map(|set| set.tally.shapes[index])
                .sum::<usize>()
        );
    }
    for index in 0..3 {
        assert_eq!(
            total.patterns[index],
            census
                .sets
                .iter()
                .map(|set| set.tally.patterns[index])
                .sum::<usize>()
        );
    }
    assert_eq!(total.files, 27);
    assert_eq!(total.judgements, 56);
    assert_eq!(total.borderline, 28);
    assert_eq!(total.stickler, 14);
    assert_eq!(total.clean, 2);
    assert_eq!(total.unjudged, 12);
    assert_eq!(total.contested, 1);
}

#[test]
fn large_collection_closing_relations_hold() {
    let collection = parsed_example("large_collection.json");
    let borderline = render_borderline(&borderline_reading_of(&collection));
    let (_, census_out) = run_on_example("census", "large_collection.json");
    let total_block = census_out
        .split("\n\n")
        .find(|block| block.starts_with("Total\n"))
        .unwrap();
    let large_block = census_out
        .split("\n\n")
        .find(|block| block.starts_with("Set, constructed large set\n"))
        .unwrap();
    assert_eq!(
        count_after(&borderline, "Borderline flags, "),
        count_after(total_block, "Borderline, ")
    );
    assert_eq!(count_after(total_block, "Borderline, "), 28);
    let (_, review_out) = run_on_example("review", "large_set.json");
    assert_eq!(
        count_after(&review_out, "Borderline, "),
        count_after(large_block, "Borderline, ")
    );
    assert_eq!(count_after(large_block, "Borderline, "), 16);
}

/// The per file added line and hunk counts of the large diff,
/// split on its file header lines.
fn large_diff_dimensions() -> Vec<(usize, usize)> {
    let text = read_example("large_check.diff");
    let mut dimensions = Vec::new();
    let mut current: Option<(usize, usize)> = None;
    for line in text.lines() {
        if line.starts_with("+++ b/") {
            if let Some(done) = current.take() {
                dimensions.push(done);
            }
            current = Some((0, 0));
        } else if let Some((added, hunks)) = &mut current {
            if line.starts_with("@@") {
                *hunks += 1;
            } else if line.starts_with('+') && !line.starts_with("+++") {
                *added += 1;
            }
        }
    }
    if let Some(done) = current.take() {
        dimensions.push(done);
    }
    dimensions
}

#[test]
fn large_diff_dimensions_hold() {
    let dimensions = large_diff_dimensions();
    assert_eq!(dimensions.len(), 8);
    let added: Vec<usize> = dimensions.iter().map(|(added, _)| *added).collect();
    assert_eq!(added, vec![42, 40, 38, 36, 38, 36, 35, 35]);
    assert_eq!(added.iter().sum::<usize>(), 300);
    let hunk_counts: Vec<usize> = dimensions.iter().map(|(_, hunks)| *hunks).collect();
    assert_eq!(hunk_counts, vec![5, 4, 6, 3, 5, 4, 6, 3]);
    assert!(hunk_counts.iter().all(|count| (3..=8).contains(count)));
}

#[test]
fn large_diff_shape_counts_stand_as_literals() {
    let text = read_example("large_check.diff");
    let detailed = check_diff_detailed(&text);
    assert_eq!(detailed.len(), 40);
    for (shape, count) in [
        ("bare except", 7),
        ("print call", 9),
        ("mutable default", 6),
        ("placeholder name", 8),
        ("call default", 6),
        ("class attribute", 4),
    ] {
        assert_eq!(
            detailed.iter().filter(|(name, _)| *name == shape).count(),
            count,
            "the shape {shape} stands at its constructed count"
        );
    }
    let (code, out) = run_on_example("check", "large_check.diff");
    assert_eq!(code, 1);
    assert!(out.contains("Objections, 40"));
}

/// The dimension pins of the large verdicts collection,
/// read off the fixture itself under the parameters the
/// preamble of CK-27 states. The pin fails against the
/// tree Round 12 closed on by the absence of the fixture
/// it reads.
#[test]
fn verdicts_collection_dimensions_hold() {
    let value = parsed_example("verdicts_collection.json");
    assert!(load_verdicts(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 10);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (4..=12).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
    let census = census_of(&value);
    assert_eq!(census.sets.len(), 10);
    let flags = census.total.borderline;
    assert!(
        (120..=200).contains(&flags),
        "the collection holds {flags} flagged lines"
    );
    let reading = verdicts_reading_of(&value);
    assert_eq!(reading.flags_read, flags);
    let groups: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .collect();
    let carrying = groups.len();
    assert!(
        carrying * 100 >= 55 * flags && carrying * 100 <= 75 * flags,
        "{carrying} of {flags} flagged lines carry verdicts"
    );
    let verdicts_total: usize = groups.iter().map(|group| group.verdicts.len()).sum();
    assert!(
        (90..=160).contains(&verdicts_total),
        "the collection holds {verdicts_total} verdicts"
    );
    assert!(groups
        .iter()
        .all(|group| (1..=2).contains(&group.verdicts.len())));
    let abstentions = groups
        .iter()
        .flat_map(|group| group.verdicts.iter())
        .filter(|verdict| verdict_outcome(verdict) == "abstained")
        .count();
    assert!(
        abstentions * 100 >= 5 * verdicts_total && abstentions * 100 <= 15 * verdicts_total,
        "{abstentions} of {verdicts_total} verdicts abstained"
    );
    assert!(groups
        .iter()
        .flat_map(|group| group.verdicts.iter())
        .all(|verdict| verdict.model == "jev-1.13.0" || verdict.model == "openai/gpt-6-luna"));
}

/// The four shape requirements the preamble of CK-27
/// states over the large verdicts collection, counted
/// from the reading the collection builds. The pin
/// fails against the tree Round 12 closed on by the
/// absence of the module it names.
#[test]
fn verdicts_collection_shape_requirements_hold() {
    let value = parsed_example("verdicts_collection.json");
    let reading = verdicts_reading_of(&value);
    assert!(
        reading
            .sections
            .iter()
            .any(|section| section.groups.is_empty()),
        "one set carries no verdicts at all"
    );
    let groups: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .collect();
    assert!(
        groups.iter().any(|group| is_contested(&group.verdicts)),
        "one flag is contested"
    );
    assert!(
        groups
            .iter()
            .any(|group| line_outcome(&group.verdicts) == "unjudged"),
        "one flag carries only abstentions"
    );
    assert_eq!(
        groups
            .iter()
            .filter(|group| line_outcome(&group.verdicts) == "unjudged")
            .count(),
        3,
        "exactly three unjudged flags carry verdicts"
    );
}

/// The closing relations of the verdicts reading over
/// the large verdicts collection, against the census
/// and the outcomes reading over the same collection
/// and against its own section counts. The pin fails
/// against the tree Round 12 closed on by the absence
/// of the module and the command it names.
#[test]
fn verdicts_collection_closing_relations_hold() {
    let (code, out) = run_on_example("verdicts", "verdicts_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("verdicts_collection.json");
    let census = census_of(&value);
    let outcomes = outcomes_reading_of(&value);
    let closings = out.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
    assert_eq!(
        count_after(closings, "Flags with verdicts, "),
        outcomes.sections[0].flags.len() + outcomes.sections[1].flags.len() + 3
    );
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
    let section_counts: Vec<usize> = out
        .lines()
        .filter(|line| line.starts_with("Verdicts read, "))
        .map(|line| {
            line.strip_prefix("Verdicts read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 11);
    assert_eq!(
        section_counts[..10].iter().sum::<usize>(),
        section_counts[10]
    );
    assert_eq!(section_counts[10], count_after(closings, "Verdicts read, "));
    assert_eq!(out, render_verdicts(&verdicts_reading_of(&value)));
}

/// The dimension pins of the constructed shapes
/// collection, read off the fixture itself under the
/// parameters the preamble of the Round 14 section
/// states. The pin fails against the tree Round 13
/// closed on by the absence of the fixture it reads.
#[test]
fn shapes_collection_dimensions_hold() {
    let value = parsed_example("shapes_collection.json");
    assert!(load_shapes(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
        assert!(
            files.iter().all(|file| !file.contains_key("verdicts")),
            "no verdict stands in the set {label}"
        );
    }
    let census = census_of(&value);
    assert!(
        (24..=40).contains(&census.total.judgements),
        "the collection holds {} judgements",
        census.total.judgements
    );
}

/// The first shape requirement of the constructed
/// shapes collection, each set carrying judgements
/// under at least four of the six shapes, counted from
/// the census tallies of its sets. The pin fails
/// against the tree Round 13 closed on by the absence
/// of the fixture it reads, and against a miscounted
/// construction by the tallies it reads.
#[test]
fn shapes_collection_each_set_carries_four_shapes() {
    let value = parsed_example("shapes_collection.json");
    let census = census_of(&value);
    assert_eq!(census.sets.len(), 2);
    for set in &census.sets {
        let standing = set.tally.shapes.iter().filter(|count| **count > 0).count();
        assert!(
            standing >= 4,
            "the set {} carries {standing} shapes",
            set.label
        );
    }
}

/// The second shape requirement, at least two shapes
/// carrying judgements from both sets, counted from
/// the census tallies of its sets.
#[test]
fn shapes_collection_two_shapes_carry_both_sets() {
    let value = parsed_example("shapes_collection.json");
    let census = census_of(&value);
    let shared = (0..6)
        .filter(|shape| census.sets.iter().all(|set| set.tally.shapes[*shape] > 0))
        .count();
    assert!(
        shared >= 2,
        "{shared} shapes carry judgements from both sets"
    );
}

/// The third shape requirement, the placeholder name
/// shape carrying at least two judgements in each set,
/// counted from the census tallies of its sets.
#[test]
fn shapes_collection_placeholder_name_stands_twice_in_each_set() {
    let value = parsed_example("shapes_collection.json");
    let census = census_of(&value);
    for set in &census.sets {
        assert!(
            set.tally.shapes[3] >= 2,
            "the set {} carries {} placeholder name judgements",
            set.label,
            set.tally.shapes[3]
        );
    }
}

/// The fourth shape requirement, every shape's
/// collection total standing at least at two, counted
/// from the census total tally.
#[test]
fn shapes_collection_every_shape_total_stands_at_two() {
    let value = parsed_example("shapes_collection.json");
    let census = census_of(&value);
    for (shape, count) in census.total.shapes.iter().enumerate() {
        assert!(
            *count >= 2,
            "shape {shape} carries {count} judgements in total"
        );
    }
}

/// The fifth shape requirement, at least one file in
/// each set carrying two judgements of different
/// shapes, counted from the shapes reading, a file
/// standing in two sections under one set.
#[test]
fn shapes_collection_a_file_in_each_set_stands_in_two_sections() {
    let value = parsed_example("shapes_collection.json");
    let reading = shapes_reading_of(&value);
    let labels: Vec<String> = reading
        .sections
        .iter()
        .flat_map(|section| {
            section
                .judgements
                .iter()
                .map(|judgement| judgement.set_label.clone())
        })
        .collect();
    for label in ["constructed shapes set 1", "constructed shapes set 2"] {
        assert!(labels.iter().any(|held| held == label));
        let stands_twice = reading.sections.iter().enumerate().any(|(index, section)| {
            section.judgements.iter().any(|first| {
                first.set_label == label
                    && reading
                        .sections
                        .iter()
                        .enumerate()
                        .any(|(other_index, other)| {
                            other_index != index
                                && other
                                    .judgements
                                    .iter()
                                    .any(|held| held.set_label == label && held.file == first.file)
                        })
            })
        });
        assert!(
            stands_twice,
            "one file of the set {label} stands in two sections"
        );
    }
}

/// The sixth shape requirement, at least one file
/// carrying two judgements of the same shape, counted
/// from the shapes reading, one section holding two
/// lines over one file of one set.
#[test]
fn shapes_collection_a_file_carries_two_judgements_of_one_shape() {
    let value = parsed_example("shapes_collection.json");
    let reading = shapes_reading_of(&value);
    let stands = reading.sections.iter().any(|section| {
        section.judgements.iter().any(|first| {
            section
                .judgements
                .iter()
                .filter(|held| held.file == first.file && held.set_label == first.set_label)
                .count()
                >= 2
        })
    });
    assert!(stands, "one file carries two judgements of one shape");
}

/// The closing relations of the shapes reading over
/// the constructed shapes collection, against the
/// census over the same collection and against its
/// own section counts. The pin fails against the tree
/// Round 13 closed on by the absence of the module and
/// the command it names, and its dimension companions
/// fail there by the fixture's absence.
#[test]
fn shapes_collection_closing_relations_hold() {
    let (code, out) = run_on_example("shapes", "shapes_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("shapes_collection.json");
    let census = census_of(&value);
    let reading = shapes_reading_of(&value);
    for (index, section) in reading.sections.iter().enumerate() {
        assert_eq!(
            section.judgements.len(),
            census.total.shapes[index],
            "section {index} binds to the census shape line for its shape"
        );
    }
    let closings = out.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Judgements read, "),
        census.total.judgements
    );
    let section_counts: Vec<usize> = out
        .lines()
        .filter(|line| line.starts_with("Judgements read, "))
        .map(|line| {
            line.strip_prefix("Judgements read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 7);
    assert_eq!(section_counts[..6].iter().sum::<usize>(), section_counts[6]);
    assert_eq!(out, render_shapes(&reading));
}

/// The first dimension pin of the constructed
/// categories collection, read off the fixture itself
/// under the parameters the preamble of the Round 16
/// section states. The pin fails against the tree
/// Round 15 closed on by the absence of the fixture
/// it reads, and against a miscounted construction
/// on the counts it reads.
#[test]
fn categories_collection_holds_two_sets_named_as_constructed() {
    let value = parsed_example("categories_collection.json");
    assert!(load_categories(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
    }
}

/// The second dimension pin of the constructed
/// categories collection, each set holding between
/// three and six files, read off the fixture itself.
#[test]
fn categories_collection_files_per_set_stand_between_three_and_six() {
    let value = parsed_example("categories_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
}

/// The third dimension pin of the constructed
/// categories collection, the judgements across the
/// collection standing between twenty four and forty,
/// counted from the census over the collection.
#[test]
fn categories_collection_judgements_stand_between_twenty_four_and_forty() {
    let value = parsed_example("categories_collection.json");
    let census = census_of(&value);
    assert!(
        (24..=40).contains(&census.total.judgements),
        "the collection holds {} judgements",
        census.total.judgements
    );
}

/// The fourth dimension pin of the constructed
/// categories collection, no verdict standing in
/// either set, read off the fixture itself.
#[test]
fn categories_collection_no_verdict_stands_in_either_set() {
    let value = parsed_example("categories_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            files.iter().all(|file| !file.contains_key("verdicts")),
            "no verdict stands in the set {label}"
        );
    }
}

/// The first shape requirement of the constructed
/// categories collection, every category carrying
/// judgements from both sets, counted from the
/// categories reading, each section holding at least
/// one judgement line carrying each set's label.
#[test]
fn categories_collection_every_category_carries_both_sets() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    for section in &reading.sections {
        for label in [
            "constructed categories set 1",
            "constructed categories set 2",
        ] {
            assert!(
                section
                    .judgements
                    .iter()
                    .any(|judgement| judgement.set_label == label),
                "the category {} carries no judgement of the set {label}",
                section.category
            );
        }
    }
}

/// The second shape requirement, the naming category
/// carrying at least two judgements in each set,
/// counted from the categories reading.
#[test]
fn categories_collection_naming_carries_two_in_each_set() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    let naming = reading
        .sections
        .iter()
        .find(|section| section.category == "naming")
        .unwrap();
    for label in [
        "constructed categories set 1",
        "constructed categories set 2",
    ] {
        let held = naming
            .judgements
            .iter()
            .filter(|judgement| judgement.set_label == label)
            .count();
        assert!(
            held >= 2,
            "the set {label} carries {held} naming judgements"
        );
    }
}

/// The third shape requirement, every category's
/// collection total standing at least at four,
/// counted from the categories reading.
#[test]
fn categories_collection_every_category_total_stands_at_four() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    for section in &reading.sections {
        assert!(
            section.judgements.len() >= 4,
            "the category {} carries {} judgements in total",
            section.category,
            section.judgements.len()
        );
    }
}

/// The fourth shape requirement, every one of the six
/// shapes carrying at least one judgement, counted
/// from the census total tally over the collection.
#[test]
fn categories_collection_every_shape_carries_a_judgement() {
    let value = parsed_example("categories_collection.json");
    let census = census_of(&value);
    for (shape, count) in census.total.shapes.iter().enumerate() {
        assert!(
            *count >= 1,
            "shape {shape} carries {count} judgements in total"
        );
    }
}

/// The fifth shape requirement, at least one file in
/// each set carrying two judgements of different
/// categories, counted from the categories reading,
/// a file standing in two sections under one set.
#[test]
fn categories_collection_a_file_in_each_set_stands_in_two_category_sections() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    let labels: Vec<String> = reading
        .sections
        .iter()
        .flat_map(|section| {
            section
                .judgements
                .iter()
                .map(|judgement| judgement.set_label.clone())
        })
        .collect();
    for label in [
        "constructed categories set 1",
        "constructed categories set 2",
    ] {
        assert!(labels.iter().any(|held| held == label));
        let stands_twice = reading.sections.iter().enumerate().any(|(index, section)| {
            section.judgements.iter().any(|first| {
                first.set_label == label
                    && reading
                        .sections
                        .iter()
                        .enumerate()
                        .any(|(other_index, other)| {
                            other_index != index
                                && other
                                    .judgements
                                    .iter()
                                    .any(|held| held.set_label == label && held.file == first.file)
                        })
            })
        });
        assert!(
            stands_twice,
            "one file of the set {label} stands in two sections"
        );
    }
}

/// The sixth shape requirement, at least one file
/// carrying two judgements of the same category,
/// counted from the categories reading, one section
/// holding two lines over one file of one set.
#[test]
fn categories_collection_a_file_carries_two_judgements_of_one_category() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    let stands = reading.sections.iter().any(|section| {
        section.judgements.iter().any(|first| {
            section
                .judgements
                .iter()
                .filter(|held| held.file == first.file && held.set_label == first.set_label)
                .count()
                >= 2
        })
    });
    assert!(stands, "one file carries two judgements of one category");
}

/// The shapes relation of the constructed categories
/// collection, each categories section's count equal
/// to the count of that category's judgement lines in
/// the shapes reading over the same collection, the
/// two readings stating the same judgements under two
/// organizations, by shape and by category. The pin
/// is compile absent against the tree Round 15 closed
/// on by the absence of the categories module it
/// names.
#[test]
fn categories_collection_sections_regroup_the_shapes_reading() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    let shapes = shapes_reading_of(&value);
    for section in &reading.sections {
        let regrouped = shapes
            .sections
            .iter()
            .flat_map(|shape_section| shape_section.judgements.iter())
            .filter(|judgement| judgement.category == section.category)
            .count();
        assert_eq!(
            section.judgements.len(),
            regrouped,
            "section {} regroups the shapes reading",
            section.category
        );
    }
}

/// The census relation of the constructed categories
/// collection, the categories closings over the
/// collection equal to the census set count and the
/// census total judgements count over the same
/// collection, and the command output standing byte
/// identical to the library rendering.
#[test]
fn categories_collection_closings_bind_to_the_census() {
    let (code, out) = run_on_example("categories", "categories_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("categories_collection.json");
    let census = census_of(&value);
    let reading = categories_reading_of(&value);
    let closings = out.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Judgements read, "),
        census.total.judgements
    );
    assert_eq!(out, render_categories(&reading));
}

/// The internal sums of the categories reading over
/// the constructed categories collection, the section
/// counts summing to the closing judgements read
/// count, read from the rendered text, and each
/// shape's count summed across the three category
/// sections equal to the shapes reading's section
/// count for that shape, the categories judgement
/// lines carrying their shape names.
#[test]
fn categories_collection_internal_sums_hold() {
    let value = parsed_example("categories_collection.json");
    let reading = categories_reading_of(&value);
    let rendered = render_categories(&reading);
    let section_counts: Vec<usize> = rendered
        .lines()
        .filter(|line| line.starts_with("Judgements read, "))
        .map(|line| {
            line.strip_prefix("Judgements read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 4);
    assert_eq!(section_counts[..3].iter().sum::<usize>(), section_counts[3]);
    let shapes = shapes_reading_of(&value);
    for section in &shapes.sections {
        let across = reading
            .sections
            .iter()
            .flat_map(|category_section| category_section.judgements.iter())
            .filter(|judgement| judgement.shape == section.shape)
            .count();
        assert_eq!(
            across,
            section.judgements.len(),
            "shape {} sums across the category sections",
            section.shape
        );
    }
}

/// The dimension pins of the constructed choices
/// collection, read off the fixture itself under the
/// parameters the preamble of the Round 15 section
/// states. The pin fails against the tree Round 14
/// closed on by the absence of the fixture it reads.
#[test]
fn choices_collection_dimensions_hold() {
    let value = parsed_example("choices_collection.json");
    assert!(load_choices(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert!(
        (6..=10).contains(&sets.len()),
        "the collection holds {} sets",
        sets.len()
    );
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=8).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
    let census = census_of(&value);
    let flags = census.total.borderline;
    assert!(
        (60..=120).contains(&flags),
        "the collection holds {flags} flagged lines"
    );
    let reading = choices_reading_of(&value);
    assert_eq!(reading.flags_read, flags);
    let carrying = reading.flags_with_verdicts;
    assert!(
        carrying * 100 >= 55 * flags && carrying * 100 <= 75 * flags,
        "{carrying} of {flags} flagged lines carry verdicts"
    );
    assert!(
        (60..=140).contains(&reading.verdicts_read),
        "the collection holds {} verdicts",
        reading.verdicts_read
    );
    let groups: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .collect();
    assert!(groups
        .iter()
        .all(|group| (1..=2).contains(&group.verdicts.len())));
    let pairs: Vec<_> = groups
        .iter()
        .flat_map(|group| group.verdicts.iter())
        .collect();
    assert_eq!(pairs.len(), reading.verdicts_read);
    let abstentions = pairs
        .iter()
        .filter(|verdict| verdict_outcome(verdict) == "abstained")
        .count();
    assert!(
        abstentions * 100 >= 5 * reading.verdicts_read
            && abstentions * 100 <= 15 * reading.verdicts_read,
        "{abstentions} of {} verdicts abstained",
        reading.verdicts_read
    );
    assert!(pairs
        .iter()
        .all(|verdict| verdict.model == "jev-1.13.0" || verdict.model == "openai/gpt-6-luna"));
}

/// The first shape requirement of the constructed
/// choices collection, at least two contested flags,
/// counted from the verdicts reading over the
/// collection, a contested flag standing in both
/// label sections of the choices reading. The pin
/// fails against the tree Round 14 closed on by the
/// absence of the fixture it reads.
#[test]
fn choices_collection_two_flags_are_contested() {
    let value = parsed_example("choices_collection.json");
    let reading = verdicts_reading_of(&value);
    let groups: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .collect();
    let contested = groups
        .iter()
        .filter(|group| is_contested(&group.verdicts))
        .count();
    assert!(contested >= 2, "{contested} flags are contested");
}

/// The second shape requirement, at least one flag
/// carrying an abstention beside a standing verdict,
/// counted from the choices reading, a flag standing
/// in the abstained section and in a label section.
#[test]
fn choices_collection_an_abstention_stands_beside_a_verdict() {
    let value = parsed_example("choices_collection.json");
    let reading = choices_reading_of(&value);
    let keys = |section: &stickler::choices::ChoicesSection| {
        section
            .groups
            .iter()
            .map(|group| (group.file.clone(), group.line, group.set_label.clone()))
            .collect::<Vec<_>>()
    };
    let abstained = keys(&reading.sections[2]);
    let labelled = [keys(&reading.sections[0]), keys(&reading.sections[1])].concat();
    assert!(
        abstained.iter().any(|key| labelled.contains(key)),
        "one flag stands in the abstained section and in a label section"
    );
}

/// The third shape requirement, at least one flag
/// carrying only abstentions, counted from the
/// choices reading, a flag standing in the abstained
/// section and in no label section.
#[test]
fn choices_collection_a_flag_carries_only_abstentions() {
    let value = parsed_example("choices_collection.json");
    let reading = choices_reading_of(&value);
    let keys = |section: &stickler::choices::ChoicesSection| {
        section
            .groups
            .iter()
            .map(|group| (group.file.clone(), group.line, group.set_label.clone()))
            .collect::<Vec<_>>()
    };
    let abstained = keys(&reading.sections[2]);
    let labelled = [keys(&reading.sections[0]), keys(&reading.sections[1])].concat();
    assert!(
        abstained.iter().any(|key| !labelled.contains(key)),
        "one flag stands in the abstained section alone"
    );
    let verdicts = verdicts_reading_of(&value);
    assert!(verdicts
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| line_outcome(&group.verdicts) == "unjudged"));
}

/// The fourth shape requirement, at least one flag
/// carrying two verdicts that chose the same label,
/// counted from the verdicts reading over the
/// collection, two standing verdicts of one flag
/// sharing their choice.
#[test]
fn choices_collection_a_flag_carries_two_verdicts_choosing_the_same_label() {
    let value = parsed_example("choices_collection.json");
    let reading = verdicts_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            let standing: Vec<_> = group
                .verdicts
                .iter()
                .filter(|verdict| verdict_outcome(verdict) != "abstained")
                .collect();
            standing.len() == 2 && standing[0].choice == standing[1].choice
        });
    assert!(
        stands,
        "one flag carries two verdicts choosing the same label"
    );
}

/// The fifth shape requirement, at least one set
/// carrying no verdicts at all, counted from the
/// verdicts reading over the collection.
#[test]
fn choices_collection_a_set_carries_no_verdicts() {
    let value = parsed_example("choices_collection.json");
    let reading = verdicts_reading_of(&value);
    assert!(
        reading
            .sections
            .iter()
            .any(|section| section.groups.is_empty()),
        "one set carries no verdicts at all"
    );
}

/// The sixth shape requirement, every outcome section
/// carrying verdicts from at least two sets, counted
/// from the choices reading, the set labels standing
/// in each section spanning two or more sets.
#[test]
fn choices_collection_every_section_spans_two_sets() {
    let value = parsed_example("choices_collection.json");
    let reading = choices_reading_of(&value);
    for section in &reading.sections {
        let mut labels: Vec<&str> = section
            .groups
            .iter()
            .map(|group| group.set_label.as_str())
            .collect();
        labels.sort_unstable();
        labels.dedup();
        assert!(
            labels.len() >= 2,
            "the section {} spans {} sets",
            section.outcome,
            labels.len()
        );
    }
}

/// The closing relations of the choices reading over
/// the constructed choices collection against the
/// verdicts reading over the same collection, each of
/// the seven closing counts standing count for count
/// against the same named count of the verdicts
/// reading. The pin fails against the tree Round 14
/// closed on by the absence of the module and the
/// command it names.
#[test]
fn choices_collection_closings_bind_to_the_verdicts_reading() {
    let (code, out) = run_on_example("choices", "choices_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("choices_collection.json");
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let choices_closings = out.split("\n\n").last().unwrap();
    let verdicts_closings = verdicts.split("\n\n").last().unwrap();
    for prefix in [
        "Sets read, ",
        "Flags read, ",
        "Flags with verdicts, ",
        "Verdicts read, ",
        "Chose stickler, ",
        "Chose clean, ",
        "Abstained, ",
    ] {
        assert_eq!(
            count_after(choices_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
}

/// The closing relations of the choices reading over
/// the constructed choices collection against the
/// census over the same collection, the flags read
/// and sets read counts standing against the census
/// totals.
#[test]
fn choices_collection_closings_bind_to_the_census() {
    let (code, out) = run_on_example("choices", "choices_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("choices_collection.json");
    let census = census_of(&value);
    let closings = out.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
}

/// The internal sums of the choices reading over the
/// constructed choices collection, its outcome counts
/// summing to its verdicts read count and its section
/// counts summing to the same.
#[test]
fn choices_collection_internal_sums_hold() {
    let (code, out) = run_on_example("choices", "choices_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("choices_collection.json");
    let closings = out.split("\n\n").last().unwrap();
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
    let section_counts: Vec<usize> = out
        .lines()
        .filter(|line| line.starts_with("Verdicts read, "))
        .map(|line| {
            line.strip_prefix("Verdicts read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 4);
    assert_eq!(section_counts[..3].iter().sum::<usize>(), section_counts[3]);
    assert_eq!(section_counts[3], count_after(closings, "Verdicts read, "));
    assert_eq!(out, render_choices(&choices_reading_of(&value)));
}

/// The first dimension pin of the constructed grounds
/// collection, read off the fixture itself under the
/// parameters the preamble of the Round 17 section
/// states. The whole file is compile absent against
/// the tree Round 16 closed on by the grounds module
/// it imports, and the pin can fail against a
/// miscounted construction on the counts it reads.
#[test]
fn grounds_collection_holds_two_sets_named_as_constructed() {
    let value = parsed_example("grounds_collection.json");
    assert!(load_grounds(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
    }
}

/// The second dimension pin of the constructed grounds
/// collection, each set holding between three and six
/// files, read off the fixture itself.
#[test]
fn grounds_collection_files_per_set_stand_between_three_and_six() {
    let value = parsed_example("grounds_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
}

/// The third dimension pin of the constructed grounds
/// collection, the flagged lines across the collection
/// standing between sixteen and twenty eight, counted
/// from the census over the collection.
#[test]
fn grounds_collection_flagged_lines_stand_between_sixteen_and_twenty_eight() {
    let value = parsed_example("grounds_collection.json");
    let census = census_of(&value);
    assert!(
        (16..=28).contains(&census.total.borderline),
        "the collection holds {} flagged lines",
        census.total.borderline
    );
}

/// The fourth dimension pin of the constructed grounds
/// collection, the verdicts across the collection
/// standing between twenty four and forty four,
/// counted from the verdicts reading over the
/// collection.
#[test]
fn grounds_collection_verdicts_stand_between_twenty_four_and_forty_four() {
    let value = parsed_example("grounds_collection.json");
    let reading = verdicts_reading_of(&value);
    let verdicts: usize = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .map(|group| group.verdicts.len())
        .sum();
    assert!(
        (24..=44).contains(&verdicts),
        "the collection holds {verdicts} verdicts"
    );
}

/// The first shape requirement of the constructed
/// grounds collection, every pattern carrying verdicts
/// from both sets, counted from the grounds reading,
/// each section holding at least one flag group
/// carrying each set's label.
#[test]
fn grounds_collection_every_pattern_carries_verdicts_from_both_sets() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    for section in &reading.sections {
        for label in ["constructed grounds set 1", "constructed grounds set 2"] {
            assert!(
                section.groups.iter().any(|group| group.set_label == label),
                "the pattern {} carries no verdict of the set {label}",
                section.pattern
            );
        }
    }
}

/// The second shape requirement, every pattern's
/// collection total of pairs standing at least at
/// four, counted from the grounds reading.
#[test]
fn grounds_collection_every_pattern_total_stands_at_least_at_four() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    for section in &reading.sections {
        let pairs: usize = section
            .groups
            .iter()
            .map(|group| group.verdicts.len())
            .sum();
        assert!(
            pairs >= 4,
            "the pattern {} carries {pairs} pairs in total",
            section.pattern
        );
    }
}

/// The third shape requirement, at least one flag of
/// each pattern carrying two verdicts, counted from
/// the grounds reading, each section holding a group
/// of two or more pairs.
#[test]
fn grounds_collection_a_flag_of_each_pattern_carries_two_verdicts() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    for section in &reading.sections {
        assert!(
            section.groups.iter().any(|group| group.verdicts.len() >= 2),
            "no flag of the pattern {} carries two verdicts",
            section.pattern
        );
    }
}

/// The fourth shape requirement, at least one flag
/// carrying verdicts of both labels, counted from the
/// grounds reading, one group holding a verdict whose
/// outcome stands as stickler and one whose outcome
/// stands as clean under the one outcome helper.
#[test]
fn grounds_collection_a_flag_carries_verdicts_of_both_labels() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            group
                .verdicts
                .iter()
                .any(|verdict| verdict_outcome(verdict) == "stickler")
                && group
                    .verdicts
                    .iter()
                    .any(|verdict| verdict_outcome(verdict) == "clean")
        });
    assert!(stands, "one flag carries verdicts of both labels");
}

/// The fifth shape requirement, at least one flag
/// carrying only abstentions, counted from the grounds
/// reading, one group standing whose every verdict's
/// outcome stands as abstained under the one outcome
/// helper.
#[test]
fn grounds_collection_a_flag_carries_only_abstentions() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            !group.verdicts.is_empty()
                && group
                    .verdicts
                    .iter()
                    .all(|verdict| verdict_outcome(verdict) == "abstained")
        });
    assert!(stands, "one flag carries only abstentions");
}

/// The sixth shape requirement, at least one flag
/// carrying no verdicts at all, counted against the
/// pattern reading over the collection, a flag that
/// reading names under its pattern standing in no
/// grounds section.
#[test]
fn grounds_collection_a_flag_carries_no_verdicts() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    let pattern = pattern_reading_of(&value);
    let stands = pattern.sections.iter().any(|section| {
        section.flags.iter().any(|flag| {
            !reading
                .sections
                .iter()
                .flat_map(|grounds_section| grounds_section.groups.iter())
                .any(|group| {
                    group.file == flag.file
                        && group.line == flag.line
                        && group.set_label == flag.set_label
                })
        })
    });
    assert!(stands, "one flag carries no verdicts");
}

/// The verdicts relation of the constructed grounds
/// collection, each of the seven closing counts of the
/// grounds reading standing count for count against
/// the same named count of the verdicts reading over
/// the same collection, both readings run here, and
/// the command output standing byte identical to the
/// library rendering.
#[test]
fn grounds_collection_closings_bind_to_the_verdicts_reading() {
    let (code, out) = run_on_example("grounds", "grounds_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("grounds_collection.json");
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let grounds_closings = out.split("\n\n").last().unwrap();
    let verdicts_closings = verdicts.split("\n\n").last().unwrap();
    for prefix in [
        "Sets read, ",
        "Flags read, ",
        "Flags with verdicts, ",
        "Verdicts read, ",
        "Chose stickler, ",
        "Chose clean, ",
        "Abstained, ",
    ] {
        assert_eq!(
            count_after(grounds_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
    assert_eq!(out, render_grounds(&grounds_reading_of(&value)));
}

/// The pattern relation of the constructed grounds
/// collection, the flag groups of each grounds section
/// being exactly the flags the pattern reading names
/// under that pattern that carry at least one verdict,
/// in the same order, and each section's closing count
/// equal to the pairs standing on those flags in the
/// verdicts reading over the same collection, all
/// three readings run here.
#[test]
fn grounds_collection_sections_bind_to_the_pattern_reading() {
    let value = parsed_example("grounds_collection.json");
    let reading = grounds_reading_of(&value);
    let pattern = pattern_reading_of(&value);
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
    for (grounds_section, pattern_section) in reading.sections.iter().zip(pattern.sections.iter()) {
        assert_eq!(grounds_section.pattern, pattern_section.pattern);
        let expected: Vec<_> = pattern_section
            .flags
            .iter()
            .filter(|flag| {
                pair_count(
                    &flag.set_label,
                    &flag.file,
                    flag.line,
                    &grounds_section.pattern,
                )
                .is_some()
            })
            .collect();
        assert_eq!(
            grounds_section.groups.len(),
            expected.len(),
            "section {}",
            grounds_section.pattern
        );
        for (group, flag) in grounds_section.groups.iter().zip(expected.iter()) {
            assert_eq!(group.file, flag.file, "section {}", grounds_section.pattern);
            assert_eq!(group.line, flag.line, "section {}", grounds_section.pattern);
            assert_eq!(
                group.set_label, flag.set_label,
                "section {}",
                grounds_section.pattern
            );
        }
        let stated: usize = grounds_section
            .groups
            .iter()
            .map(|group| group.verdicts.len())
            .sum();
        let counted: usize = expected
            .iter()
            .map(|flag| {
                pair_count(
                    &flag.set_label,
                    &flag.file,
                    flag.line,
                    &grounds_section.pattern,
                )
                .unwrap()
            })
            .sum();
        assert_eq!(stated, counted, "section {}", grounds_section.pattern);
    }
}

/// The internal sums of the grounds reading over the
/// constructed grounds collection, the section closing
/// counts summing to the closing verdicts read count
/// and the three outcome counts summing to the same,
/// read from the rendered text.
#[test]
fn grounds_collection_internal_sums_hold() {
    let value = parsed_example("grounds_collection.json");
    let rendered = render_grounds(&grounds_reading_of(&value));
    let section_counts: Vec<usize> = rendered
        .lines()
        .filter(|line| line.starts_with("Verdicts read, "))
        .map(|line| {
            line.strip_prefix("Verdicts read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 4);
    assert_eq!(section_counts[..3].iter().sum::<usize>(), section_counts[3]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(section_counts[3], count_after(closings, "Verdicts read, "));
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// The first dimension pin of the constructed
/// judgements collection, read off the fixture itself
/// under the parameters the preamble of the Round 18
/// section states. The fixture stands absent against
/// the tree Round 17 closed on, so the pin fails
/// there by the fixture's absence, and the pin can
/// fail against a miscounted construction on the
/// counts it reads.
#[test]
fn judgements_collection_holds_two_sets_named_as_constructed() {
    let value = parsed_example("judgements_collection.json");
    assert!(load_judgements(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
    }
}

/// The second dimension pin of the constructed
/// judgements collection, each set holding between
/// three and six files, read off the fixture itself.
#[test]
fn judgements_collection_files_per_set_stand_between_three_and_six() {
    let value = parsed_example("judgements_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
}

/// The third dimension pin of the constructed
/// judgements collection, the flagged lines across
/// the collection standing between sixteen and twenty
/// eight, counted from the census over the collection.
#[test]
fn judgements_collection_flagged_lines_stand_between_sixteen_and_twenty_eight() {
    let value = parsed_example("judgements_collection.json");
    let census = census_of(&value);
    assert!(
        (16..=28).contains(&census.total.borderline),
        "the collection holds {} flagged lines",
        census.total.borderline
    );
}

/// The fourth dimension pin of the constructed
/// judgements collection, the verdicts across the
/// collection standing between twenty four and forty
/// four, counted from the verdicts reading over the
/// collection.
#[test]
fn judgements_collection_verdicts_stand_between_twenty_four_and_forty_four() {
    let value = parsed_example("judgements_collection.json");
    let reading = verdicts_reading_of(&value);
    let verdicts: usize = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .map(|group| group.verdicts.len())
        .sum();
    assert!(
        (24..=44).contains(&verdicts),
        "the collection holds {verdicts} verdicts"
    );
}

/// The first shape requirement of the constructed
/// judgements collection, every judgement carrying
/// verdicts from both sets, counted from the
/// judgements reading, each section holding at least
/// one flag group carrying each set's label.
#[test]
fn judgements_collection_every_judgement_carries_verdicts_from_both_sets() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    for section in &reading.sections {
        for label in [
            "constructed judgements set 1",
            "constructed judgements set 2",
        ] {
            assert!(
                section.groups.iter().any(|group| group.set_label == label),
                "the judgement {} carries no verdict of the set {label}",
                section.judgement
            );
        }
    }
}

/// The second shape requirement, every judgement's
/// collection total of pairs standing at least at
/// four, counted from the judgements reading.
#[test]
fn judgements_collection_every_judgement_total_stands_at_least_at_four() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    for section in &reading.sections {
        let pairs: usize = section
            .groups
            .iter()
            .map(|group| group.verdicts.len())
            .sum();
        assert!(
            pairs >= 4,
            "the judgement {} carries {pairs} pairs in total",
            section.judgement
        );
    }
}

/// The third shape requirement, at least one flag of
/// each judgement carrying two verdicts, counted from
/// the judgements reading, each section holding a
/// group of two or more pairs.
#[test]
fn judgements_collection_a_flag_of_each_judgement_carries_two_verdicts() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    for section in &reading.sections {
        assert!(
            section.groups.iter().any(|group| group.verdicts.len() >= 2),
            "no flag of the judgement {} carries two verdicts",
            section.judgement
        );
    }
}

/// The fourth shape requirement, at least one flag
/// carrying verdicts of both labels, counted from the
/// judgements reading, one group holding a verdict
/// whose outcome stands as stickler and one whose
/// outcome stands as clean under the one outcome
/// helper.
#[test]
fn judgements_collection_a_flag_carries_verdicts_of_both_labels() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            group
                .verdicts
                .iter()
                .any(|verdict| verdict_outcome(verdict) == "stickler")
                && group
                    .verdicts
                    .iter()
                    .any(|verdict| verdict_outcome(verdict) == "clean")
        });
    assert!(stands, "one flag carries verdicts of both labels");
}

/// The fifth shape requirement, at least one flag
/// carrying only abstentions, counted from the
/// judgements reading, one group standing whose every
/// verdict's outcome stands as abstained under the
/// one outcome helper.
#[test]
fn judgements_collection_a_flag_carries_only_abstentions() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            !group.verdicts.is_empty()
                && group
                    .verdicts
                    .iter()
                    .all(|verdict| verdict_outcome(verdict) == "abstained")
        });
    assert!(stands, "one flag carries only abstentions");
}

/// The sixth shape requirement, at least one flag
/// carrying no verdicts at all, counted against the
/// outcomes reading over the collection, a flag that
/// reading names under its judgement standing in no
/// judgements section.
#[test]
fn judgements_collection_a_flag_carries_no_verdicts() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    let outcomes = outcomes_reading_of(&value);
    let stands = outcomes.sections.iter().any(|section| {
        section.flags.iter().any(|flag| {
            !reading
                .sections
                .iter()
                .flat_map(|judgements_section| judgements_section.groups.iter())
                .any(|group| {
                    group.file == flag.file
                        && group.line == flag.line
                        && group.set_label == flag.set_label
                })
        })
    });
    assert!(stands, "one flag carries no verdicts");
}

/// The verdicts relation of the constructed judgements
/// collection, each of the seven closing counts of the
/// judgements reading standing count for count against
/// the same named count of the verdicts reading over
/// the same collection, both readings run here, and
/// the command output standing byte identical to the
/// library rendering.
#[test]
fn judgements_collection_closings_bind_to_the_verdicts_reading() {
    let (code, out) = run_on_example("judgements", "judgements_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("judgements_collection.json");
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let judgements_closings = out.split("\n\n").last().unwrap();
    let verdicts_closings = verdicts.split("\n\n").last().unwrap();
    for prefix in [
        "Sets read, ",
        "Flags read, ",
        "Flags with verdicts, ",
        "Verdicts read, ",
        "Chose stickler, ",
        "Chose clean, ",
        "Abstained, ",
    ] {
        assert_eq!(
            count_after(judgements_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
    assert_eq!(out, render_judgements(&judgements_reading_of(&value)));
}

/// The outcomes relation of the constructed judgements
/// collection, the flag groups of each judgements
/// section being exactly the flags the outcomes
/// reading names under that judgement that carry at
/// least one verdict, in the same order, and each
/// section's closing count equal to the pairs standing
/// on those flags in the verdicts reading over the
/// same collection, all three readings run here.
#[test]
fn judgements_collection_sections_bind_to_the_outcomes_reading() {
    let value = parsed_example("judgements_collection.json");
    let reading = judgements_reading_of(&value);
    let outcomes = outcomes_reading_of(&value);
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
    for (judgements_section, outcomes_section) in
        reading.sections.iter().zip(outcomes.sections.iter())
    {
        assert_eq!(judgements_section.judgement, outcomes_section.judgement);
        let expected: Vec<_> = outcomes_section
            .flags
            .iter()
            .filter(|flag| {
                pair_count(&flag.set_label, &flag.file, flag.line, &flag.pattern).is_some()
            })
            .collect();
        assert_eq!(
            judgements_section.groups.len(),
            expected.len(),
            "section {}",
            judgements_section.judgement
        );
        for (group, flag) in judgements_section.groups.iter().zip(expected.iter()) {
            assert_eq!(
                group.file, flag.file,
                "section {}",
                judgements_section.judgement
            );
            assert_eq!(
                group.line, flag.line,
                "section {}",
                judgements_section.judgement
            );
            assert_eq!(
                group.set_label, flag.set_label,
                "section {}",
                judgements_section.judgement
            );
        }
        let stated: usize = judgements_section
            .groups
            .iter()
            .map(|group| group.verdicts.len())
            .sum();
        let counted: usize = expected
            .iter()
            .map(|flag| pair_count(&flag.set_label, &flag.file, flag.line, &flag.pattern).unwrap())
            .sum();
        assert_eq!(stated, counted, "section {}", judgements_section.judgement);
    }
}

/// The internal sums of the judgements reading over
/// the constructed judgements collection, the section
/// closing counts summing to the closing verdicts read
/// count and the three outcome counts summing to the
/// same, read from the rendered text.
#[test]
fn judgements_collection_internal_sums_hold() {
    let value = parsed_example("judgements_collection.json");
    let rendered = render_judgements(&judgements_reading_of(&value));
    let section_counts: Vec<usize> = rendered
        .lines()
        .filter(|line| line.starts_with("Verdicts read, "))
        .map(|line| {
            line.strip_prefix("Verdicts read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 4);
    assert_eq!(section_counts[..3].iter().sum::<usize>(), section_counts[3]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(section_counts[3], count_after(closings, "Verdicts read, "));
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// The first dimension pin of the constructed
/// standings collection, read off the fixture itself
/// under the parameters the preamble of the Round 19
/// section states. The fixture stands absent against
/// the tree Round 18 closed on, so the pin fails
/// there by the fixture's absence, and the pin can
/// fail against a miscounted construction on the
/// counts it reads.
#[test]
fn standings_collection_holds_two_sets_named_as_constructed() {
    let value = parsed_example("standings_collection.json");
    assert!(load_standings(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
    }
}

/// The second dimension pin of the constructed
/// standings collection, each set holding between
/// three and six files, read off the fixture itself.
#[test]
fn standings_collection_files_per_set_stand_between_three_and_six() {
    let value = parsed_example("standings_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
}

/// The third dimension pin of the constructed
/// standings collection, the flagged lines across
/// the collection standing between sixteen and twenty
/// eight, counted from the census over the collection.
#[test]
fn standings_collection_flagged_lines_stand_between_sixteen_and_twenty_eight() {
    let value = parsed_example("standings_collection.json");
    let census = census_of(&value);
    assert!(
        (16..=28).contains(&census.total.borderline),
        "the collection holds {} flagged lines",
        census.total.borderline
    );
}

/// The fourth dimension pin of the constructed
/// standings collection, the verdicts across the
/// collection standing between twenty four and forty
/// four, counted from the verdicts reading over the
/// collection.
#[test]
fn standings_collection_verdicts_stand_between_twenty_four_and_forty_four() {
    let value = parsed_example("standings_collection.json");
    let reading = verdicts_reading_of(&value);
    let verdicts: usize = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .map(|group| group.verdicts.len())
        .sum();
    assert!(
        (24..=44).contains(&verdicts),
        "the collection holds {verdicts} verdicts"
    );
}

/// The first shape requirement of the constructed
/// standings collection, at least two files standing
/// in both sets, each carrying recorded verdicts in
/// both of its standings, counted from the standings
/// reading, the sections whose flag groups carry each
/// set's label standing at least at two.
#[test]
fn standings_collection_two_files_stand_in_both_sets_and_carry_verdicts_in_each_standing() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
    let both = reading
        .sections
        .iter()
        .filter(|section| {
            ["constructed standings set 1", "constructed standings set 2"]
                .iter()
                .all(|label| section.groups.iter().any(|group| group.set_label == *label))
        })
        .count();
    assert!(
        both >= 2,
        "{both} files stand in both sets carrying verdicts in each standing"
    );
}

/// The second shape requirement, every file section's
/// total of pairs standing at least at four, counted
/// from the standings reading.
#[test]
fn standings_collection_every_file_sections_pairs_stand_at_least_at_four() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
    for section in &reading.sections {
        let pairs: usize = section
            .groups
            .iter()
            .map(|group| group.verdicts.len())
            .sum();
        assert!(
            pairs >= 4,
            "the file {} carries {pairs} pairs in total",
            section.file
        );
    }
}

/// The third shape requirement, at least one flag of
/// each sectioned file carrying two verdicts, counted
/// from the standings reading, each section holding a
/// group of two or more pairs.
#[test]
fn standings_collection_a_flag_of_each_sectioned_file_carries_two_verdicts() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
    for section in &reading.sections {
        assert!(
            section.groups.iter().any(|group| group.verdicts.len() >= 2),
            "no flag of the file {} carries two verdicts",
            section.file
        );
    }
}

/// The fourth shape requirement, at least one flag
/// carrying verdicts of both labels, counted from the
/// standings reading, one group holding a verdict
/// whose outcome stands as stickler and one whose
/// outcome stands as clean under the one outcome
/// helper.
#[test]
fn standings_collection_a_flag_carries_verdicts_of_both_labels() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            group
                .verdicts
                .iter()
                .any(|verdict| verdict_outcome(verdict) == "stickler")
                && group
                    .verdicts
                    .iter()
                    .any(|verdict| verdict_outcome(verdict) == "clean")
        });
    assert!(stands, "one flag carries verdicts of both labels");
}

/// The fifth shape requirement, at least one flag
/// carrying only abstentions, counted from the
/// standings reading, one group standing whose every
/// verdict's outcome stands as abstained under the
/// one outcome helper.
#[test]
fn standings_collection_a_flag_carries_only_abstentions() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .any(|group| {
            !group.verdicts.is_empty()
                && group
                    .verdicts
                    .iter()
                    .all(|verdict| verdict_outcome(verdict) == "abstained")
        });
    assert!(stands, "one flag carries only abstentions");
}

/// The sixth shape requirement, at least one flag
/// carrying no verdicts in a file standing a section,
/// counted against the files reading over the
/// collection, a flag that reading names under a
/// sectioned file standing in no standings group.
#[test]
fn standings_collection_a_flag_carries_no_verdicts_in_a_file_standing_a_section() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
    let files = files_reading_of(&value);
    let stands = files
        .sections
        .iter()
        .filter(|files_section| {
            reading
                .sections
                .iter()
                .any(|section| section.file == files_section.file)
        })
        .any(|files_section| {
            files_section.flags.iter().any(|flag| {
                !reading
                    .sections
                    .iter()
                    .flat_map(|section| section.groups.iter())
                    .any(|group| {
                        group.file == files_section.file
                            && group.line == flag.line
                            && group.set_label == flag.set_label
                    })
            })
        });
    assert!(
        stands,
        "one flag carries no verdicts in a file standing a section"
    );
}

/// The verdicts relation of the constructed standings
/// collection, each of the seven closing counts of the
/// standings reading standing count for count against
/// the same named count of the verdicts reading over
/// the same collection, both readings run here, and
/// the command output standing byte identical to the
/// library rendering.
#[test]
fn standings_collection_closings_bind_to_the_verdicts_reading() {
    let (code, out) = run_on_example("standings", "standings_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("standings_collection.json");
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let standings_closings = out.split("\n\n").last().unwrap();
    let verdicts_closings = verdicts.split("\n\n").last().unwrap();
    for prefix in [
        "Sets read, ",
        "Flags read, ",
        "Flags with verdicts, ",
        "Verdicts read, ",
        "Chose stickler, ",
        "Chose clean, ",
        "Abstained, ",
    ] {
        assert_eq!(
            count_after(standings_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
    assert_eq!(out, render_standings(&standings_reading_of(&value)));
}

/// The files relation of the constructed standings
/// collection, the flag groups of each standings
/// section being exactly the flags the files reading
/// names under that file that carry at least one
/// verdict, in the same order, and each section's
/// closing count equal to the pairs standing on those
/// flags in the verdicts reading over the same
/// collection, all three readings run here.
#[test]
fn standings_collection_sections_bind_to_the_files_reading() {
    let value = parsed_example("standings_collection.json");
    let reading = standings_reading_of(&value);
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
    let carrying: Vec<_> = files
        .sections
        .iter()
        .filter(|files_section| {
            files_section.flags.iter().any(|flag| {
                pair_count(
                    &flag.set_label,
                    &files_section.file,
                    flag.line,
                    &flag.pattern,
                )
                .is_some()
            })
        })
        .collect();
    assert_eq!(reading.sections.len(), carrying.len());
    for (standings_section, files_section) in reading.sections.iter().zip(carrying.iter()) {
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

/// The internal sums of the standings reading over
/// the constructed standings collection, the section
/// closing counts summing to the closing verdicts read
/// count and the three outcome counts summing to the
/// same, read from the rendered text.
#[test]
fn standings_collection_internal_sums_hold() {
    let value = parsed_example("standings_collection.json");
    let rendered = render_standings(&standings_reading_of(&value));
    let section_counts: Vec<usize> = rendered
        .lines()
        .filter(|line| line.starts_with("Verdicts read, "))
        .map(|line| {
            line.strip_prefix("Verdicts read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect();
    assert_eq!(section_counts.len(), 7);
    assert_eq!(section_counts[..6].iter().sum::<usize>(), section_counts[6]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(section_counts[6], count_after(closings, "Verdicts read, "));
    assert_eq!(
        count_after(closings, "Chose stickler, ")
            + count_after(closings, "Chose clean, ")
            + count_after(closings, "Abstained, "),
        count_after(closings, "Verdicts read, ")
    );
}

/// The first dimension pin of the constructed states
/// collection, read off the fixture itself under the
/// parameters the preamble of the Round 20 section
/// states. The fixture stands absent against the tree
/// Round 19 closed on, so the pin fails there by the
/// fixture's absence, and the pin can fail against a
/// miscounted construction on the counts it reads.
#[test]
fn states_collection_holds_two_sets_named_as_constructed() {
    let value = parsed_example("states_collection.json");
    assert!(load_states(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
    }
}

/// The second dimension pin of the constructed states
/// collection, each set holding between three and six
/// files, read off the fixture itself.
#[test]
fn states_collection_files_per_set_stand_between_three_and_six() {
    let value = parsed_example("states_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
}

/// The third dimension pin of the constructed states
/// collection, the flagged lines across the collection
/// standing between sixteen and twenty eight, counted
/// from the census over the collection.
#[test]
fn states_collection_flagged_lines_stand_between_sixteen_and_twenty_eight() {
    let value = parsed_example("states_collection.json");
    let census = census_of(&value);
    assert!(
        (16..=28).contains(&census.total.borderline),
        "the collection holds {} flagged lines",
        census.total.borderline
    );
}

/// The fourth dimension pin of the constructed states
/// collection, the verdicts across the collection
/// standing between twenty four and forty four, counted
/// from the verdicts reading over the collection.
#[test]
fn states_collection_verdicts_stand_between_twenty_four_and_forty_four() {
    let value = parsed_example("states_collection.json");
    let reading = verdicts_reading_of(&value);
    let verdicts: usize = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .map(|group| group.verdicts.len())
        .sum();
    assert!(
        (24..=44).contains(&verdicts),
        "the collection holds {verdicts} verdicts"
    );
}

/// The first shape requirement of the constructed
/// states collection, at least two files standing in
/// both sets, each carrying recorded verdicts in both
/// of its standings, counted from the states reading,
/// the file groups whose flag groups carry each set's
/// label standing at least at two.
#[test]
fn states_collection_two_files_stand_in_both_sets_and_carry_verdicts_in_each_standing() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let both = reading
        .sections
        .iter()
        .flat_map(|section| section.files.iter())
        .filter(|file_group| {
            ["constructed states set 1", "constructed states set 2"]
                .iter()
                .all(|label| {
                    file_group
                        .groups
                        .iter()
                        .any(|group| group.set_label == *label)
                })
        })
        .count();
    assert!(
        both >= 2,
        "{both} files stand in both sets carrying verdicts in each standing"
    );
}

/// The second shape requirement, at least one file
/// standing judged clean over the whole collection and
/// at least one standing unjudged while carrying
/// verdicts, so all three sections stand populated,
/// counted from the states reading.
#[test]
fn states_collection_a_file_stands_judged_clean_and_a_file_stands_unjudged_over_the_whole_collection(
) {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    assert!(
        !reading.sections[0].files.is_empty(),
        "one file stands judged stickler over the whole collection"
    );
    assert!(
        !reading.sections[1].files.is_empty(),
        "one file stands judged clean over the whole collection"
    );
    assert!(
        !reading.sections[2].files.is_empty(),
        "one file stands unjudged while carrying verdicts"
    );
}

/// The third shape requirement, a file standing in
/// both sets reading unjudged on the flags of its
/// first set alone and judged clean over the pool of
/// the whole collection, the first set's pool read
/// from the verdicts reading over the collection and
/// the pooled state from the states reading.
#[test]
fn states_collection_a_file_in_both_sets_reads_unjudged_on_its_first_set_alone_and_judged_clean_pooled(
) {
    let value = parsed_example("states_collection.json");
    let verdicts = verdicts_reading_of(&value);
    let reading = states_reading_of(&value);
    let stands = reading.sections[1].files.iter().any(|file_group| {
        let first_pool: Vec<_> = verdicts.sections[0]
            .groups
            .iter()
            .filter(|group| group.file == file_group.file)
            .flat_map(|group| group.verdicts.iter().cloned())
            .collect();
        !first_pool.is_empty()
            && line_outcome(&first_pool) == "unjudged"
            && file_group
                .groups
                .iter()
                .any(|group| group.set_label == "constructed states set 2")
    });
    assert!(
        stands,
        "one file reads unjudged on its first set alone and judged clean pooled"
    );
}

/// The fourth shape requirement, a file standing in
/// both sets reading judged clean on the flags of its
/// first set alone and judged stickler over the pool
/// of the whole collection, read as the third
/// requirement reads its file.
#[test]
fn states_collection_a_file_in_both_sets_reads_judged_clean_on_its_first_set_alone_and_judged_stickler_pooled(
) {
    let value = parsed_example("states_collection.json");
    let verdicts = verdicts_reading_of(&value);
    let reading = states_reading_of(&value);
    let stands = reading.sections[0].files.iter().any(|file_group| {
        let first_pool: Vec<_> = verdicts.sections[0]
            .groups
            .iter()
            .filter(|group| group.file == file_group.file)
            .flat_map(|group| group.verdicts.iter().cloned())
            .collect();
        !first_pool.is_empty()
            && line_outcome(&first_pool) == "clean"
            && file_group
                .groups
                .iter()
                .any(|group| group.set_label == "constructed states set 2")
    });
    assert!(
        stands,
        "one file reads judged clean on its first set alone and judged stickler pooled"
    );
}

/// The fifth shape requirement, every file standing a
/// section carrying at least four pairs in total,
/// counted from the states reading.
#[test]
fn states_collection_every_sectioned_files_pairs_stand_at_least_at_four() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    for section in &reading.sections {
        for file_group in &section.files {
            let pairs: usize = file_group
                .groups
                .iter()
                .map(|group| group.verdicts.len())
                .sum();
            assert!(
                pairs >= 4,
                "the file {} carries {pairs} pairs in total",
                file_group.file
            );
        }
    }
}

/// The sixth shape requirement, at least one flag of
/// each sectioned file carrying two verdicts, counted
/// from the states reading, each file group holding a
/// group of two or more pairs.
#[test]
fn states_collection_a_flag_of_each_sectioned_file_carries_two_verdicts() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    for section in &reading.sections {
        for file_group in &section.files {
            assert!(
                file_group
                    .groups
                    .iter()
                    .any(|group| group.verdicts.len() >= 2),
                "no flag of the file {} carries two verdicts",
                file_group.file
            );
        }
    }
}

/// The seventh shape requirement, at least one flag
/// carrying verdicts of both labels inside a sectioned
/// file, counted from the states reading, one group
/// holding a verdict whose outcome stands as stickler
/// and one whose outcome stands as clean under the one
/// outcome helper.
#[test]
fn states_collection_a_flag_carries_verdicts_of_both_labels() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.files.iter())
        .flat_map(|file_group| file_group.groups.iter())
        .any(|group| {
            group
                .verdicts
                .iter()
                .any(|verdict| verdict_outcome(verdict) == "stickler")
                && group
                    .verdicts
                    .iter()
                    .any(|verdict| verdict_outcome(verdict) == "clean")
        });
    assert!(stands, "one flag carries verdicts of both labels");
}

/// The eighth shape requirement, at least one flag
/// carrying only abstentions inside a sectioned file,
/// counted from the states reading, one group standing
/// whose every verdict's outcome stands as abstained
/// under the one outcome helper.
#[test]
fn states_collection_a_flag_carries_only_abstentions() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.files.iter())
        .flat_map(|file_group| file_group.groups.iter())
        .any(|group| {
            !group.verdicts.is_empty()
                && group
                    .verdicts
                    .iter()
                    .all(|verdict| verdict_outcome(verdict) == "abstained")
        });
    assert!(stands, "one flag carries only abstentions");
}

/// The ninth shape requirement, restated in its
/// loadable form by FIX-CK-30. As first stated it
/// stood as one flag carrying only handed word
/// verdicts, the word abstained standing as a choice
/// at a confidence that is no abstention, and no
/// loader admitted file can carry such a flag, the
/// shared validation naming a choice outside the two
/// labels a defect in every collection it loads. The
/// requirement stands as one flag carrying only
/// abstentions by confidence under CK-7, label
/// choices standing below one half, inside a file
/// standing unjudged on those verdicts, counted from
/// the states reading, one group of the unjudged
/// section standing whose every verdict is an
/// abstention under the predicate of CK-7.
#[test]
fn states_collection_a_flag_carries_only_abstentions_in_a_file_standing_unjudged() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let stands = reading.sections[2].files.iter().any(|file_group| {
        file_group
            .groups
            .iter()
            .any(|group| !group.verdicts.is_empty() && group.verdicts.iter().all(is_abstention))
    });
    assert!(
        stands,
        "one flag carries only abstentions in a file standing unjudged"
    );
}

/// The tenth shape requirement, at least one flag
/// carrying no verdicts in a file standing a section,
/// counted against the files reading over the
/// collection, a flag that reading names under a
/// sectioned file standing in no states group.
#[test]
fn states_collection_a_flag_carries_no_verdicts_in_a_file_standing_a_section() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let files = files_reading_of(&value);
    let stands = files
        .sections
        .iter()
        .filter(|files_section| {
            reading
                .sections
                .iter()
                .flat_map(|section| section.files.iter())
                .any(|file_group| file_group.file == files_section.file)
        })
        .any(|files_section| {
            files_section.flags.iter().any(|flag| {
                !reading
                    .sections
                    .iter()
                    .flat_map(|section| section.files.iter())
                    .flat_map(|file_group| file_group.groups.iter())
                    .any(|group| {
                        group.file == files_section.file
                            && group.line == flag.line
                            && group.set_label == flag.set_label
                    })
            })
        });
    assert!(
        stands,
        "one flag carries no verdicts in a file standing a section"
    );
}

/// The verdicts relation of the constructed states
/// collection, each of the seven closing counts of the
/// states reading standing count for count against the
/// same named count of the verdicts reading over the
/// same collection, both readings run here, and the
/// command output standing byte identical to the
/// library rendering.
#[test]
fn states_collection_closings_bind_to_the_verdicts_reading() {
    let (code, out) = run_on_example("states", "states_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("states_collection.json");
    let verdicts = render_verdicts(&verdicts_reading_of(&value));
    let states_closings = out.split("\n\n").last().unwrap();
    let verdicts_closings = verdicts.split("\n\n").last().unwrap();
    for prefix in [
        "Sets read, ",
        "Flags read, ",
        "Flags with verdicts, ",
        "Verdicts read, ",
        "Chose stickler, ",
        "Chose clean, ",
        "Abstained, ",
    ] {
        assert_eq!(
            count_after(states_closings, prefix),
            count_after(verdicts_closings, prefix),
            "{prefix} binds to the verdicts reading"
        );
    }
    assert_eq!(out, render_states(&states_reading_of(&value)));
}

/// The standings relation of the constructed states
/// collection, the flag groups of each file group of
/// the states reading being exactly the flag groups of
/// that file's section in the standings reading over
/// the same collection, in the same order, with the
/// same pairs, both readings run here.
#[test]
fn states_collection_file_groups_bind_to_the_standings_reading() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let standings = standings_reading_of(&value);
    let file_groups: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.files.iter())
        .collect();
    assert_eq!(file_groups.len(), standings.sections.len());
    for section in &standings.sections {
        let file_group = file_groups
            .iter()
            .find(|file_group| file_group.file == section.file)
            .unwrap();
        assert_eq!(
            file_group.groups.len(),
            section.groups.len(),
            "file {}",
            section.file
        );
        for (states_group, standings_group) in file_group.groups.iter().zip(section.groups.iter()) {
            assert_eq!(states_group.file, standings_group.file);
            assert_eq!(states_group.line, standings_group.line);
            assert_eq!(states_group.pattern, standings_group.pattern);
            assert_eq!(states_group.set_label, standings_group.set_label);
            assert_eq!(states_group.verdicts.len(), standings_group.verdicts.len());
        }
    }
}

/// The judgements relation of the constructed states
/// collection, each file carrying verdicts standing in
/// the states section of the precedence over the
/// judgements of its flags exactly as the judgements
/// reading over the same collection states those
/// judgements, and the states sections holding exactly
/// the files the judgements reading names, both
/// readings run here. It catches the majority
/// derivation over the constructed collection.
#[test]
fn states_collection_states_bind_to_the_judgements_reading() {
    let value = parsed_example("states_collection.json");
    let reading = states_reading_of(&value);
    let judgements = judgements_reading_of(&value);
    let mut files: Vec<String> = Vec::new();
    for section in &judgements.sections {
        for group in &section.groups {
            if !files.contains(&group.file) {
                files.push(group.file.clone());
            }
        }
    }
    let stated: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.files.iter())
        .collect();
    assert_eq!(stated.len(), files.len());
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
        let standing = reading
            .sections
            .iter()
            .find(|section| section.files.iter().any(|group| &group.file == file))
            .unwrap();
        assert_eq!(standing.state, expected, "file {file}");
    }
}

/// The internal sums of the states reading over the
/// constructed states collection, each section's file
/// group closing counts summing to its section closing
/// count and the three section closing counts summing
/// to the closing verdicts read count, read from the
/// rendered text.
#[test]
fn states_collection_internal_sums_hold() {
    let value = parsed_example("states_collection.json");
    let rendered = render_states(&states_reading_of(&value));
    let blocks: Vec<&str> = rendered.split("\n\n").collect();
    assert_eq!(blocks.len(), 5);
    let mut section_sum = 0usize;
    for block in &blocks[1..4] {
        let counts: Vec<usize> = block
            .lines()
            .filter(|line| line.starts_with("Verdicts read, "))
            .map(|line| {
                line.strip_prefix("Verdicts read, ")
                    .unwrap()
                    .parse()
                    .unwrap()
            })
            .collect();
        let section_closing = *counts.last().unwrap();
        assert_eq!(
            counts[..counts.len() - 1].iter().sum::<usize>(),
            section_closing,
            "section block {block}"
        );
        section_sum += section_closing;
    }
    let closings = blocks[4];
    assert_eq!(section_sum, count_after(closings, "Verdicts read, "));
}

/// The first dimension pin of the constructed flags
/// collection, read off the fixture itself under the
/// parameters the preamble of the Round 21 section
/// states. The fixture stands absent against the tree
/// Round 20 closed on, so the pin fails there by the
/// fixture's absence, and the pin can fail against a
/// miscounted construction on the counts it reads.
#[test]
fn flags_collection_holds_two_sets_named_as_constructed() {
    let value = parsed_example("flags_collection.json");
    assert!(load_flags(&value).is_empty());
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    assert_eq!(sets.len(), 2);
    for set in sets {
        let label = set
            .get("label")
            .and_then(|label| label.as_jstring())
            .unwrap()
            .to_rust_string()
            .unwrap();
        assert!(
            label.contains("constructed"),
            "the set {label} is named as constructed"
        );
    }
}

/// The second dimension pin of the constructed flags
/// collection, each set holding between three and six
/// files, read off the fixture itself.
#[test]
fn flags_collection_files_per_set_stand_between_three_and_six() {
    let value = parsed_example("flags_collection.json");
    let sets = value.get("sets").and_then(|sets| sets.as_array()).unwrap();
    for set in sets {
        let files = set
            .get("change_set")
            .unwrap()
            .get("files")
            .and_then(|files| files.as_array())
            .unwrap();
        assert!(
            (3..=6).contains(&files.len()),
            "a set holds {} files",
            files.len()
        );
    }
}

/// The third dimension pin of the constructed flags
/// collection, the flagged lines across the collection
/// standing between sixteen and twenty eight, counted
/// from the census over the collection.
#[test]
fn flags_collection_flagged_lines_stand_between_sixteen_and_twenty_eight() {
    let value = parsed_example("flags_collection.json");
    let census = census_of(&value);
    assert!(
        (16..=28).contains(&census.total.borderline),
        "the collection holds {} flagged lines",
        census.total.borderline
    );
}

/// The fourth dimension pin of the constructed flags
/// collection, the verdicts across the collection
/// standing between twenty four and forty four, counted
/// from the verdicts reading over the collection.
#[test]
fn flags_collection_verdicts_stand_between_twenty_four_and_forty_four() {
    let value = parsed_example("flags_collection.json");
    let reading = verdicts_reading_of(&value);
    let verdicts: usize = reading
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .map(|group| group.verdicts.len())
        .sum();
    assert!(
        (24..=44).contains(&verdicts),
        "the collection holds {verdicts} verdicts"
    );
}

/// The first shape requirement of the constructed
/// flags collection, at least two files standing in
/// both sets, each carrying recorded verdicts in both
/// of its standings, counted from the flags reading
/// for the standings and from the verdicts reading
/// for the verdicts each standing carries.
#[test]
fn flags_collection_two_files_stand_in_both_sets_and_carry_verdicts_in_each_standing() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let files: Vec<&str> = reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .map(|line| line.file.as_str())
        .collect();
    let mut both = 0;
    for file in [
        "anchor.py",
        "beacon.py",
        "compass.py",
        "dune.py",
        "ember.py",
        "fjord.py",
    ] {
        if !files.contains(&file) {
            continue;
        }
        let in_both = ["constructed flags set 1", "constructed flags set 2"]
            .iter()
            .all(|label| {
                reading
                    .sections
                    .iter()
                    .flat_map(|section| section.lines.iter())
                    .any(|line| line.file == file && line.set_label == *label)
            });
        let verdicts_in_both = [0usize, 1usize].iter().all(|index| {
            verdicts.sections[*index]
                .groups
                .iter()
                .any(|group| group.file == file)
        });
        if in_both && verdicts_in_both {
            both += 1;
        }
    }
    assert!(
        both >= 2,
        "{both} files stand in both sets carrying verdicts in each standing"
    );
}

/// The second shape requirement, at least one file
/// standing judged clean over the whole collection and
/// at least one standing unjudged while carrying
/// verdicts, so all three sections stand populated,
/// counted from the flags reading.
#[test]
fn flags_collection_a_file_stands_judged_clean_and_a_file_stands_unjudged_over_the_whole_collection(
) {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    assert!(
        !reading.sections[0].lines.is_empty(),
        "one file stands judged stickler over the whole collection"
    );
    assert!(
        !reading.sections[1].lines.is_empty(),
        "one file stands judged clean over the whole collection"
    );
    assert!(
        !reading.sections[2].lines.is_empty(),
        "one file stands unjudged while carrying verdicts"
    );
}

/// The third shape requirement, a file standing in
/// both sets reading unjudged on the flags of its
/// first set alone and judged clean over the pool of
/// the whole collection, the first set's pool read
/// from the verdicts reading over the collection and
/// the pooled seating from the flags reading.
#[test]
fn flags_collection_a_file_in_both_sets_reads_unjudged_on_its_first_set_alone_and_judged_clean_pooled(
) {
    let value = parsed_example("flags_collection.json");
    let verdicts = verdicts_reading_of(&value);
    let reading = flags_reading_of(&value);
    let stands = reading.sections[1].lines.iter().any(|line| {
        let first_pool: Vec<_> = verdicts.sections[0]
            .groups
            .iter()
            .filter(|group| group.file == line.file)
            .flat_map(|group| group.verdicts.iter().cloned())
            .collect();
        !first_pool.is_empty()
            && line_outcome(&first_pool) == "unjudged"
            && reading
                .sections
                .iter()
                .flat_map(|section| section.lines.iter())
                .any(|held| held.file == line.file && held.set_label == "constructed flags set 2")
    });
    assert!(
        stands,
        "one file reads unjudged on its first set alone and judged clean pooled"
    );
}

/// The fourth shape requirement, a file standing in
/// both sets reading judged clean on the flags of its
/// first set alone and judged stickler over the pool
/// of the whole collection, read as the third
/// requirement reads its file.
#[test]
fn flags_collection_a_file_in_both_sets_reads_judged_clean_on_its_first_set_alone_and_judged_stickler_pooled(
) {
    let value = parsed_example("flags_collection.json");
    let verdicts = verdicts_reading_of(&value);
    let reading = flags_reading_of(&value);
    let stands = reading.sections[0].lines.iter().any(|line| {
        let first_pool: Vec<_> = verdicts.sections[0]
            .groups
            .iter()
            .filter(|group| group.file == line.file)
            .flat_map(|group| group.verdicts.iter().cloned())
            .collect();
        !first_pool.is_empty()
            && line_outcome(&first_pool) == "clean"
            && reading
                .sections
                .iter()
                .flat_map(|section| section.lines.iter())
                .any(|held| held.file == line.file && held.set_label == "constructed flags set 2")
    });
    assert!(
        stands,
        "one file reads judged clean on its first set alone and judged stickler pooled"
    );
}

/// The fifth shape requirement, every file standing a
/// section carrying at least four flag lines in total,
/// counted from the flags reading.
#[test]
fn flags_collection_every_sectioned_file_carries_at_least_four_flag_lines() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let mut files: Vec<&str> = Vec::new();
    for section in &reading.sections {
        for line in &section.lines {
            if !files.contains(&line.file.as_str()) {
                files.push(line.file.as_str());
            }
        }
    }
    assert!(!files.is_empty());
    for file in files {
        let lines = reading
            .sections
            .iter()
            .flat_map(|section| section.lines.iter())
            .filter(|line| line.file == file)
            .count();
        assert!(
            lines >= 4,
            "the file {file} carries {lines} flag lines in total"
        );
    }
}

/// The sixth shape requirement, a flag of every
/// sectioned file carrying two recorded verdicts,
/// counted from the verdicts reading against the
/// sectioned files of the flags reading.
#[test]
fn flags_collection_a_flag_of_each_sectioned_file_carries_two_verdicts() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let mut files: Vec<&str> = Vec::new();
    for section in &reading.sections {
        for line in &section.lines {
            if !files.contains(&line.file.as_str()) {
                files.push(line.file.as_str());
            }
        }
    }
    assert!(!files.is_empty());
    for file in files {
        assert!(
            verdicts
                .sections
                .iter()
                .flat_map(|section| section.groups.iter())
                .any(|group| group.file == file && group.verdicts.len() >= 2),
            "no flag of the file {file} carries two verdicts"
        );
    }
}

/// The seventh shape requirement, at least one flag
/// carrying no verdicts in a file standing a section,
/// counted against the verdicts reading over the
/// collection, a flag standing as a line in the flags
/// reading standing in no verdicts group.
#[test]
fn flags_collection_a_flag_carries_no_verdicts_in_a_file_standing_a_section() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let stands = reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .any(|line| {
            !verdicts
                .sections
                .iter()
                .filter(|section| section.label == line.set_label)
                .flat_map(|section| section.groups.iter())
                .any(|group| group.file == line.file && group.line == line.line)
        });
    assert!(
        stands,
        "one flag carries no verdicts in a file standing a section"
    );
}

/// The eighth shape requirement's companion, at least
/// one file carrying flags and no recorded verdicts
/// anywhere, standing in no section while the closing
/// flags read count counts its flags, counted from
/// the files reading against the flags reading.
#[test]
fn flags_collection_a_file_carries_flags_and_no_verdicts_and_stands_in_no_section() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let sectioned: Vec<&str> = reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .map(|line| line.file.as_str())
        .collect();
    let stands = files.sections.iter().any(|files_section| {
        !files_section.flags.is_empty()
            && !sectioned.contains(&files_section.file.as_str())
            && !verdicts
                .sections
                .iter()
                .flat_map(|section| section.groups.iter())
                .any(|group| group.file == files_section.file)
    });
    assert!(
        stands,
        "one file carries flags and no verdicts and stands in no section"
    );
    let section_lines: usize = reading
        .sections
        .iter()
        .map(|section| section.lines.len())
        .sum();
    assert!(
        reading.flags_read > section_lines,
        "the closing flags read count counts the unsectioned file's flags"
    );
}

/// The eighth shape requirement, at least two flags
/// standing contested, counted from the flags reading,
/// each contested flag standing in the judged stickler
/// section by the consequence the preamble records.
#[test]
fn flags_collection_two_flags_stand_contested() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let contested: Vec<_> = reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .filter(|line| line.contested)
        .collect();
    assert!(
        contested.len() >= 2,
        "{} flags stand contested",
        contested.len()
    );
    assert!(
        reading.sections[0]
            .lines
            .iter()
            .filter(|line| line.contested)
            .count()
            == contested.len(),
        "every contested flag stands in the judged stickler section"
    );
}

/// The eighth shape requirement's second half, at
/// least one flag carrying recorded verdicts of both
/// labels with an abstaining side and standing not
/// contested in a file standing a section, counted
/// from the verdicts reading against the flags
/// reading.
#[test]
fn flags_collection_a_flag_carries_both_labels_with_an_abstaining_side_and_stands_not_contested() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let verdicts = verdicts_reading_of(&value);
    let sectioned: Vec<&str> = reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .map(|line| line.file.as_str())
        .collect();
    let stands = verdicts
        .sections
        .iter()
        .flat_map(|section| section.groups.iter())
        .filter(|group| sectioned.contains(&group.file.as_str()))
        .any(|group| {
            group
                .verdicts
                .iter()
                .any(|verdict| verdict.choice == "stickler")
                && group
                    .verdicts
                    .iter()
                    .any(|verdict| verdict.choice == "clean")
                && group.verdicts.iter().any(is_abstention)
                && !is_contested(&group.verdicts)
        });
    assert!(
        stands,
        "one flag carries both labels with an abstaining side and stands not contested"
    );
}

/// The census relation of the constructed flags
/// collection, each of the closing counts of the flags
/// reading standing against the same named count of
/// the census over the same collection, the sets read
/// count the count of the sets the census reads and
/// the flags read count its total block's flagged
/// lines, and the command output standing byte
/// identical to the library rendering.
#[test]
fn flags_collection_closings_bind_to_the_census() {
    let (code, out) = run_on_example("flags", "flags_collection.json");
    assert_eq!(code, 0);
    let value = parsed_example("flags_collection.json");
    let census = census_of(&value);
    let closings = out.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
    assert_eq!(out, render_flags(&flags_reading_of(&value)));
}

/// The files relation of the constructed flags
/// collection, the flag lines of each section being
/// exactly the flags the files reading names under
/// the files standing in that state, in the same
/// collection order within each file, both readings
/// run here. It catches the per set seating mutation
/// over the constructed collection, a file's lines
/// splitting across sections by gathering.
#[test]
fn flags_collection_flag_lines_bind_to_the_files_reading() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let states = states_reading_of(&value);
    for (flags_section, states_section) in reading.sections.iter().zip(states.sections.iter()) {
        let stated_files: Vec<&str> = states_section
            .files
            .iter()
            .map(|group| group.file.as_str())
            .collect();
        let mut expected: Vec<(String, usize, String, String)> = Vec::new();
        for files_section in &files.sections {
            if stated_files.contains(&files_section.file.as_str()) {
                for flag in &files_section.flags {
                    expected.push((
                        files_section.file.clone(),
                        flag.line,
                        flag.pattern.clone(),
                        flag.set_label.clone(),
                    ));
                }
            }
        }
        let mut seated: Vec<(String, usize, String, String)> = flags_section
            .lines
            .iter()
            .map(|line| {
                (
                    line.file.clone(),
                    line.line,
                    line.pattern.clone(),
                    line.set_label.clone(),
                )
            })
            .collect();
        expected.sort();
        seated.sort();
        assert_eq!(seated, expected, "state {}", flags_section.state);
        for files_section in &files.sections {
            if stated_files.contains(&files_section.file.as_str()) {
                let held: Vec<_> = flags_section
                    .lines
                    .iter()
                    .filter(|line| line.file == files_section.file)
                    .collect();
                assert_eq!(
                    held.len(),
                    files_section.flags.len(),
                    "file {}",
                    files_section.file
                );
                for (line, flag) in held.iter().zip(files_section.flags.iter()) {
                    assert_eq!(line.line, flag.line);
                    assert_eq!(line.pattern, flag.pattern);
                    assert_eq!(line.set_label, flag.set_label);
                    assert_eq!(line.contested, flag.contested);
                }
            }
        }
    }
}

/// The states relation of the constructed flags
/// collection, each section standing exactly the files
/// the states reading seats in its state and its line
/// count standing as the sum over those files of the
/// section flags read counts of the files reading,
/// all three readings run here. It catches the first
/// set pooling mutation and the seating guard mutation
/// over the constructed collection, the guard through
/// the file carrying flags and no verdicts that the
/// states reading seats nowhere.
#[test]
fn flags_collection_sections_bind_to_the_states_reading() {
    let value = parsed_example("flags_collection.json");
    let reading = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let states = states_reading_of(&value);
    for (flags_section, states_section) in reading.sections.iter().zip(states.sections.iter()) {
        assert_eq!(flags_section.state, states_section.state);
        let mut seated: Vec<&str> = Vec::new();
        for line in &flags_section.lines {
            if !seated.contains(&line.file.as_str()) {
                seated.push(line.file.as_str());
            }
        }
        let mut stated: Vec<&str> = states_section
            .files
            .iter()
            .map(|group| group.file.as_str())
            .collect();
        seated.sort();
        stated.sort();
        assert_eq!(seated, stated, "state {}", flags_section.state);
        let expected_lines: usize = stated
            .iter()
            .map(|file| {
                files
                    .sections
                    .iter()
                    .find(|section| &section.file == file)
                    .map(|section| section.flags.len())
                    .unwrap_or(0)
            })
            .sum();
        assert_eq!(flags_section.lines.len(), expected_lines);
    }
}

/// The internal sums of the flags reading over the
/// constructed flags collection, the section contested
/// counts summing to the closing contested flags count
/// and the section flags read counts summing to the
/// closing flags read count less the flags of the
/// files standing in no section, read from the
/// rendered text against the files reading.
#[test]
fn flags_collection_internal_sums_hold() {
    let value = parsed_example("flags_collection.json");
    let rendered = render_flags(&flags_reading_of(&value));
    let blocks: Vec<&str> = rendered.split("\n\n").collect();
    assert_eq!(blocks.len(), 5);
    let mut flags_sum = 0usize;
    let mut contested_sum = 0usize;
    for block in &blocks[1..4] {
        flags_sum += count_after(block, "Flags read, ");
        contested_sum += count_after(block, "Contested, ");
    }
    let closings = blocks[4];
    assert_eq!(contested_sum, count_after(closings, "Contested flags, "));
    let reading = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let sectioned: Vec<&str> = reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .map(|line| line.file.as_str())
        .collect();
    let unsectioned_flags: usize = files
        .sections
        .iter()
        .filter(|section| !sectioned.contains(&section.file.as_str()))
        .map(|section| section.flags.len())
        .sum();
    assert_eq!(
        flags_sum + unsectioned_flags,
        count_after(closings, "Flags read, ")
    );
}
