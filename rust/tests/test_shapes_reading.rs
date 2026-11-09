//! Pins for CK-29 and CK-30, the shapes file and
//! reading.
//!
//! The shapes pins build their collections by hand, so
//! each one fails under the negation of the rule it
//! names, the sections standing per shape in the order
//! CK-3 and CK-5 state them, every shape standing
//! whether it raised or not, the judgement lines
//! naming each judgement by its file, its standing
//! number among the judgements of its file, its
//! category, and its set, and the closing counts in
//! their stated relations to the sections and to the
//! census closings over the same collection. The
//! loader pins name each defect of the file in the
//! forms CK-12 states, one pin per defect. The fencing
//! pins hold the rendering to the one line rule at the
//! library surface, the reading standing born inside
//! the fencing programme. Every pin runs offline on
//! recorded and constructed entries, and no pin makes
//! a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent
//! against the tree Round 13 closed on, by the absence
//! of the module it names, and within the slice each
//! pin discriminates against the wrong build its own
//! comment names. The golden pin discriminates by
//! content against any build whose counts, order,
//! sections, or judgement lines differ. The census
//! binding pins fail on a build whose counts travel a
//! second checking path or gather the pairs by any
//! attribution but the checker's own shape names. The
//! standing number pin fails on a build that renumbers
//! per shape. The same shape ordinal pin fails on a
//! build that states one line per file per shape. The
//! one line two shapes pin fails on a build that reads
//! one judgement per line. The empty shape and empty
//! collection pins fail on a build that stands only
//! the shapes that raised. The loader pins each fail
//! on a loader that accepts the defect they name. The
//! fencing pins each fail on a raw print in their
//! position, and the false counts pin fails on a
//! rendering that prints a handed judgements count
//! against the sections it states.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::render::{render_shapes, render_shapes_defects};
use stickler::shapes::{
    load_shapes, shapes_reading_of, ShapeJudgement, ShapesReading, ShapesSection,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-shapes-{}-{}-{}",
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

fn run_shapes(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "shapes.json", text);
    let (code, out, _err) = run(&["shapes", &path]);
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

/// One change set entry as JSON text.
fn entry_json(file: &str, diff: &str) -> String {
    format!(
        "{{\"file\":\"{}\",\"diff\":\"{}\"}}",
        jstr(file),
        jstr(diff)
    )
}

/// One collection text over the given set entries, each a
/// pair of a label and a change set JSON text, in the form
/// CK-29 and CK-12 share.
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

/// A diff holding one bare except judgement and no other,
/// the added except line closing the try block.
const BARE_EXCEPT_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -1,3 +1,5 @@\n def load():\n     try:\n         return read()\n+    except:\n+        return None\n";

/// A diff holding a class attribute judgement before a
/// call default judgement in one file, the basket shape
/// of the shipped round 2 set.
const CLASS_THEN_CALL_DIFF: &str = "--- a/order.py\n+++ b/order.py\n@@ -1,1 +1,4 @@\n class Order:\n+    seen = []\n+    def stamp(self, at=now()):\n+        return at\n";

/// A diff holding two print call judgements in one file
/// and no other judgement.
const TWO_PRINTS_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -0,0 +1,3 @@\n+def show(rows):\n+        print(rows)\n+        print(len(rows))\n";

/// A diff holding one added def line that raises both
/// the mutable default shape and the placeholder name
/// shape, the mutable default standing first in the
/// shape order of CK-3.
const ONE_LINE_TWO_SHAPES_DIFF: &str =
    "--- a/app.py\n+++ b/app.py\n@@ -0,0 +1,2 @@\n+def foo(history=[]):\n+    return history\n";

/// The section closing counts of a shapes rendering in
/// section order, read from the rendered text, with the
/// closing judgements read count standing last.
fn judgements_read_counts(text: &str) -> Vec<usize> {
    text.lines()
        .filter(|line| line.starts_with("Judgements read, "))
        .map(|line| {
            line.strip_prefix("Judgements read, ")
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect()
}

#[test]
fn the_example_matches_its_golden_through_the_library() {
    let text = std::fs::read_to_string(examples().join("shapes.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_shapes(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("shapes.txt")).unwrap();
    assert_eq!(render_shapes(&shapes_reading_of(&value)), golden);
}

/// The shipped collection, read whole. The round 2 set
/// carries all five judgements of the collection, the
/// pilot set and the constructed round 4 example carry
/// none, and the shapes closings read three sets and
/// five judgements, the counts the census reads over
/// the same collection in its totals.
#[test]
fn the_shipped_collection_closes_against_the_census_totals() {
    let text = std::fs::read_to_string(examples().join("shapes.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = shapes_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.sets_read, census.sets.len());
    let stated: usize = reading
        .sections
        .iter()
        .map(|section| section.judgements.len())
        .sum();
    assert_eq!(stated, 5);
    assert_eq!(stated, census.total.judgements);
    let rendered = render_shapes(&reading);
    assert!(rendered.lines().any(|line| line == "Sets read, 3"));
    assert!(rendered.lines().any(|line| line == "Judgements read, 5"));
}

/// Each section count of the shipped rendering equals
/// the Shape line the census total block states for
/// that shape over the same collection, read from the
/// two rendered texts, the six counts standing at one,
/// one, one, zero, one, and one in shape order.
#[test]
fn each_section_count_equals_the_census_shape_line_for_that_shape() {
    let text = std::fs::read_to_string(examples().join("shapes.json")).unwrap();
    let value = parse(&text).unwrap();
    let census_text = stickler::render::render_census(&census_of(&value));
    let total_block = census_text.split("\nTotal\n").nth(1).unwrap();
    let census_counts: Vec<usize> = total_block
        .lines()
        .filter(|line| line.starts_with("Shape, "))
        .map(|line| line.rsplit(", ").next().unwrap().parse().unwrap())
        .collect();
    assert_eq!(census_counts, vec![1, 1, 1, 0, 1, 1]);
    let rendered = render_shapes(&shapes_reading_of(&value));
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts.len(), 7);
    assert_eq!(counts[..6], census_counts[..]);
}

/// The section counts of the shipped rendering sum to
/// its closing judgements read count, read from the
/// rendered text.
#[test]
fn the_section_counts_sum_to_the_closing_judgements_read_count() {
    let text = std::fs::read_to_string(examples().join("shapes.json")).unwrap();
    let value = parse(&text).unwrap();
    let rendered = render_shapes(&shapes_reading_of(&value));
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts[..6].iter().sum::<usize>(), counts[6]);
    assert_eq!(counts[6], 5);
}

/// A constructed one set collection whose judgements
/// stand under one shape only renders the other five
/// sections standing, each stating the no judgements
/// line and closing at zero.
#[test]
fn a_collection_under_one_shape_stands_the_other_five_sections_empty() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", BARE_EXCEPT_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_shapes(&value), Vec::<String>::new());
    assert_eq!(
        render_shapes(&shapes_reading_of(&value)),
        "Shapes\n\
         \n\
         Shape, bare except\n\
         app.py, judgement 1, smell, set, one set\n\
         Judgements read, 1\n\
         \n\
         Shape, print call\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, mutable default\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, placeholder name\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, call default\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, class attribute\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Sets read, 1\n\
         Judgements read, 1\n"
    );
}

/// An empty collection renders the head line, the six
/// sections each stating the no judgements line and
/// closing at zero, and the closing counts at zero.
#[test]
fn an_empty_collection_renders_every_section_empty_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_shapes(&value), Vec::<String>::new());
    assert_eq!(
        render_shapes(&shapes_reading_of(&value)),
        "Shapes\n\
         \n\
         Shape, bare except\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, print call\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, mutable default\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, placeholder name\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, call default\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, class attribute\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Sets read, 0\n\
         Judgements read, 0\n"
    );
}

/// Over a constructed two set collection where one
/// shape carries a judgement in each set, the section
/// lists the first set's judgement before the second
/// set's, in collection order.
#[test]
fn a_shape_section_lists_judgements_in_collection_order_across_sets() {
    let one = BARE_EXCEPT_DIFF.replace("app.py", "one.py");
    let two = BARE_EXCEPT_DIFF.replace("app.py", "two.py");
    let text = collection_json(&[
        ("first set", change_set_json(&[entry_json("one.py", &one)])),
        ("second set", change_set_json(&[entry_json("two.py", &two)])),
    ]);
    let value = parse(&text).unwrap();
    let rendered = render_shapes(&shapes_reading_of(&value));
    let first = rendered
        .lines()
        .position(|line| line == "one.py, judgement 1, smell, set, first set")
        .unwrap();
    let second = rendered
        .lines()
        .position(|line| line == "two.py, judgement 1, smell, set, second set")
        .unwrap();
    assert!(first < second, "{rendered}");
}

/// The standing number of a judgement counts over all
/// the judgements of its file and does not restart per
/// shape, so a file carrying a class attribute
/// judgement before a call default judgement stands at
/// judgement 1 in the class attribute section and at
/// judgement 2 in the call default section.
#[test]
fn the_standing_number_counts_over_all_the_judgements_of_a_file() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("order.py", CLASS_THEN_CALL_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(
        render_shapes(&shapes_reading_of(&value)),
        "Shapes\n\
         \n\
         Shape, bare except\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, print call\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, mutable default\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, placeholder name\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Shape, call default\n\
         order.py, judgement 2, structure, set, one set\n\
         Judgements read, 1\n\
         \n\
         Shape, class attribute\n\
         order.py, judgement 1, structure, set, one set\n\
         Judgements read, 1\n\
         \n\
         Sets read, 1\n\
         Judgements read, 2\n"
    );
}

/// A file carrying two judgements of the same shape
/// stands twice in that shape's section, at two
/// ascending standing numbers.
#[test]
fn two_judgements_of_one_shape_in_one_file_stand_at_two_numbers() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", TWO_PRINTS_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_shapes(&shapes_reading_of(&value));
    let first = rendered
        .lines()
        .position(|line| line == "app.py, judgement 1, smell, set, one set")
        .unwrap();
    let second = rendered
        .lines()
        .position(|line| line == "app.py, judgement 2, smell, set, one set")
        .unwrap();
    assert_eq!(second, first + 1, "{rendered}");
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![0, 2, 0, 0, 0, 0, 2]);
}

/// One added def line raising two shapes stands in
/// both sections, at the standing numbers the shape
/// order of CK-3 gives them, the mutable default
/// number before the placeholder number.
#[test]
fn one_line_raising_two_shapes_stands_in_both_sections() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", ONE_LINE_TWO_SHAPES_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_shapes(&shapes_reading_of(&value));
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, structure, set, one set"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 2, naming, set, one set"),
        "{rendered}"
    );
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![0, 0, 1, 1, 0, 0, 2]);
}

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["shapes must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_shapes(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(load_shapes(&value), vec!["sets must be a list".to_string()]);
}

#[test]
fn an_entry_that_is_not_an_object_is_refused() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_that_is_empty_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_missing_change_set_field_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_shapes(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the last set of a collection renders
/// the defect list and no reading, whole or nothing,
/// so no partial shapes reading ever prints.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", BARE_EXCEPT_DIFF)])
    );
    let (code, out) = run_shapes(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Shapes\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// Build one judgement line by hand in the family's
/// form.
fn handed_judgement(file: &str, number: usize, category: &str, set_label: &str) -> ShapeJudgement {
    ShapeJudgement {
        file: file.to_string(),
        number,
        category: category.to_string(),
        set_label: set_label.to_string(),
    }
}

/// A file name holding a break and a judgements read
/// line's own text prints as the empty field, so no
/// judgement line or closing line is forged.
#[test]
fn a_line_break_in_a_judgement_file_forges_no_closing_line() {
    let reading = ShapesReading {
        sections: vec![ShapesSection {
            shape: "bare except".to_string(),
            judgements: vec![handed_judgement(
                "evil.py\nJudgements read, 99",
                1,
                "smell",
                "one set",
            )],
        }],
        sets_read: 1,
    };
    let rendered = render_shapes(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == ", judgement 1, smell, set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Judgements read, 99"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Judgements read, 1"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// A set label holding a break and a closing line's
/// own text prints as the empty field in the judgement
/// line, so no closing line is forged.
#[test]
fn a_line_break_in_a_set_label_forges_no_closing_line() {
    let reading = ShapesReading {
        sections: vec![ShapesSection {
            shape: "bare except".to_string(),
            judgements: vec![handed_judgement(
                "app.py",
                1,
                "smell",
                "one set\nSets read, 99",
            )],
        }],
        sets_read: 1,
    };
    let rendered = render_shapes(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, smell, set, "),
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

/// A category holding a break and a judgements read
/// line's own text prints as the empty field in its
/// judgement line, so no closing line is forged.
#[test]
fn a_line_break_in_a_category_is_fenced_in_its_judgement_line() {
    let reading = ShapesReading {
        sections: vec![ShapesSection {
            shape: "bare except".to_string(),
            judgements: vec![handed_judgement(
                "app.py",
                1,
                "smell\nJudgements read, 99",
                "one set",
            )],
        }],
        sets_read: 1,
    };
    let rendered = render_shapes(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, , set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Judgements read, 99"),
        "{rendered}"
    );
}

/// A section shape name holding a break and a
/// structural word prints fenced in its head, so the
/// head line stands as one line and no closing line
/// is forged, the posture CK-30 states for a handed
/// section outside the six shapes.
#[test]
fn a_line_break_in_a_section_shape_name_is_fenced_in_its_head() {
    let reading = ShapesReading {
        sections: vec![ShapesSection {
            shape: "bare except\nJudgements read, 99".to_string(),
            judgements: vec![handed_judgement("app.py", 1, "smell", "one set")],
        }],
        sets_read: 1,
    };
    let rendered = render_shapes(&reading);
    assert!(rendered.lines().any(|line| line == "Shape, "), "{rendered}");
    assert!(
        !rendered.lines().any(|line| line == "Judgements read, 99"),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Judgements read, 1"),
        "{rendered}"
    );
}

/// The closing judgements read count prints as the sum
/// over the sections the rendering states, so a handed
/// reading states the summed count its sections carry
/// and no handed count can print against them.
#[test]
fn the_closing_judgements_count_prints_as_the_sum_over_the_sections() {
    let reading = ShapesReading {
        sections: vec![
            ShapesSection {
                shape: "bare except".to_string(),
                judgements: vec![
                    handed_judgement("app.py", 1, "smell", "one set"),
                    handed_judgement("app.py", 2, "smell", "one set"),
                ],
            },
            ShapesSection {
                shape: "print call".to_string(),
                judgements: vec![handed_judgement("app.py", 3, "smell", "one set")],
            },
        ],
        sets_read: 1,
    };
    let rendered = render_shapes(&reading);
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![2, 1, 3]);
}

/// A malformed shapes rendering names its defects
/// under the Shapes head in the standing defect form.
#[test]
fn a_malformed_shapes_rendering_names_its_defects() {
    let rendered = render_shapes_defects(&["sets is missing".to_string()]);
    assert_eq!(
        rendered,
        "Shapes\nVerdict, invalid\nDefects, 1\nDefect, sets is missing\n"
    );
}
