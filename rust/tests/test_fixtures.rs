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
//! Every pin runs offline on the constructed fixtures, and no
//! pin makes a live call.

use std::path::PathBuf;

use stickler::borderline::{borderline_reading_of, load_borderline};
use stickler::census::{census_of, load_census};
use stickler::changeset::load_changeset;
use stickler::checker::check_diff_detailed;
use stickler::cli::cli_main;
use stickler::json::{parse, JsonValue};
use stickler::render::render_borderline;

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
