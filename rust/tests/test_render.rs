//! Pins for the rendering shared by every command, and the
//! golden files. Translated from tests/test_render.py.

use std::path::PathBuf;

use stickler::changeset::{check_changeset, review_changeset, Block, FileResult};
use stickler::checker::{Flag, Objection};
use stickler::json::{parse, JString, JsonNumber, JsonValue};
use stickler::rejudged::{ComparedInput, Rejudged};
use stickler::render::{
    render_rejudged, render_review, render_set_defects, render_set_review, render_split,
};
use stickler::split::{Split, SplitFlag};
use stickler::verdicts::VerdictEntry;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    JsonValue::Object(
        pairs
            .into_iter()
            .map(|(key, value)| (JString::from_text(key), value))
            .collect(),
    )
}

fn text(value: &str) -> JsonValue {
    JsonValue::Str(JString::from_text(value))
}

const OBJECTION: &str = r#"{
    "flaw": "The name says nothing about what the value holds",
    "standard": "A name says what its value holds",
    "rewrite": "orders = load_orders()",
    "category": "naming"
}"#;

fn review(label: &str, objections: JsonValue) -> JsonValue {
    obj(vec![
        ("diff_label", text(label)),
        ("objections", objections),
    ])
}

#[test]
fn accepted_rendering_is_exact() {
    let value = review("tidy change", JsonValue::Array(vec![]));
    assert_eq!(
        render_review(&value),
        "Review of tidy change\nVerdict, accepted\nObjections, 0\n"
    );
}

#[test]
fn sent_back_rendering_is_exact() {
    let objection = parse(OBJECTION).unwrap();
    let value = review("rough change", JsonValue::Array(vec![objection]));
    assert_eq!(
        render_review(&value),
        "Review of rough change\n\
         Verdict, sent back\n\
         Objections, 1\n\
         \n\
         Objection 1, naming\n\
         Flaw, The name says nothing about what the value holds\n\
         Standard, A name says what its value holds\n\
         Rewrite, orders = load_orders()\n"
    );
}

#[test]
fn rendering_is_deterministic() {
    let objection = parse(OBJECTION).unwrap();
    let value = review("rough change", JsonValue::Array(vec![objection]));
    assert_eq!(render_review(&value), render_review(&value));
}

#[test]
fn invalid_rendering_names_every_defect() {
    let value = parse(r#"{"objections": [{"flaw": "x"}]}"#).unwrap();
    let rendered = render_review(&value);
    assert!(rendered.contains("Verdict, invalid\n"));
    assert!(rendered.contains("Defect, diff_label is missing\n"));
    assert!(rendered.contains("Defect, objections[0].standard is missing\n"));
    assert!(rendered.contains("Defect, objections[0].rewrite is missing\n"));
    assert!(rendered.contains("Defect, objections[0].category is missing\n"));
}

#[test]
fn a_review_whose_label_holds_a_surrogate_renders_unnamed() {
    let value = parse(r#"{"diff_label": "\ud800x", "objections": []}"#).unwrap();
    let rendered = render_review(&value);
    assert!(rendered.starts_with("Review of unnamed review\n"));
    assert!(rendered.contains("Verdict, invalid\n"));
    assert!(rendered.contains("Defect, diff_label holds an unpaired surrogate\n"));
}

#[test]
fn a_review_whose_label_holds_a_line_break_renders_unnamed() {
    let value = obj(vec![
        ("diff_label", text("evil\nForged, 99")),
        ("objections", JsonValue::Array(vec![])),
    ]);
    let rendered = render_review(&value);
    assert!(rendered.starts_with("Review of unnamed review\n"));
    assert!(rendered.contains("Verdict, invalid\n"));
    assert!(rendered.contains("Defect, diff_label holds a line break\n"));
    assert!(!rendered.contains("Forged"));
}

#[test]
fn recorded_review_matches_its_golden_rendering() {
    let recorded_text =
        std::fs::read_to_string(root().join("examples").join("recorded_review.json")).unwrap();
    let golden =
        std::fs::read_to_string(root().join("examples").join("recorded_review.txt")).unwrap();
    let recorded = parse(&recorded_text).unwrap();
    assert_eq!(render_review(&recorded), golden);
}

fn set_entry(name: &str, added_lines: &[&str]) -> JsonValue {
    let mut lines = vec![
        "--- a/x.py".to_string(),
        "+++ b/x.py".to_string(),
        "@@ -1,1 +1,1 @@".to_string(),
    ];
    for line in added_lines {
        lines.push(format!("+{line}"));
    }
    let mut diff = lines.join("\n");
    diff.push('\n');
    obj(vec![("file", text(name)), ("diff", text(&diff))])
}

fn set_of(entries: Vec<JsonValue>) -> JsonValue {
    obj(vec![("files", JsonValue::Array(entries))])
}

#[test]
fn a_clean_set_rendering_is_exact() {
    let value = set_of(vec![set_entry("a.py", &["    logger.debug(x)"])]);
    let results = check_changeset(&value);
    assert_eq!(
        render_set_review(&results),
        "Change set review\n\
         Files, 1\n\
         \n\
         File, a.py\n\
         Clean\n\
         \n\
         Summary\n\
         Judgements, 0\n"
    );
}

#[test]
fn a_set_rendering_is_exact_with_a_judgement() {
    let value = set_of(vec![set_entry("a.py", &["    print(total)"])]);
    let results = check_changeset(&value);
    assert_eq!(
        render_set_review(&results),
        "Change set review\n\
         Files, 1\n\
         \n\
         File, a.py\n\
         Judgements, 1\n\
         \n\
         Judgement 1, smell\n\
         Flaw, The added line sends diagnostics through a print call\n\
         Standard, Diagnostics travel through the logging module so they \
         can be levelled, filtered, and silenced\n\
         Rewrite,     logger.debug(total)\n\
         \n\
         Summary\n\
         Judgements, 1\n\
         Shape, print call, 1\n\
         Files with judgements, worst first\n\
         Worst, a.py, 1\n"
    );
}

#[test]
fn the_summary_counts_by_shape_in_the_stated_shape_order() {
    let value = set_of(vec![
        set_entry("a.py", &["def stamp(at=now()):", "    print(total)"]),
        set_entry("b.py", &["    except:"]),
    ]);
    let rendered = render_set_review(&check_changeset(&value));
    let shape_lines: Vec<&str> = rendered
        .lines()
        .filter(|line| line.starts_with("Shape, "))
        .collect();
    assert_eq!(
        shape_lines,
        vec![
            "Shape, bare except, 1",
            "Shape, print call, 1",
            "Shape, call default, 1",
        ]
    );
}

#[test]
fn the_worst_first_order_breaks_ties_by_file_name() {
    let value = set_of(vec![
        set_entry("zeta.py", &["    print(total)"]),
        set_entry("mid.py", &["    print(total)", "    except:"]),
        set_entry("alpha.py", &["    print(total)"]),
        set_entry("clean.py", &["    logger.debug(total)"]),
    ]);
    let rendered = render_set_review(&check_changeset(&value));
    let worst_lines: Vec<&str> = rendered
        .lines()
        .filter(|line| line.starts_with("Worst, "))
        .collect();
    assert_eq!(
        worst_lines,
        vec![
            "Worst, mid.py, 2",
            "Worst, alpha.py, 1",
            "Worst, zeta.py, 1"
        ]
    );
}

#[test]
fn the_set_rendering_is_deterministic() {
    let value = set_of(vec![set_entry("a.py", &["    print(total)"])]);
    let results = check_changeset(&value);
    assert_eq!(render_set_review(&results), render_set_review(&results));
}

#[test]
fn the_example_change_set_matches_its_golden_rendering() {
    let set_text =
        std::fs::read_to_string(root().join("examples").join("change_set.json")).unwrap();
    let golden =
        std::fs::read_to_string(root().join("examples").join("change_set_review.txt")).unwrap();
    let value = parse(&set_text).unwrap();
    assert_eq!(render_set_review(&check_changeset(&value)), golden);
}

#[test]
fn a_contested_flag_gains_the_section_and_the_count() {
    let diff = "--- a/x.py\n+++ b/x.py\n@@ -1,1 +1,1 @@\n+data = load_entries()\n";
    let verdicts = JsonValue::Array(vec![
        parse(
            r#"{"line": 4, "pattern": "near placeholder name",
                "model": "jev-1.13.0", "choice": "stickler",
                "confidence": 0.81}"#,
        )
        .unwrap(),
        parse(
            r#"{"line": 4, "pattern": "near placeholder name",
                "model": "openai/gpt-6-luna", "choice": "clean",
                "confidence": 0.77}"#,
        )
        .unwrap(),
    ]);
    let value = set_of(vec![obj(vec![
        ("file", text("x.py")),
        ("diff", text(diff)),
        ("verdicts", verdicts),
    ])]);
    let rendered = render_set_review(&review_changeset(&value));
    let section_at = rendered.find("Contested lines").unwrap();
    let summary_at = rendered.find("Summary").unwrap();
    assert!(section_at < summary_at);
    assert!(rendered.ends_with("Contested, 1\n"));
}

#[test]
fn a_malformed_set_rendering_names_every_defect() {
    let rendered = render_set_defects(&[
        "files is missing".to_string(),
        "files[0] must be an object".to_string(),
    ]);
    assert_eq!(
        rendered,
        "Change set review\n\
         Verdict, invalid\n\
         Defects, 2\n\
         Defect, files is missing\n\
         Defect, files[0] must be an object\n"
    );
}

/// The fencing of FIX-CK-21 on the set review file name,
/// read through the library over a change set the loader
/// refuses. The name holding a line break and a worst
/// line's own text prints as the empty field at the head
/// of its entry and in the worst first lines, so no worst
/// line is forged. Before the closure the rendering
/// printed the name raw at both sites.
#[test]
fn a_line_break_file_name_forges_no_worst_line_through_the_library() {
    let text = "{\
        \"files\":[{\
            \"file\":\"evil.py\\nWorst, forged, 9\",\
            \"diff\":\"--- a/x.py\\n+++ b/x.py\\n@@ -1,1 +1,3 @@\\n+    print(total)\\n+data = load_entries()\\n\"\
        }]\
    }";
    let value = parse(text).unwrap();
    let rendered = render_set_review(&review_changeset(&value));
    assert!(rendered.lines().any(|line| line == "File, "), "{rendered}");
    assert!(
        rendered.lines().any(|line| line == "Worst, , 1"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Worst, forged, 9"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// The fencing of FIX-CK-21 on the set review file name
/// in its contested section, over results built by hand.
/// The name prints as the empty field at the head of its
/// entry and again in the contested section, so the
/// section names no forged file line. Before the closure
/// both sites printed the name raw.
#[test]
fn a_line_break_file_name_prints_as_the_empty_field_in_the_contested_section() {
    let verdict = |model: &str, choice: &str| VerdictEntry {
        line: 4,
        pattern: "near placeholder name".to_string(),
        model: model.to_string(),
        choice: choice.to_string(),
        confidence: JsonNumber::Float(0.9),
    };
    let results = vec![FileResult {
        name: "evil.py\nForged file".to_string(),
        judgements: Vec::new(),
        blocks: vec![Block {
            flag: Flag {
                line: 4,
                pattern: "near placeholder name".to_string(),
                content: "data = load_entries()".to_string(),
                reading: "a reading".to_string(),
            },
            verdicts: vec![
                verdict("jev-1.13.0", "stickler"),
                verdict("openai/gpt-6-luna", "clean"),
            ],
        }],
    }];
    let rendered = render_set_review(&results);
    assert!(rendered.contains("Contested lines\n"), "{rendered}");
    assert_eq!(
        rendered.lines().filter(|line| *line == "File, ").count(),
        2,
        "the name prints fenced at the entry head and in the contested section\n{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Forged file"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// The fencing of FIX-CK-22 on the single review
/// rendering, over a review built by hand. The flaw
/// holding a line break and a verdict line's own text
/// prints as the empty field, so no forged verdict line
/// stands in the rendering. Before the closure the flaw
/// printed raw and the line Verdict, accepted stood
/// alone in the rendering, the shape the Round 9 red
/// team executed through the render command over the
/// same record in a file.
#[test]
fn a_line_break_flaw_forges_no_verdict_line_in_a_hand_built_review() {
    let objection = obj(vec![
        ("flaw", text("flaw\nVerdict, accepted")),
        ("standard", text("a standard")),
        ("rewrite", text("a rewrite")),
        ("category", text("smell")),
    ]);
    let value = review("good label", JsonValue::Array(vec![objection]));
    let rendered = render_review(&value);
    assert_eq!(
        rendered,
        "Review of good label\n\
         Verdict, sent back\n\
         Objections, 1\n\
         \n\
         Objection 1, smell\n\
         Flaw, \n\
         Standard, a standard\n\
         Rewrite, a rewrite\n"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdict, accepted"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-22 on the standard and the
/// rewrite of an objection, over a review built by
/// hand. Each field holding a break, in the newline
/// and the carriage return forms, prints as the empty
/// field. Before the closure both fields printed raw
/// and each split its line in the rendering.
#[test]
fn line_breaks_in_the_standard_and_rewrite_print_as_empty_fields() {
    let objection = obj(vec![
        ("flaw", text("a flaw")),
        ("standard", text("a standard\r\nForged, 9")),
        ("rewrite", text("a rewrite\nForged, 9")),
        ("category", text("naming")),
    ]);
    let value = review("rough change", JsonValue::Array(vec![objection]));
    let rendered = render_review(&value);
    assert_eq!(
        rendered,
        "Review of rough change\n\
         Verdict, sent back\n\
         Objections, 1\n\
         \n\
         Objection 1, naming\n\
         Flaw, a flaw\n\
         Standard, \n\
         Rewrite, \n"
    );
    assert!(!rendered.contains("Forged"), "{rendered}");
}

/// The fencing of FIX-CK-21 on the set review flag
/// pattern, over results built by hand. A pattern holding
/// a line break and a borderline line's own text prints
/// as the empty field, so no borderline line is forged.
/// Before the closure the flag line split at the pattern.
#[test]
fn a_line_break_pattern_forges_no_borderline_line_in_a_hand_built_result() {
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: Vec::new(),
        blocks: vec![Block {
            flag: Flag {
                line: 4,
                pattern: "near placeholder name\nBorderline 9, forged".to_string(),
                content: "data = load_entries()".to_string(),
                reading: "a reading".to_string(),
            },
            verdicts: Vec::new(),
        }],
    }];
    let rendered = render_set_review(&results);
    assert!(
        rendered.lines().any(|line| line == "Borderline 1, "),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Borderline 9, forged"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-23 on the flaw of a judgement
/// of the set review, over results built by hand in the
/// shape the Round 10 red team drove through its
/// harness. The flaw holding a line break and a verdict
/// line's own text prints as the empty field, so no
/// forged verdict line stands in the rendering. Before
/// the closure the flaw printed raw and the line
/// Verdict, accepted stood alone in the rendering.
#[test]
fn a_line_break_flaw_forges_no_verdict_line_in_a_hand_built_result() {
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: vec![(
            "print call",
            Objection {
                flaw: "flaw\nVerdict, accepted".to_string(),
                standard: "a standard".to_string(),
                rewrite: "a rewrite".to_string(),
                category: "smell".to_string(),
            },
        )],
        blocks: Vec::new(),
    }];
    let rendered = render_set_review(&results);
    assert_eq!(
        rendered,
        "Change set review\n\
         Files, 1\n\
         \n\
         File, app.py\n\
         Judgements, 1\n\
         \n\
         Judgement 1, smell\n\
         Flaw, \n\
         Standard, a standard\n\
         Rewrite, a rewrite\n\
         \n\
         Summary\n\
         Judgements, 1\n\
         Shape, print call, 1\n\
         Files with judgements, worst first\n\
         Worst, app.py, 1\n"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdict, accepted"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-23 on the standard and the
/// rewrite of a judgement of the set review, over
/// results built by hand. The standard holding a break
/// and an objections count line's own text and the
/// rewrite holding a break and the summary head's own
/// text each print as the empty field. Before the
/// closure both fields printed raw, the standard
/// minting the forged count line and the rewrite a
/// second summary line standing alone in the stream.
#[test]
fn line_breaks_in_the_standard_and_rewrite_forge_no_count_or_summary_line() {
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: vec![(
            "print call",
            Objection {
                flaw: "a flaw".to_string(),
                standard: "a standard\nObjections, 99".to_string(),
                rewrite: "a rewrite\r\nSummary".to_string(),
                category: "smell".to_string(),
            },
        )],
        blocks: Vec::new(),
    }];
    let rendered = render_set_review(&results);
    assert!(
        rendered.lines().any(|line| line == "Standard, "),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Rewrite, "),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Objections, 99"),
        "{rendered}"
    );
    assert_eq!(
        rendered.lines().filter(|line| *line == "Summary").count(),
        1,
        "{rendered}"
    );
}

/// The fencing of FIX-CK-23 on the category of a
/// judgement of the set review, over results built by
/// hand. The category holding a break and a judgement
/// line's own text prints as the empty field in the
/// judgement head. Before the closure the category
/// printed raw and split the head line.
#[test]
fn a_line_break_category_prints_as_the_empty_field_in_a_hand_built_result() {
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: vec![(
            "print call",
            Objection {
                flaw: "a flaw".to_string(),
                standard: "a standard".to_string(),
                rewrite: "a rewrite".to_string(),
                category: "smell\nJudgement 9, forged".to_string(),
            },
        )],
        blocks: Vec::new(),
    }];
    let rendered = render_set_review(&results);
    assert!(
        rendered.lines().any(|line| line == "Judgement 1, "),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Judgement 9, forged"),
        "{rendered}"
    );
    assert!(!rendered.contains("forged"), "{rendered}");
}

/// The fencing of FIX-CK-23 on the content and the
/// shape reading of a flag of the set review, over
/// results built by hand. The content holding a break
/// and the contested head's own text and the reading
/// holding a break and the summary head's own text
/// each print as the empty field. Before the closure
/// both fields printed raw, the content minting the
/// contested head over a flag nothing contests and
/// the reading a second summary line.
#[test]
fn line_breaks_in_the_flag_content_and_reading_forge_no_contested_or_summary_line() {
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: Vec::new(),
        blocks: vec![Block {
            flag: Flag {
                line: 4,
                pattern: "broad handler".to_string(),
                content: "content\nContested lines".to_string(),
                reading: "reading\nSummary".to_string(),
            },
            verdicts: Vec::new(),
        }],
    }];
    let rendered = render_set_review(&results);
    assert_eq!(
        rendered,
        "Change set review\n\
         Files, 1\n\
         \n\
         File, app.py\n\
         \n\
         Borderline 1, broad handler\n\
         Line, \n\
         Shape reading, \n\
         Unjudged\n\
         \n\
         Summary\n\
         Judgements, 0\n\
         Borderline, 1\n\
         Verdict lines, stickler 0, clean 0\n\
         Unjudged, 1\n\
         Contested, 0\n"
    );
    assert!(
        !rendered.lines().any(|line| line == "Contested lines"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-23 on the choice of a recorded
/// verdict of the set review, over results built by
/// hand, the choice exercised in both its labels. Each
/// choice holding a break prints as the empty field in
/// its verdict line, and the reason the rendering
/// builds from the model and the choice, holding the
/// break the choice carries, prints as the empty field
/// whole. Before the closure the choice printed raw in
/// both places and the reason line split at the choice.
#[test]
fn line_breaks_in_the_choice_print_as_empty_fields_in_verdict_and_reason() {
    let verdict = |model: &str, choice: &str| VerdictEntry {
        line: 4,
        pattern: "near placeholder name".to_string(),
        model: model.to_string(),
        choice: choice.to_string(),
        confidence: JsonNumber::Float(0.9),
    };
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: Vec::new(),
        blocks: vec![Block {
            flag: Flag {
                line: 4,
                pattern: "near placeholder name".to_string(),
                content: "data = load_entries()".to_string(),
                reading: "a reading".to_string(),
            },
            verdicts: vec![
                verdict("jev-1.13.0", "stickler\nVerdict, accepted"),
                verdict("openai/gpt-6-luna", "clean\rVerdict, sent back"),
            ],
        }],
    }];
    let rendered = render_set_review(&results);
    assert!(
        rendered.lines().any(|line| line == "Verdict, jev-1.13.0, "),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "Verdict, openai/gpt-6-luna, "),
        "{rendered}"
    );
    assert_eq!(
        rendered.lines().filter(|line| *line == "Reason, ").count(),
        2,
        "{rendered}"
    );
    assert!(!rendered.contains("chose"), "{rendered}");
    assert!(
        !rendered.lines().any(|line| line == "Verdict, accepted"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdict, sent back"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-23 on the model of a recorded
/// verdict of the set review, over results built by
/// hand, on an abstained verdict and on a contested
/// flag both. The model holding a break prints as the
/// empty field in the abstained verdict line, in the
/// verdict lines of the contested flag, and in the
/// verdict lines of the contested section, and the
/// flag content holding a break prints as the empty
/// field in the contested section as in its block, and
/// the reasons built from the models print as the
/// empty field whole. Before the closure the model
/// printed raw at every one of those sites.
#[test]
fn line_breaks_in_the_model_print_as_empty_fields_in_verdict_reason_and_contested() {
    let verdict = |model: &str, choice: &str, confidence: f64| VerdictEntry {
        line: 4,
        pattern: "near placeholder name".to_string(),
        model: model.to_string(),
        choice: choice.to_string(),
        confidence: JsonNumber::Float(confidence),
    };
    let results = vec![FileResult {
        name: "app.py".to_string(),
        judgements: Vec::new(),
        blocks: vec![
            Block {
                flag: Flag {
                    line: 4,
                    pattern: "near placeholder name".to_string(),
                    content: "data = load_entries()".to_string(),
                    reading: "a reading".to_string(),
                },
                verdicts: vec![verdict("model\nVerdict, accepted", "stickler", 0.3)],
            },
            Block {
                flag: Flag {
                    line: 5,
                    pattern: "broad handler".to_string(),
                    content: "content\nSummary".to_string(),
                    reading: "a reading".to_string(),
                },
                verdicts: vec![
                    verdict("jev\nVerdict, forged", "stickler", 0.9),
                    verdict("openai\nContested, 99", "clean", 0.9),
                ],
            },
        ],
    }];
    let rendered = render_set_review(&results);
    assert!(
        rendered.lines().any(|line| line == "Verdict, , abstained"),
        "{rendered}"
    );
    assert_eq!(
        rendered
            .lines()
            .filter(|line| *line == "Verdict, , stickler")
            .count(),
        2,
        "the stickler verdict line stands fenced in its block and in the contested section\n{rendered}"
    );
    assert_eq!(
        rendered
            .lines()
            .filter(|line| *line == "Verdict, , clean")
            .count(),
        2,
        "the clean verdict line stands fenced in its block and in the contested section\n{rendered}"
    );
    assert_eq!(
        rendered.lines().filter(|line| *line == "Line, ").count(),
        2,
        "the content of the contested flag prints fenced in its block and in the contested section\n{rendered}"
    );
    assert_eq!(
        rendered.lines().filter(|line| *line == "Reason, ").count(),
        3,
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdict, accepted"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Verdict, forged"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Contested, 99"),
        "{rendered}"
    );
    assert_eq!(
        rendered.lines().filter(|line| *line == "Summary").count(),
        1,
        "{rendered}"
    );
}

/// The fencing of FIX-CK-25 on the first judgement word
/// of a compared input of the rejudged reading, over a
/// reading built by hand. The word holding a line break
/// and a held count line's own text prints as the empty
/// field. Before the closure the word printed raw and
/// the forged count line stood in the rendering.
#[test]
fn a_line_break_in_the_first_word_of_a_rejudged_pair_forges_no_count_line() {
    let reading = Rejudged {
        first_label: "first set".to_string(),
        second_label: "second set".to_string(),
        pairs: vec![ComparedInput {
            file: "app.py".to_string(),
            line: 4,
            pattern: "near placeholder name".to_string(),
            first: "stickler\nHeld, 99",
            second: "stickler",
            held: true,
        }],
        inputs_read: 1,
        held: 1,
        changed: 0,
    };
    let rendered = render_rejudged(&reading);
    assert!(rendered.lines().any(|line| line == "First, "), "{rendered}");
    assert!(
        rendered.lines().any(|line| line == "Second, stickler"),
        "{rendered}"
    );
    assert!(rendered.lines().any(|line| line == "Held, 1"), "{rendered}");
    assert!(
        !rendered.lines().any(|line| line == "Held, 99"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-25 on the second judgement word
/// of a compared input of the rejudged reading, over a
/// reading built by hand. The word holding a carriage
/// return, a line break, and a changed count line's own
/// text prints as the empty field. Before the closure
/// the word printed raw and split its line.
#[test]
fn a_line_break_in_the_second_word_of_a_rejudged_pair_forges_no_count_line() {
    let reading = Rejudged {
        first_label: "first set".to_string(),
        second_label: "second set".to_string(),
        pairs: vec![ComparedInput {
            file: "app.py".to_string(),
            line: 4,
            pattern: "near placeholder name".to_string(),
            first: "stickler",
            second: "clean\r\nChanged, 99",
            held: false,
        }],
        inputs_read: 1,
        held: 0,
        changed: 1,
    };
    let rendered = render_rejudged(&reading);
    assert!(
        rendered.lines().any(|line| line == "Second, "),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Changed, 1"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Changed, 99"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-25 on the first route word of a
/// flag of the split reading, over a reading built by
/// hand. The word holding a line break and an agree
/// count line's own text prints as the empty field.
/// Before the closure the word printed raw and the
/// forged count line stood in the rendering.
#[test]
fn a_line_break_in_the_first_word_of_a_split_flag_forges_no_count_line() {
    let reading = Split {
        first_route: "route one".to_string(),
        second_route: "route two".to_string(),
        flags: vec![SplitFlag {
            file: "app.py".to_string(),
            line: 4,
            pattern: "near placeholder name".to_string(),
            first: "stickler\nRoutes agree, 99",
            second: "stickler",
            joint: "stickler",
        }],
        routes_agree: 0,
        routes_differ: 1,
        joint_differs: 1,
    };
    let rendered = render_split(&reading);
    assert!(
        rendered.lines().any(|line| line == "First route, "),
        "{rendered}"
    );
    assert!(
        rendered
            .lines()
            .any(|line| line == "Second route, stickler"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Routes agree, 99"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-25 on the second route word of
/// a flag of the split reading, over a reading built by
/// hand. The word holding a line break and a differ
/// count line's own text prints as the empty field.
/// Before the closure the word printed raw and the
/// forged count line stood in the rendering.
#[test]
fn a_line_break_in_the_second_word_of_a_split_flag_forges_no_count_line() {
    let reading = Split {
        first_route: "route one".to_string(),
        second_route: "route two".to_string(),
        flags: vec![SplitFlag {
            file: "app.py".to_string(),
            line: 4,
            pattern: "near placeholder name".to_string(),
            first: "clean",
            second: "stickler\nRoutes differ, 99",
            joint: "clean",
        }],
        routes_agree: 0,
        routes_differ: 1,
        joint_differs: 1,
    };
    let rendered = render_split(&reading);
    assert!(
        rendered.lines().any(|line| line == "Second route, "),
        "{rendered}"
    );
    assert!(
        rendered.lines().any(|line| line == "Routes differ, 1"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Routes differ, 99"),
        "{rendered}"
    );
}

/// The fencing of FIX-CK-25 on the joint word of a flag
/// of the split reading, over a reading built by hand.
/// The word holding a line break and a joint count
/// line's own text prints as the empty field. Before
/// the closure the word printed raw and the forged
/// count line stood in the rendering.
#[test]
fn a_line_break_in_the_joint_word_of_a_split_flag_forges_no_count_line() {
    let reading = Split {
        first_route: "route one".to_string(),
        second_route: "route two".to_string(),
        flags: vec![SplitFlag {
            file: "app.py".to_string(),
            line: 4,
            pattern: "near placeholder name".to_string(),
            first: "stickler",
            second: "stickler",
            joint: "unjudged\nJoint differs from a route, 99",
        }],
        routes_agree: 1,
        routes_differ: 0,
        joint_differs: 1,
    };
    let rendered = render_split(&reading);
    assert!(rendered.lines().any(|line| line == "Joint, "), "{rendered}");
    assert!(
        rendered
            .lines()
            .any(|line| line == "Joint differs from a route, 1"),
        "{rendered}"
    );
    assert!(
        !rendered
            .lines()
            .any(|line| line == "Joint differs from a route, 99"),
        "{rendered}"
    );
}
