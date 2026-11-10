//! Pins for CK-33 and CK-34, the categories file and
//! reading.
//!
//! The categories pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per category in the
//! order the formats section states the vocabulary,
//! naming, structure, and smell, every category standing
//! whether any judgement stands under it or not, the
//! judgement lines naming each judgement by its file,
//! its standing number among the judgements of its file,
//! its shape, and its set, and the closing counts in
//! their stated relations to the sections, to the shapes
//! reading regrouped by category, and to the census
//! closings over the same collection. The loader pins
//! name each defect of the file in the forms CK-12
//! states, one pin per defect. The fencing pins hold the
//! rendering to the one line rule at the library surface,
//! the reading standing born inside the fencing
//! programme. Every pin runs offline on recorded and
//! constructed entries, and no pin makes a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent at
//! base, against the tree Round 15 closed on, by the
//! absence of the module it names, and within the slice
//! each pin discriminates against the wrong build its
//! own comment names. The golden pin discriminates by
//! content against any build whose counts, order,
//! sections, or judgement lines differ, and it catches
//! the vocabulary order mutation MA and the within
//! section renumbering mutation MB on the shipped
//! collection. The shapes binding pin fails on a build
//! whose category assignment travels a second path,
//! deriving the category from the shape name by a table
//! of its own. The census binding pin fails on a build
//! whose counts travel a second checking path. The sum
//! pin catches the seat by line mutation MC where a
//! shared line would drop a judgement from a section
//! while the closing still counted it. The empty section
//! and empty collection pins fail on a build that stands
//! only the categories that raised. The cross set order
//! pin fails on a build that gathers by set completion
//! order or reverses the collection. The one line two
//! categories pin catches MC in its cross section form,
//! against a build that seats a line's judgements in one
//! section whole. The same category two shapes pin
//! catches MC in its within section form, against a build
//! that deduplicates a file within a section or seats by
//! line. The standing number pin catches MB, against
//! within section renumbering. The section order pin
//! catches MA in its pure form, against a build that
//! orders the sections by first appearance. The loader
//! pins each fail on a loader that accepts the defect
//! they name, and the whole or nothing pin catches the
//! dropped whole or nothing mutation MD. The fencing
//! pins each fail on a raw print in their position, the
//! first five catching the dropped fencing mutation ME
//! at their positions, and the false counts pin catches
//! the stored count mutation MF, against a rendering
//! that prints a handed judgements count against the
//! sections it states.

use std::path::{Path, PathBuf};

use stickler::categories::{
    categories_reading_of, load_categories, CategoriesReading, CategoriesSection, CategoryJudgement,
};
use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::json::parse;
use stickler::render::render_categories;
use stickler::shapes::shapes_reading_of;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-categories-{}-{}-{}",
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

fn run_categories(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "categories.json", text);
    let (code, out, _err) = run(&["categories", &path]);
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
/// CK-33 and CK-12 share.
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
/// of the shipped round 2 set, both judgements standing
/// under the structure category.
const CLASS_THEN_CALL_DIFF: &str = "--- a/order.py\n+++ b/order.py\n@@ -1,1 +1,4 @@\n class Order:\n+    seen = []\n+    def stamp(self, at=now()):\n+        return at\n";

/// A diff holding one added def line that raises both
/// the print call shape and the mutable default shape on
/// that one line, the print call standing first in the
/// shape order of CK-3, so the file carries one smell
/// judgement and one structure judgement over one line.
const ONE_LINE_TWO_CATEGORIES_DIFF: &str =
    "--- a/app.py\n+++ b/app.py\n@@ -0,0 +1 @@\n+def show(items=[]): print(items)\n";

/// A diff holding one added def line that raises both
/// the mutable default shape and the call default shape
/// on that one line, in that order in the shape order of
/// CK-3, both judgements standing under the structure
/// category under two shapes.
const SAME_CATEGORY_TWO_SHAPES_DIFF: &str =
    "--- a/app.py\n+++ b/app.py\n@@ -0,0 +1,2 @@\n+def stamp(at=now(), history=[]):\n+    return at\n";

/// A diff whose judgements interleave the categories in
/// file order, a call default under structure, a print
/// call under smell, and a mutable default under
/// structure, at standing numbers one, two, and three.
const INTERLEAVED_DIFF: &str = "--- a/app.py\n+++ b/app.py\n@@ -0,0 +1,4 @@\n+def stamp(at=now()):\n+    print(at)\n+def keep(history=[]):\n+    return history\n";

/// The section closing counts of a categories rendering
/// in section order, read from the rendered text, with
/// the closing judgements read count standing last.
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
    let text = std::fs::read_to_string(examples().join("categories.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_categories(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("categories.txt")).unwrap();
    assert_eq!(render_categories(&categories_reading_of(&value)), golden);
}

/// The shipped collection, read whole. The round 2 set
/// carries all five judgements of the collection, the
/// pilot set and the constructed round 4 example carry
/// none, and each section count of the categories
/// reading equals the count of that category's judgement
/// lines in the shapes reading over the same collection,
/// the two readings stating the same judgements under
/// two organizations, by shape and by category. The
/// categories closings read three sets and five
/// judgements, the counts the shapes reading closes on
/// over the same collection.
#[test]
fn the_shipped_collection_closes_against_the_shapes_reading() {
    let text = std::fs::read_to_string(examples().join("categories.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = categories_reading_of(&value);
    let shapes = shapes_reading_of(&value);
    for (section, category) in reading
        .sections
        .iter()
        .zip(["naming", "structure", "smell"])
    {
        assert_eq!(section.category, category);
        let regrouped = shapes
            .sections
            .iter()
            .flat_map(|shape_section| shape_section.judgements.iter())
            .filter(|judgement| judgement.category == category)
            .count();
        assert_eq!(section.judgements.len(), regrouped, "section {category}");
    }
    let counts: Vec<usize> = reading
        .sections
        .iter()
        .map(|section| section.judgements.len())
        .collect();
    assert_eq!(counts, vec![0, 3, 2]);
    let shapes_total: usize = shapes
        .sections
        .iter()
        .map(|section| section.judgements.len())
        .sum();
    assert_eq!(counts.iter().sum::<usize>(), shapes_total);
    assert_eq!(shapes_total, 5);
}

/// The shipped collection closes against the census
/// totals over the same collection, the categories sets
/// read count equal to the census set count and the
/// closing judgements read count equal to the census
/// total judgements count.
#[test]
fn the_shipped_collection_closes_against_the_census_totals() {
    let text = std::fs::read_to_string(examples().join("categories.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = categories_reading_of(&value);
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
    let rendered = render_categories(&reading);
    assert!(rendered.lines().any(|line| line == "Sets read, 3"));
    assert!(rendered.lines().any(|line| line == "Judgements read, 5"));
}

/// The section counts of the shipped rendering sum to
/// its closing judgements read count, read from the
/// rendered text.
#[test]
fn the_section_counts_sum_to_the_closing_judgements_read_count() {
    let text = std::fs::read_to_string(examples().join("categories.json")).unwrap();
    let value = parse(&text).unwrap();
    let rendered = render_categories(&categories_reading_of(&value));
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![0, 3, 2, 5]);
    assert_eq!(counts[..3].iter().sum::<usize>(), counts[3]);
}

/// A constructed one set collection whose judgements
/// stand under structure only renders the naming and
/// smell sections standing, each stating the no
/// judgements line and closing at zero.
#[test]
fn a_collection_under_one_category_stands_the_other_two_sections_empty() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("order.py", CLASS_THEN_CALL_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(load_categories(&value), Vec::<String>::new());
    assert_eq!(
        render_categories(&categories_reading_of(&value)),
        "Categories\n\
         \n\
         Category, naming\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Category, structure\n\
         order.py, judgement 1, class attribute, set, one set\n\
         order.py, judgement 2, call default, set, one set\n\
         Judgements read, 2\n\
         \n\
         Category, smell\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Sets read, 1\n\
         Judgements read, 2\n"
    );
}

/// An empty collection renders the head line, the three
/// sections each stating the no judgements line and
/// closing at zero, and the closing counts at zero.
#[test]
fn an_empty_collection_renders_every_section_empty_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_categories(&value), Vec::<String>::new());
    assert_eq!(
        render_categories(&categories_reading_of(&value)),
        "Categories\n\
         \n\
         Category, naming\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Category, structure\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Category, smell\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Sets read, 0\n\
         Judgements read, 0\n"
    );
}

/// Over a constructed two set collection where one
/// category carries a judgement in each set, the section
/// lists the first set's judgement before the second
/// set's, in collection order.
#[test]
fn a_category_section_lists_judgements_in_collection_order_across_sets() {
    let one = BARE_EXCEPT_DIFF.replace("app.py", "one.py");
    let two = BARE_EXCEPT_DIFF.replace("app.py", "two.py");
    let text = collection_json(&[
        ("first set", change_set_json(&[entry_json("one.py", &one)])),
        ("second set", change_set_json(&[entry_json("two.py", &two)])),
    ]);
    let value = parse(&text).unwrap();
    let rendered = render_categories(&categories_reading_of(&value));
    let first = rendered
        .lines()
        .position(|line| line == "one.py, judgement 1, bare except, set, first set")
        .unwrap();
    let second = rendered
        .lines()
        .position(|line| line == "two.py, judgement 1, bare except, set, second set")
        .unwrap();
    assert!(first < second, "{rendered}");
}

/// One added def line raising two shapes of two
/// categories stands in both sections over its two
/// judgements, the file standing in the structure
/// section over the mutable default judgement and in
/// the smell section over the print call judgement, each
/// line naming its own standing number and its own
/// shape, and never in one section whole over the line.
#[test]
fn one_line_raising_two_categories_stands_in_both_sections() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", ONE_LINE_TWO_CATEGORIES_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_categories(&categories_reading_of(&value));
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 2, mutable default, set, one set"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, print call, set, one set"),
        "{rendered}"
    );
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![0, 1, 1, 2]);
}

/// One added def line raising two shapes of one category
/// stands twice in that category's section, each line
/// naming its own shape and its own standing number, so
/// the section holds the file twice over two judgements
/// and not once over the line.
#[test]
fn one_line_raising_two_shapes_of_one_category_stands_twice_in_its_section() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", SAME_CATEGORY_TWO_SHAPES_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_categories(&categories_reading_of(&value));
    let first = rendered
        .lines()
        .position(|line| line == "app.py, judgement 1, mutable default, set, one set")
        .unwrap();
    let second = rendered
        .lines()
        .position(|line| line == "app.py, judgement 2, call default, set, one set")
        .unwrap();
    assert_eq!(second, first + 1, "{rendered}");
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![0, 2, 0, 2]);
}

/// The standing number of a judgement counts over all
/// the judgements of its file and does not restart per
/// category, so a file whose judgements interleave the
/// categories in file order, structure, smell, structure,
/// carries standing numbers one and three in the
/// structure section and never one and two.
#[test]
fn the_standing_number_counts_over_all_the_judgements_of_a_file() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", INTERLEAVED_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_categories(&categories_reading_of(&value));
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, call default, set, one set"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 3, mutable default, set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered
            .lines()
            .any(|line| line == "app.py, judgement 2, mutable default, set, one set"),
        "{rendered}"
    );
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![0, 2, 1, 3]);
}

/// Over a constructed collection whose only judgements
/// stand under smell, the naming and structure sections
/// still stand before the smell section, each stating
/// the no judgements line, the section order being the
/// stated vocabulary order and not the order of first
/// appearance in the collection.
#[test]
fn the_sections_stand_in_vocabulary_order_not_first_appearance_order() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json("app.py", BARE_EXCEPT_DIFF)]),
    )]);
    let value = parse(&text).unwrap();
    assert_eq!(
        render_categories(&categories_reading_of(&value)),
        "Categories\n\
         \n\
         Category, naming\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Category, structure\n\
         No judgements\n\
         Judgements read, 0\n\
         \n\
         Category, smell\n\
         app.py, judgement 1, bare except, set, one set\n\
         Judgements read, 1\n\
         \n\
         Sets read, 1\n\
         Judgements read, 1\n"
    );
}

#[test]
fn a_value_that_is_not_an_object_is_refused() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["categories must be a JSON object".to_string()]
    );
}

#[test]
fn a_missing_sets_field_is_refused() {
    let value = parse("{}").unwrap();
    assert_eq!(load_categories(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn a_sets_field_that_is_not_a_list_is_refused() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets must be a list".to_string()]
    );
}

#[test]
fn an_entry_that_is_not_an_object_is_refused() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn a_label_that_is_empty_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_missing_change_set_field_is_refused() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_stands_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_categories(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the last set of a collection renders
/// the defect list and no reading, whole or nothing,
/// so no partial categories reading ever prints.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", BARE_EXCEPT_DIFF)])
    );
    let (code, out) = run_categories(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Categories\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// Build one judgement line by hand in the family's
/// form.
fn handed_judgement(file: &str, number: usize, shape: &str, set_label: &str) -> CategoryJudgement {
    CategoryJudgement {
        file: file.to_string(),
        number,
        shape: shape.to_string(),
        set_label: set_label.to_string(),
    }
}

/// A file name holding a break and a judgements read
/// line's own text prints as the empty field, so no
/// judgement line or closing line is forged.
#[test]
fn a_line_break_in_a_judgement_file_forges_no_closing_line() {
    let reading = CategoriesReading {
        sections: vec![CategoriesSection {
            category: "smell".to_string(),
            judgements: vec![handed_judgement(
                "evil.py\nJudgements read, 99",
                1,
                "print call",
                "one set",
            )],
        }],
        sets_read: 1,
    };
    let rendered = render_categories(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == ", judgement 1, print call, set, one set"),
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
    let reading = CategoriesReading {
        sections: vec![CategoriesSection {
            category: "smell".to_string(),
            judgements: vec![handed_judgement(
                "app.py",
                1,
                "print call",
                "one set\nSets read, 99",
            )],
        }],
        sets_read: 1,
    };
    let rendered = render_categories(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, print call, set, "),
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

/// A shape name holding a break and a judgements read
/// line's own text prints as the empty field in its
/// judgement line, so no closing line is forged.
#[test]
fn a_line_break_in_a_shape_name_is_fenced_in_its_judgement_line() {
    let reading = CategoriesReading {
        sections: vec![CategoriesSection {
            category: "smell".to_string(),
            judgements: vec![handed_judgement(
                "app.py",
                1,
                "print call\nJudgements read, 99",
                "one set",
            )],
        }],
        sets_read: 1,
    };
    let rendered = render_categories(&reading);
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

/// A section category name holding a break and a
/// structural word prints fenced in its head, so the
/// head line stands as one line and no closing line
/// is forged.
#[test]
fn a_line_break_in_a_section_category_name_is_fenced_in_its_head() {
    let reading = CategoriesReading {
        sections: vec![CategoriesSection {
            category: "smell\nJudgements read, 99".to_string(),
            judgements: vec![handed_judgement("app.py", 1, "print call", "one set")],
        }],
        sets_read: 1,
    };
    let rendered = render_categories(&reading);
    assert!(
        rendered.lines().any(|line| line == "Category, "),
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
}

/// A handed section carrying a category outside the
/// three words prints its head as it stands and forges
/// no line, and the reading over it is not the
/// categories reading of any collection, in the posture
/// CK-34 states for handed categories outside the
/// reading's stated form.
#[test]
fn a_handed_category_outside_the_three_words_prints_its_head_and_forges_no_line() {
    let reading = CategoriesReading {
        sections: vec![CategoriesSection {
            category: "bizarre".to_string(),
            judgements: vec![handed_judgement("app.py", 1, "print call", "one set")],
        }],
        sets_read: 1,
    };
    let rendered = render_categories(&reading);
    assert!(
        rendered.lines().any(|line| line == "Category, bizarre"),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, judgement 1, print call, set, one set"),
        "{rendered}"
    );
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![1, 1]);
}

/// The closing judgements read count prints as the sum
/// over the sections the rendering states, so a handed
/// reading states the summed count its sections carry
/// and no handed count can print against them.
#[test]
fn the_closing_judgements_count_prints_as_the_sum_over_the_sections() {
    let reading = CategoriesReading {
        sections: vec![
            CategoriesSection {
                category: "naming".to_string(),
                judgements: vec![
                    handed_judgement("app.py", 1, "placeholder name", "one set"),
                    handed_judgement("app.py", 2, "placeholder name", "one set"),
                ],
            },
            CategoriesSection {
                category: "smell".to_string(),
                judgements: vec![handed_judgement("app.py", 3, "print call", "one set")],
            },
        ],
        sets_read: 1,
    };
    let rendered = render_categories(&reading);
    let counts = judgements_read_counts(&rendered);
    assert_eq!(counts, vec![2, 1, 3]);
}
