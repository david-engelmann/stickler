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
//! Every pin runs offline on the constructed fixtures, and no
//! pin makes a live call.

use std::path::PathBuf;

use stickler::borderline::{borderline_reading_of, load_borderline};
use stickler::census::{census_of, load_census};
use stickler::changeset::load_changeset;
use stickler::checker::check_diff_detailed;
use stickler::cli::cli_main;
use stickler::json::{parse, JsonValue};
use stickler::outcomes::outcomes_reading_of;
use stickler::render::{render_borderline, render_shapes, render_verdicts};
use stickler::shapes::{load_shapes, shapes_reading_of};
use stickler::verdicts::{is_contested, line_outcome, verdict_outcome};
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
