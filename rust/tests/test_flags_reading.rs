//! Pins for CK-43 and CK-44, the flags file and
//! reading.
//!
//! The flags pins build their collections by hand,
//! so each one fails under the negation of the rule it
//! names, the sections standing per file verdict state
//! in the order CK-8 states the judgements and standing
//! whether populated or not, a file whose flags carry
//! no recorded verdict standing in no section while
//! its flags count in the closing flags read count,
//! every flag of a sectioned file standing as one flag
//! line in the form of CK-20 whether it carries a
//! recorded verdict or carries none, and the closing
//! counts in their stated relations to the sections
//! and to the census, files, and states readings over
//! the same collection. The loader pins name each
//! defect of the file in the forms CK-12 states, one
//! pin per defect. The fencing pins hold the rendering
//! to the one line rule at the library surface, the
//! three handed fields of the flag line, the file name,
//! the pattern, and the set label, each printing
//! through the one helper, the reading standing born
//! inside the fencing programme. Every pin runs
//! offline on recorded and constructed entries, and no
//! pin makes a live call.
//!
//! The discrimination split of this file stands as
//! follows, pin by pin. Every pin is compile absent at
//! base, against the tree Round 20 closed on, by the
//! absence of the module it names, and within the slice
//! each pin discriminates against the wrong build its
//! own comment names. The golden pin catches the
//! section order mutation MA, the shipped sections
//! standing in the order CK-8 states the judgements and
//! not in alphabetical order. It passes under the first
//! set pooling mutation MB and the per set seating
//! mutation MC by design, the shipped collection
//! standing no file in more than one set, under the
//! stored count mutation MF by design, the stored and
//! computed counts agreeing there at eighteen, and
//! under the seating guard mutation MG by design, the
//! shipped collection's three verdictless files holding
//! no flags, so seating them prints no line the golden
//! does not state. The census binding pin discriminates
//! by content on the counts it reads against the census
//! over the same collection. The files binding pin
//! catches a build that gathers a file's flags by a
//! second path or drops a verdictless flag, the
//! verdictless cache.py line 19 standing among its
//! lines. The states binding pin catches a build that
//! seats a file under a state the states reading does
//! not state for it, or that counts a section's lines
//! against flags the files reading does not name. The
//! sums pin catches MG on its hand built collection
//! carrying a verdictless flagged file, the seated
//! file's lines raising the section sum to the closing
//! count the pin holds apart, and it passes under MF,
//! its sums reading the sections and not the stored
//! closing. The one file pin states the grain
//! divergence whole, the standing file's verdictless
//! flag standing as a line where the states reading
//! states no group for it. The empty collection pin
//! passes under MG by design, the empty collection
//! carrying no file the deleted guard could seat. The
//! cross set order pin catches MC in its pure form, its
//! fixture standing judged stickler on its first set's
//! flags alone and judged clean on its second set's
//! alone, so per set seating splits its lines across
//! two sections while the pooled state seats both in
//! one. It passes under MB, its file standing judged
//! stickler in its first standing, which is the state
//! the pool reads. The verdictless flag pin fails on a
//! build that stands only verdict carrying flags as
//! lines. The no section pin catches MG in pure form,
//! the verdictless file seating in the unjudged section
//! with all its flag lines. The section order pin
//! catches MA in its pure form. The contested pin fails
//! on a build that counts a flag holding both labels
//! with an abstaining side as contested or drops a
//! contested flag from its section count. The two
//! precedence pins fail on a majority derivation of the
//! state, the first standing judged stickler at one
//! stickler outcome against two clean and the second
//! standing judged clean on one clean outcome over a
//! flag carrying only abstentions. The pooling pin
//! catches MB and MC in pure form, its file reading
//! judged clean on its first set's flags alone and
//! judged stickler over the pool of the whole
//! collection. The states collection pin carries the
//! cross binding figures in full, ten and two, six and
//! none, two and none, and binds each section's lines
//! to the files reading under the states reading's
//! seating, so a build that misseats a file or drops a
//! verdictless line fails it over the constructed
//! collection as over the shipped one. The loader pins
//! each fail on a loader that accepts the defect they
//! name, and the whole or nothing pin catches the
//! dropped whole or nothing mutation MD in pure form
//! alone, its defective entry standing at index 1
//! behind a sound first entry, and the other loader
//! pins pass under MD, their defects standing at the
//! head of the value or in the first entry. The fencing
//! pins each fail on a raw print in their position, the
//! file name pin catching the dropped helper mutation
//! ME in pure form at its position and the pattern and
//! set label pins passing under ME, their positions
//! keeping the helper. The closing counts pin catches
//! MF, under which the closing flags read count prints
//! from a stored count rather than the count the
//! builder computes over the whole collection, its
//! hand built collection carrying a verdictless flagged
//! file the sections do not state. No eighth mutation
//! stands against this file, the round introducing no
//! new derivation, its state being CK-41's and its
//! contested predicate being CK-9's.

use std::path::{Path, PathBuf};

use stickler::census::census_of;
use stickler::cli::cli_main;
use stickler::files::files_reading_of;
use stickler::flags::{flags_reading_of, load_flags, FlagsLine, FlagsReading, FlagsSection};
use stickler::json::parse;
use stickler::render::render_flags;
use stickler::states::states_reading_of;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples() -> PathBuf {
    root().join("examples")
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "stickler-flags-{}-{}-{}",
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

fn run_flags(dir: &Path, text: &str) -> (i32, String) {
    let path = write(dir, "flags.json", text);
    let (code, out, _err) = run(&["flags", &path]);
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
/// CK-43 and CK-12 share.
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

/// The flags read counts of a flags rendering in standing
/// order, the three section closing counts and the
/// closing flags read count standing last.
fn flags_read_counts(text: &str) -> Vec<usize> {
    text.lines()
        .filter(|line| line.starts_with("Flags read, "))
        .map(|line| line.strip_prefix("Flags read, ").unwrap().parse().unwrap())
        .collect()
}

#[test]
fn the_example_matches_its_golden_through_the_library() {
    let text = std::fs::read_to_string(examples().join("flags.json")).unwrap();
    let value = parse(&text).unwrap();
    assert_eq!(load_flags(&value), Vec::<String>::new());
    let golden = std::fs::read_to_string(examples().join("flags.txt")).unwrap();
    assert_eq!(render_flags(&flags_reading_of(&value)), golden);
}

/// The shipped collection, read whole. The closings of
/// the flags reading stand in their relations to the
/// census over the same collection, the sets read count
/// the count of the sets the census reads and the flags
/// read count the census total block's flagged lines,
/// read from the builder's counts and from the rendered
/// text alike.
#[test]
fn the_closings_bind_to_the_census_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("flags.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    let census = census_of(&value);
    assert_eq!(reading.sets_read, census.sets.len());
    assert_eq!(reading.sets_read, 3);
    assert_eq!(reading.flags_read, census.total.borderline);
    assert_eq!(reading.flags_read, 18);
    let rendered = render_flags(&reading);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), census.sets.len());
    assert_eq!(
        count_after(closings, "Flags read, "),
        census.total.borderline
    );
    assert_eq!(count_after(closings, "Files read, "), 5);
    assert_eq!(count_after(closings, "Contested flags, "), 2);
}

/// The shipped collection, read whole. The flag lines of
/// each section are exactly the flags the files reading
/// names under the files standing in that state, in the
/// same collection order, the lines of the files
/// reading's sections for those files standing here
/// under one head, both readings and the states reading
/// run here, so a build that gathers a file's flags by
/// a second path fails this pin wherever that path and
/// the files reading disagree, and a build that passes
/// a verdictless flag over fails it on the line count
/// of its file, cache.py standing four lines here
/// against the three groups of the states reading.
#[test]
fn the_flag_lines_bind_to_the_files_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("flags.json")).unwrap();
    let value = parse(&text).unwrap();
    let flags = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let states = states_reading_of(&value);
    for (flags_section, states_section) in flags.sections.iter().zip(states.sections.iter()) {
        assert_eq!(flags_section.state, states_section.state);
        let stated_files: Vec<&str> = states_section
            .files
            .iter()
            .map(|group| group.file.as_str())
            .collect();
        let mut expected: Vec<FlagsLine> = Vec::new();
        for files_section in &files.sections {
            if stated_files.contains(&files_section.file.as_str()) {
                for flag in &files_section.flags {
                    expected.push(FlagsLine {
                        file: files_section.file.clone(),
                        line: flag.line,
                        pattern: flag.pattern.clone(),
                        set_label: flag.set_label.clone(),
                        contested: flag.contested,
                    });
                }
            }
        }
        assert_eq!(
            flags_section.lines, expected,
            "state {}",
            flags_section.state
        );
    }
}

/// The shipped collection, read whole. Each section
/// stands the files the states reading seats in its
/// state, no file standing in two sections and no
/// sectioned file missing, and each section's line
/// count is the sum over its files of the section flags
/// read counts of the files reading, both readings and
/// the states reading run here.
#[test]
fn the_sections_bind_to_the_states_reading_over_the_same_collection() {
    let text = std::fs::read_to_string(examples().join("flags.json")).unwrap();
    let value = parse(&text).unwrap();
    let flags = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let states = states_reading_of(&value);
    let mut seen: Vec<&str> = Vec::new();
    for (flags_section, states_section) in flags.sections.iter().zip(states.sections.iter()) {
        let mut section_files: Vec<&str> = Vec::new();
        for line in &flags_section.lines {
            if !section_files.contains(&line.file.as_str()) {
                section_files.push(line.file.as_str());
            }
        }
        let mut stated: Vec<&str> = states_section
            .files
            .iter()
            .map(|group| group.file.as_str())
            .collect();
        let mut seated = section_files.clone();
        stated.sort();
        seated.sort();
        assert_eq!(seated, stated, "state {}", flags_section.state);
        for file in &section_files {
            assert!(!seen.contains(file), "file {file} stands in two sections");
            seen.push(file);
        }
        let expected_lines: usize = section_files
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
    assert_eq!(seen.len(), 5);
}

/// The section flags read counts of the shipped
/// rendering sum to the count of the flags standing on
/// files carrying a state, computed with the builder
/// built counts, and the closing flags read count
/// counts the flagged lines of the whole collection,
/// the two standing apart over a hand built collection
/// carrying a verdictless flagged file, whose flags the
/// closing count counts and no section states. It
/// catches the seating guard mutation MG on the hand
/// built collection, the seated file's lines raising
/// the section sum to the closing count.
#[test]
fn the_section_counts_sum_and_the_closing_flags_read_counts_the_whole_collection() {
    let text = std::fs::read_to_string(examples().join("flags.json")).unwrap();
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    let rendered = render_flags(&reading);
    let counts = flags_read_counts(&rendered);
    assert_eq!(counts, vec![18, 0, 0, 18]);
    assert_eq!(counts[..3].iter().sum::<usize>(), counts[3]);
    assert_eq!(counts[3], reading.flags_read);
    let hand = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            ),
            entry_json("ghost.py", FLAGS_DIFF, ""),
        ]),
    )]);
    let hand_value = parse(&hand).unwrap();
    let hand_reading = flags_reading_of(&hand_value);
    let section_sum: usize = hand_reading
        .sections
        .iter()
        .map(|section| section.lines.len())
        .sum();
    assert_eq!(section_sum, 1);
    assert_eq!(hand_reading.flags_read, 3);
    assert!(section_sum < hand_reading.flags_read);
}

/// A constructed one set collection whose verdicts
/// stand only on one file's flags stands that file's
/// flag lines alone, in the judged clean section its
/// one clean choice earns it, every flag of the file
/// standing as a line whether it carries a verdict or
/// carries none, the other file of the collection,
/// carrying flags with no verdicts, standing in no
/// section while the closing flags read count counts
/// its flag. The verdictless flag of the standing file
/// stands as a flag line here, where the states
/// reading states no flag group for it.
#[test]
fn a_collection_with_verdicts_on_one_file_stands_that_files_flag_lines_alone() {
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
    assert_eq!(load_flags(&value), Vec::<String>::new());
    assert_eq!(
        render_flags(&flags_reading_of(&value)),
        "Flags\n\
         \n\
         Judged stickler\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Judged clean\n\
         app.py, line 4, near placeholder name, set, one set\n\
         app.py, line 5, broad handler, set, one set\n\
         Flags read, 2\n\
         Contested, 0\n\
         \n\
         Unjudged\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Sets read, 1\n\
         Files read, 1\n\
         Flags read, 3\n\
         Contested flags, 0\n"
    );
}

/// An empty collection renders the head line, the
/// three sections each stating the no flagged lines
/// line and closing at zero, and the closing counts
/// at zero.
#[test]
fn an_empty_collection_renders_three_sections_each_stating_no_flagged_lines_and_closes_at_zero() {
    let value = parse("{\"sets\": []}").unwrap();
    assert_eq!(load_flags(&value), Vec::<String>::new());
    assert_eq!(
        render_flags(&flags_reading_of(&value)),
        "Flags\n\
         \n\
         Judged stickler\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Judged clean\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Unjudged\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Sets read, 0\n\
         Files read, 0\n\
         Flags read, 0\n\
         Contested flags, 0\n"
    );
}

/// Over a constructed two set collection where one
/// file stands in both sets, judged stickler on its
/// first set's flags alone and judged clean on its
/// second set's alone, the file stands once, in the
/// judged stickler section the pool reads, with its
/// flag lines in collection order, the first set's
/// line before the second set's. It catches the per
/// set seating mutation MC in its pure form, under
/// which the second set's line seats in the judged
/// clean section and the file stands in two sections.
#[test]
fn a_file_standing_in_two_sets_stands_once_with_its_flag_lines_in_collection_order() {
    let text = collection_json(&[
        (
            "first set",
            change_set_json(&[entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            )]),
        ),
        (
            "second set",
            change_set_json(&[entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.8}]",
            )]),
        ),
    ]);
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    assert_eq!(reading.sections[0].lines.len(), 2);
    assert!(reading.sections[1].lines.is_empty());
    assert!(reading.sections[2].lines.is_empty());
    assert_eq!(reading.sections[0].lines[0].set_label, "first set");
    assert_eq!(reading.sections[0].lines[1].set_label, "second set");
    assert_eq!(
        render_flags(&reading),
        "Flags\n\
         \n\
         Judged stickler\n\
         app.py, line 4, near placeholder name, set, first set\n\
         app.py, line 4, near placeholder name, set, second set\n\
         Flags read, 2\n\
         Contested, 0\n\
         \n\
         Judged clean\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Unjudged\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Sets read, 2\n\
         Files read, 1\n\
         Flags read, 2\n\
         Contested flags, 0\n"
    );
}

/// A constructed flag carrying no verdicts stands as
/// a flag line in its file's section, the file
/// carrying a stickler choice on its other flag, so a
/// build that stands only verdict carrying flags as
/// lines drops it and fails here.
#[test]
fn a_flag_carrying_no_verdicts_stands_as_a_flag_line_in_its_files_section() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            FLAGS_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let rendered = render_flags(&flags_reading_of(&value));
    assert_eq!(
        rendered,
        "Flags\n\
         \n\
         Judged stickler\n\
         app.py, line 4, near placeholder name, set, one set\n\
         app.py, line 5, broad handler, set, one set\n\
         Flags read, 2\n\
         Contested, 0\n\
         \n\
         Judged clean\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Unjudged\n\
         No flagged lines\n\
         Flags read, 0\n\
         Contested, 0\n\
         \n\
         Sets read, 1\n\
         Files read, 1\n\
         Flags read, 2\n\
         Contested flags, 0\n"
    );
}

/// A constructed file carrying flags and no recorded
/// verdicts anywhere stands in no section and none of
/// its flag lines stand, while the closing flags read
/// count counts its flags. It catches the seating
/// guard mutation MG in pure form, under which the
/// file seats in the unjudged section with all its
/// flag lines.
#[test]
fn a_file_carrying_flags_and_no_verdicts_stands_in_no_section() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            ),
            entry_json("ghost.py", FLAGS_DIFF, ""),
        ]),
    )]);
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    assert!(reading
        .sections
        .iter()
        .flat_map(|section| section.lines.iter())
        .all(|line| line.file != "ghost.py"));
    let rendered = render_flags(&reading);
    assert!(!rendered.contains("ghost.py"), "{rendered}");
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Files read, "), 1);
    assert_eq!(count_after(closings, "Flags read, "), 3);
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
    let rendered = render_flags(&flags_reading_of(&value));
    let position = |needle: &str| rendered.lines().position(|line| line == needle).unwrap();
    let stickler = position("Judged stickler");
    let clean = position("Judged clean");
    let unjudged = position("Unjudged");
    assert!(stickler < clean && clean < unjudged, "{rendered}");
    let harvest = position("harvest.py, line 4, near placeholder name, set, one set");
    let windows = position("windows.py, line 4, near placeholder name, set, one set");
    let cache = position("cache.py, line 4, near placeholder name, set, one set");
    assert!(stickler < harvest && harvest < clean, "{rendered}");
    assert!(clean < windows && windows < unjudged, "{rendered}");
    assert!(unjudged < cache, "{rendered}");
}

/// A constructed flag holding both labels without
/// abstaining stands contested and counts in its
/// section and in the closing count, and a flag
/// holding both labels with an abstaining side stands
/// not contested and counts in neither, under CK-9.
#[test]
fn a_contested_flag_counts_in_its_section_and_a_flag_holding_both_labels_with_an_abstaining_side_does_not(
) {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            FLAGS_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.3}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    assert_eq!(reading.sections[0].lines.len(), 2);
    assert!(reading.sections[0].lines[0].contested);
    assert!(!reading.sections[0].lines[1].contested);
    let rendered = render_flags(&reading);
    let section = rendered.split("\n\n").nth(1).unwrap();
    assert_eq!(count_after(section, "Contested, "), 1);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Contested flags, "), 1);
}

/// A constructed file carrying one stickler choice on
/// one flag and only clean choices on another seats
/// all its flag lines in the judged stickler section,
/// in the precedence of CK-41, though its second flag
/// stands judged clean under CK-8.
#[test]
fn a_file_carrying_one_stickler_choice_seats_all_its_flag_lines_in_the_judged_stickler_section() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[entry_json(
            "app.py",
            FLAGS_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.9},{\"line\":5,\"pattern\":\"broad handler\",\"model\":\"openai/gpt-6-luna\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    )]);
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    assert_eq!(reading.sections[0].state, "stickler");
    assert_eq!(reading.sections[0].lines.len(), 2);
    assert!(reading.sections[1].lines.is_empty());
    assert!(reading.sections[2].lines.is_empty());
}

/// A constructed file carrying no stickler choice and
/// one clean choice seats all its flag lines in the
/// judged clean section, in the precedence of CK-41,
/// though another of its flags carries only
/// abstentions and so stands unjudged under CK-8.
#[test]
fn a_file_carrying_no_stickler_choice_and_one_clean_choice_seats_its_flag_lines_in_the_judged_clean_section(
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
    let reading = flags_reading_of(&value);
    assert_eq!(reading.sections[1].state, "clean");
    assert_eq!(reading.sections[1].lines.len(), 2);
    assert!(reading.sections[0].lines.is_empty());
    assert!(reading.sections[2].lines.is_empty());
}

/// Over a constructed two set collection, a file
/// reading judged clean on its first set's flags alone
/// stands judged stickler over the pool of the whole
/// collection once its second set carries a stickler
/// choice, and all its flag lines stand in the judged
/// stickler section, the state pooling every recorded
/// verdict standing on the file's flags across the
/// collection under CK-41. It catches the first set
/// pooling mutation MB and the per set seating mutation
/// MC in pure form.
#[test]
fn a_file_pools_its_verdicts_across_sets_for_the_section_it_stands_in() {
    let first = (
        "first set",
        change_set_json(&[entry_json(
            "app.py",
            ONE_FLAG_DIFF,
            "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"clean\",\"confidence\":0.8}]",
        )]),
    );
    let alone = parse(&collection_json(std::slice::from_ref(&first))).unwrap();
    let alone_reading = flags_reading_of(&alone);
    assert_eq!(alone_reading.sections[1].lines.len(), 1);
    assert_eq!(alone_reading.sections[1].lines[0].file, "app.py");
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
    let reading = flags_reading_of(&value);
    assert_eq!(reading.sections[0].lines.len(), 2);
    assert!(reading.sections[1].lines.is_empty());
    assert!(reading.sections[2].lines.is_empty());
}

/// The constructed states collection, read whole. The
/// flags reading over it stands at ten flag lines
/// contested two in the judged stickler section, six
/// contested none in the judged clean section, and two
/// contested none in the unjudged section, closing at
/// two sets read, six files read, eighteen flags read,
/// and two contested flags, and each section's lines
/// are exactly the flags the files reading names under
/// the files the states reading seats in that state,
/// all three readings run here. The two verdictless
/// flags of the collection, timber.py set 1 line 6 and
/// saddle.py set 2 line 5, stand as flag lines here,
/// where the states reading states no flag group for
/// either.
#[test]
fn the_flags_over_the_states_collection_bind_to_the_states_and_files_readings() {
    let text = std::fs::read_to_string(examples().join("states_collection.json")).unwrap();
    let value = parse(&text).unwrap();
    let flags = flags_reading_of(&value);
    let files = files_reading_of(&value);
    let states = states_reading_of(&value);
    let line_counts: Vec<usize> = flags
        .sections
        .iter()
        .map(|section| section.lines.len())
        .collect();
    assert_eq!(line_counts, vec![10, 6, 2]);
    let contested_counts: Vec<usize> = flags
        .sections
        .iter()
        .map(|section| section.lines.iter().filter(|line| line.contested).count())
        .collect();
    assert_eq!(contested_counts, vec![2, 0, 0]);
    assert_eq!(flags.sets_read, 2);
    assert_eq!(flags.flags_read, 18);
    let rendered = render_flags(&flags);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), 2);
    assert_eq!(count_after(closings, "Files read, "), 6);
    assert_eq!(count_after(closings, "Flags read, "), 18);
    assert_eq!(count_after(closings, "Contested flags, "), 2);
    for (flags_section, states_section) in flags.sections.iter().zip(states.sections.iter()) {
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
    }
    assert!(rendered
        .lines()
        .any(|line| line
            == "timber.py, line 6, near placeholder name, set, constructed states set 1"));
    assert!(rendered
        .lines()
        .any(|line| line
            == "saddle.py, line 5, near placeholder name, set, constructed states set 2"));
}

#[test]
fn the_loader_refuses_a_file_that_is_not_an_object() {
    let value = parse("[]").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["flags must be a JSON object".to_string()]
    );
}

#[test]
fn the_loader_refuses_a_file_missing_its_sets() {
    let value = parse("{}").unwrap();
    assert_eq!(load_flags(&value), vec!["sets is missing".to_string()]);
}

#[test]
fn the_loader_refuses_a_sets_value_that_is_not_a_list() {
    let value = parse("{\"sets\": {}}").unwrap();
    assert_eq!(load_flags(&value), vec!["sets must be a list".to_string()]);
}

#[test]
fn the_loader_refuses_an_entry_that_is_not_an_object() {
    let value = parse("{\"sets\": [42]}").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0] must be an object".to_string()]
    );
}

#[test]
fn an_entry_defect_names_its_position() {
    let value = parse("{\"sets\": [{\"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0].label is missing".to_string()]
    );
}

#[test]
fn the_loader_refuses_an_entry_with_an_empty_label() {
    let value = parse("{\"sets\": [{\"label\": \"\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0].label must be a non empty string".to_string()]
    );
}

#[test]
fn the_loader_refuses_a_label_holding_a_line_break() {
    let text = collection_json(&[("evil\nlabel", change_set_json(&[]))]);
    let value = parse(&text).unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0].label holds a line break".to_string()]
    );
}

#[test]
fn the_loader_refuses_a_label_holding_an_unpaired_surrogate() {
    let value =
        parse("{\"sets\": [{\"label\": \"\\ud800\", \"change_set\": {\"files\": []}}]}").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0].label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn the_loader_refuses_an_entry_missing_its_change_set() {
    let value = parse("{\"sets\": [{\"label\": \"one\"}]}").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0].change_set is missing".to_string()]
    );
}

#[test]
fn a_member_defect_renders_in_the_census_path_form() {
    let value = parse("{\"sets\": [{\"label\": \"one\", \"change_set\": {}}]}").unwrap();
    assert_eq!(
        load_flags(&value),
        vec!["sets[0].change_set.files is missing".to_string()]
    );
}

/// A defect in the second entry of a collection, at
/// index 1 behind a sound first entry, renders the
/// defect list and no reading, whole or nothing, so
/// no partial flags reading ever prints. It catches
/// the dropped whole or nothing mutation MD in pure
/// form alone.
#[test]
fn a_defective_collection_renders_its_defects_and_no_reading() {
    let dir = temp_dir("defects");
    let text = format!(
        "{{\"sets\": [{{\"label\": \"sound set\", \"change_set\": {}}}, \
         {{\"change_set\": {{\"files\": []}}}}]}}",
        change_set_json(&[entry_json("app.py", ONE_FLAG_DIFF, "")])
    );
    let (code, out) = run_flags(&dir, &text);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "Flags\n\
         Verdict, invalid\n\
         Defects, 1\n\
         Defect, sets[1].label is missing\n"
    );
}

/// Build one flag line by hand in the family's form.
fn handed_line(file: &str, pattern: &str, set_label: &str) -> FlagsLine {
    FlagsLine {
        file: file.to_string(),
        line: 4,
        pattern: pattern.to_string(),
        set_label: set_label.to_string(),
        contested: false,
    }
}

/// Build one section by hand in the family's form.
fn handed_section(state: &'static str, lines: Vec<FlagsLine>) -> FlagsSection {
    FlagsSection { state, lines }
}

/// Build a reading by hand around one handed line in
/// the judged stickler section.
fn handed_reading(line: FlagsLine) -> FlagsReading {
    FlagsReading {
        sections: vec![
            handed_section("stickler", vec![line]),
            handed_section("clean", vec![]),
            handed_section("unjudged", vec![]),
        ],
        sets_read: 1,
        flags_read: 1,
    }
}

/// A file name holding a break and a flags read line's
/// own text prints as the empty field in the flag line,
/// so no closing line is forged. It catches the dropped
/// helper mutation ME in pure form at this position.
#[test]
fn a_line_break_in_a_flag_lines_file_name_prints_as_the_empty_field() {
    let reading = handed_reading(handed_line(
        "evil.py\nFlags read, 99",
        "near placeholder name",
        "one set",
    ));
    let rendered = render_flags(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == ", line 4, near placeholder name, set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Flags read, 99"),
        "{rendered}"
    );
    assert!(!rendered.contains("evil.py"), "{rendered}");
}

/// A pattern holding a break and a flags read line's
/// own text prints as the empty field in the flag line,
/// so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_lines_pattern_prints_as_the_empty_field() {
    let reading = handed_reading(handed_line(
        "app.py",
        "near placeholder name\nFlags read, 99",
        "one set",
    ));
    let rendered = render_flags(&reading);
    assert!(
        rendered
            .lines()
            .any(|line| line == "app.py, line 4, , set, one set"),
        "{rendered}"
    );
    assert!(
        !rendered.lines().any(|line| line == "Flags read, 99"),
        "{rendered}"
    );
}

/// A set label holding a break and a sets read line's
/// own text prints as the empty field in the flag line,
/// so no closing line is forged.
#[test]
fn a_line_break_in_a_flag_lines_set_label_prints_as_the_empty_field() {
    let reading = handed_reading(handed_line(
        "app.py",
        "near placeholder name",
        "one set\nSets read, 99",
    ));
    let rendered = render_flags(&reading);
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

/// The closing counts print as the counts they state,
/// the closing flags read count standing as the count
/// the builder computes over the whole collection,
/// counting the flags of a file carrying no verdicts
/// that no section states, and not as the sum of the
/// section counts. It catches the stored count
/// mutation MF, under which a stored count standing
/// in place of the builder's count prints.
#[test]
fn the_closing_counts_print_as_the_counts_they_state() {
    let text = collection_json(&[(
        "one set",
        change_set_json(&[
            entry_json(
                "app.py",
                ONE_FLAG_DIFF,
                "[{\"line\":4,\"pattern\":\"near placeholder name\",\"model\":\"jev-1.13.0\",\"choice\":\"stickler\",\"confidence\":0.9}]",
            ),
            entry_json("ghost.py", FLAGS_DIFF, ""),
        ]),
    )]);
    let value = parse(&text).unwrap();
    let reading = flags_reading_of(&value);
    assert_eq!(reading.flags_read, 3);
    let rendered = render_flags(&reading);
    assert_eq!(flags_read_counts(&rendered), vec![1, 0, 0, 3]);
    let closings = rendered.split("\n\n").last().unwrap();
    assert_eq!(count_after(closings, "Sets read, "), 1);
    assert_eq!(count_after(closings, "Files read, "), 1);
    assert_eq!(count_after(closings, "Flags read, "), 3);
    assert_eq!(count_after(closings, "Contested flags, "), 0);
}
