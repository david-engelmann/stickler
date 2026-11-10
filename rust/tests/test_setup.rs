//! Setup pins. Each test can fail for a real reason against a
//! repo that lost a setup file or let its status drift from
//! its contract. Translated from tests/test_setup.py, with the
//! pins the port adds for its own status line, contract
//! section, and CI step.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative)).unwrap()
}

#[test]
fn agents_carries_the_canonical_marker() {
    let text = read("AGENTS.md");
    assert!(text.contains("<!-- repo annex below this line -->"));
    assert!(text.contains("39 repo storefront perfection program"));
}

#[test]
fn readme_states_the_closed_round_count() {
    let text = read("README.md");
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn contract_names_the_borderline_test_file_for_ck_7() {
    let text = read("CONTRACT.md");
    assert!(text.contains("Test, tests/test_borderline.py and tests/test_verdicts.py"));
}

#[test]
fn contract_carries_the_setup_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "CK-1"] {
        assert!(text.contains(oid), "obligation {oid}");
    }
}

#[test]
fn contract_carries_the_formats_section() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## The formats, version 1"));
    assert!(text.contains("FIX-CK-24"));
    for form in [
        "The review form stands at version 1",
        "The change set form stands at version 1",
        "The collection form stands at version 1",
        "The verdict envelope forms stand at version 1",
        "The run record form stands at version 1",
    ] {
        assert!(text.contains(form), "form {form}");
    }
    assert!(text.contains("answers.decision"));
    assert!(text.contains("message.content"));
    assert!(text.contains(
        "The borderline lines of a change set and the judgement standing on each are computed under CK-7 and CK-8"
    ));
}

#[test]
fn readme_reports_the_contract_status() {
    let text = read("README.md");
    let expected = "- Contract, SETUP-1 to SETUP-4 and CK-1 to CK-6 met. \
        CK-1 to CK-4 stand in full form in CONTRACT.md with the Round 1 \
        repair amendments, and Round 1 closed on the merged slice 2 after \
        three reviews, a repair pass, and an approving re-review.";
    assert!(
        text.lines().any(|line| line == expected),
        "the Round 1 status line stands"
    );
}

#[test]
fn readme_reports_the_round_3_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-7 and CK-8 met. They stand in full form in \
        CONTRACT.md with the Round 3 repair amendments, and Round 3 \
        closed on the merged slice 4 after three reviews, a repair \
        pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 4 has landed. It carries the borderline \
        flagging with its three patterns, the verdict seat reading \
        both decision model routes, the recorded verdict path \
        through the set review, and the recorded borderline pilot \
        under examples, with the suite running offline on the \
        recorded envelopes.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(!text.contains("Slice 4 is built and in review"));
}

#[test]
fn readme_reports_the_round_4_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-9 and CK-10 met. They stand in full form in \
        CONTRACT.md with the Round 4 repair amendments, and Round 4 \
        closed on the merged slice 5 after three reviews, a repair \
        pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 5 has landed. It carries the contested \
        line reading of the set review, with the contested section \
        before the summary and the contested count in the summary, \
        and the constructed contested example under examples, with \
        the suite running offline on the recorded pilot and the \
        constructed example.";
    assert!(text.lines().any(|line| line == slice_line));
    // Adjusted with the Round 5 slice, as the named mechanical
    // consequence of slice 6 standing in review. The count of
    // slices standing in review was one while Round 5 was
    // built, until the Round 5 closure landed slice 6 and the
    // count returned to zero. The Round 5 pin below holds the
    // standing line and the slice line themselves. Adjusted
    // again with the Round 6 slice, as the named mechanical
    // consequence of slice 7 standing in review, so the count
    // stood at one while Round 6 was built, until the Round 6
    // closure landed slice 7 and the count returned to zero.
    // Adjusted again with the Round 7 slice, as the named
    // mechanical consequence of slice 8 standing in review,
    // so the count stood at one while Round 7 was built,
    // until the Round 7 closure landed slice 8 and the
    // count returned to zero. Adjusted again with the
    // Round 8 slice, as the named mechanical consequence
    // of slice 9 standing in review, so the count stood
    // at one while Round 8 was built, until the Round 8
    // closure landed slice 9 and the count returned to zero.
    // Adjusted again with the Round 9 slice, as the named
    // mechanical consequence of slice 10 standing in
    // review, so the count stood at one while Round 9
    // was built, until the Round 9 closure landed slice
    // 10 and the count returned to zero. Adjusted again
    // with the Round 10 slice, as the named mechanical
    // consequence of slice 11 standing in review, so the
    // count stood at one while Round 10 was built, until
    // the Round 10 closure landed slice 11 and the count
    // returned to zero. Adjusted again with the Round 11
    // slice, as the named mechanical consequence of
    // slice 12 standing in review, so the count stood
    // at one while Round 11 was built, until the Round 11
    // closure landed slice 12 and the count returned to
    // zero. Adjusted again with the Round 12 slice, as
    // the named mechanical consequence of slice 13
    // standing in review, so the count stood at one
    // while Round 12 was built, until the Round 12
    // closure landed slice 13 and the count returned to
    // zero. Adjusted again with the Round 13 slice, as
    // the named mechanical consequence of slice 14
    // standing in review, so the count stood at one
    // while Round 13 was built, until the Round 13
    // closure landed slice 14 and the count returned to
    // zero. Adjusted again with the Round 14 slice, as
    // the named mechanical consequence of slice 15
    // standing in review, so the count stood at one
    // while Round 14 was built, until the Round 14
    // closure landed slice 15 and the count returned to
    // zero. Adjusted again with the Round 15 slice, as
    // the named mechanical consequence of slice 16
    // standing in review, so the count stood at one
    // while Round 15 was built, until the Round 15
    // closure landed slice 16 and the count returned to
    // zero.
    assert_eq!(text.matches("built and in review").count(), 0);
}

#[test]
fn readme_reports_the_round_5_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-11 and CK-12 met. They stand in full form in \
        CONTRACT.md, and Round 5 closed on the merged slice 6 after \
        three reviews, a repair pass, and an approving owner gate.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 6 has landed. It carries the \
        judging census over a stated collection of recorded review \
        inputs, with the census command beside check, render, and \
        review, and the census example under examples standing over \
        the Round 2 set, the pilot set, and the constructed Round 4 \
        example named as constructed, with the suite running offline \
        on the recorded and constructed sets.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_6_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-13 and CK-14 met. They stand in full form in \
        CONTRACT.md with the Round 6 repair amendment, and Round 6 \
        closed on the merged slice 7 after three reviews, a repair \
        pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 7 has landed. It carries the \
        rejudging run over the pilot flags through the verdict seat's \
        Jev route, recorded as the rejudged set under examples, and \
        the rejudged ledger reading over a stated pair of recorded \
        sets, with the rejudged command beside check, render, review, \
        and census, with the suite running offline on the recorded \
        envelopes and sets.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_7_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-15 and CK-16 met. They stand in full form in \
        CONTRACT.md, and Round 7 closed on the merged slice 8 after three \
        reviews, a repair pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 8 has landed. It carries the split \
        reading over one recorded set, stating the judgement of each of \
        its flags under the first named route alone, under the second \
        named route alone, and under both routes jointly, with the split \
        example over the pilot set under examples, and the split command \
        beside check, render, review, census, and rejudged, with the \
        suite running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_8_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-17 and CK-18 met. They stand in full form in \
        CONTRACT.md with the Round 8 repair amendments, and Round 8 \
        closed on the merged slice 9 after three reviews, a repair \
        pass, and an approving re-review.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 9 has landed. It carries the \
        contested reading over a recorded collection of change sets, one \
        section per set in collection order, naming each set's contested \
        flags with file, line, and pattern, or the no contest line where \
        a set holds none, with per set flags read and contested counts \
        and closing counts over the whole collection, with the contested \
        example under examples standing over the same collection as the \
        census example, and the contested command beside check, render, \
        review, census, rejudged, and split, with the suite running \
        offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_9_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-19 and CK-20 met. They stand in full form in \
        CONTRACT.md with the FIX-CK-21 fencing closure, and Round 9 \
        closed on the merged slice 10 after three reviews approving \
        with no findings, with the red team's one observation on the \
        single review rendering carried to Round 10 as a fencing \
        component.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 10 has landed. It carries the \
        pattern reading over a recorded collection of change sets, one \
        section per borderline pattern in the order the contract states them, \
        naming each flag of the collection standing under that pattern \
        with file, line, and the set it stands in, with per section flags \
        read and contested counts and closing counts over the whole \
        collection, with the pattern example under examples standing \
        over the same collection as the census example, and the pattern \
        command beside check, render, review, census, rejudged, split, \
        and contested, and the library fencing closure over the census, \
        rejudged, split, and set review renderings, with the suite \
        running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_10_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-21 and CK-22 met. They stand in full form in \
        CONTRACT.md with the FIX-CK-22 fencing closure, and Round 10 \
        closed on the merged slice 11 after three reviews, with the \
        red team's one observation on the set review rendering \
        carried to a later round as a fencing candidate.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 11 has landed. It carries the \
        file reading over a recorded collection of change sets, one \
        section per file in first appearance order across the \
        collection, naming each flag standing on that file with line, \
        pattern, and the set it stands in, with per file flags read \
        and contested counts and closing counts over the whole \
        collection, with the constructed files example under examples \
        standing over two constructed sets sharing one file, and the \
        files command beside check, render, review, census, rejudged, \
        split, contested, and pattern, and the review rendering \
        fencing closure over the single review rendering, and the \
        README brought to the fleet README standard, with the suite \
        running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_11_standing() {
    let text = read("README.md");
    let standing = "- Contract, CK-23 and CK-24 met. They stand in full form in \
        CONTRACT.md with the FIX-CK-23 fencing closure, and Round 11 \
        closed on the merged slice 12 after three reviews approving, \
        with no repair pass.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 12 has landed. It carries the \
        borderline reading over a recorded collection of change sets, one \
        section per set holding at least one borderline flag in \
        collection order, naming each borderline flag of the set with \
        file, line, pattern, the flagged line as written, and the shape \
        reading the checker computed for it, with per section borderline \
        counts and closing counts over the whole collection, with the \
        borderline example under examples standing over the same \
        collection as the census example, and the borderline command \
        beside check, render, review, census, rejudged, split, \
        contested, pattern, and files, and the set review rendering \
        fencing closure over the judgement, flag, and verdict fields of \
        the set review rendering, with the suite running offline on the \
        recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn readme_reports_the_round_12_status() {
    let text = read("README.md");
    let standing = "- Contract, FIX-CK-24, FIX-CK-25, CK-25, and CK-26 met. \
        They stand in full form in CONTRACT.md with the FIX-CK-24, \
        FIX-CK-25, and FIX-CK-26 repair amendments, and Round 12 \
        closed on the merged slice 13 after three reviews, one \
        standards Minor and one red team Minor repaired.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 13 has landed. It carries the \
        formats of the repo stated at version 1 in the contract with the \
        consolidated formats section, the static outcome word fencing \
        in the split and rejudged renderings, the outcomes reading over \
        a recorded collection of change sets, one section per judgement \
        in the order the contract states them, naming each flag of the \
        collection standing under that judgement with file, line, \
        pattern, and the set it stands in, with per section flags read \
        counts and closing counts over the whole collection, with the \
        outcomes example under examples standing over the same \
        collection as the census example, and the outcomes command \
        beside check, render, review, census, rejudged, split, \
        contested, pattern, files, and borderline, and the constructed \
        large change set and large diff under examples, with the suite \
        running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

/// The Round 13 pins. The obligations pin fails against
/// the tree Round 12 closed on by the absence of every
/// text it reads, and the status pin fails on that
/// tree's README by the absence of the two lines it
/// holds, each line read whole as the standing pins
/// read theirs.
#[test]
fn contract_carries_the_round_13_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-27."));
    assert!(text.contains("- CK-28."));
    assert!(text.contains("the label FIX-CK-27 is not consumed"));
    let sets = read("rust/src/sets.rs");
    // Adjusted with the Round 15 slice, as the named mechanical
    // consequence of CK-31 joining the obligations the sets.rs
    // sentence names. The sentence gains CK-31, so the pinned
    // substring gains it too, in the form the Round 14 pin
    // below reads as well.
    assert!(sets.contains("CK-25, CK-27, CK-29, and CK-31 in CONTRACT.md"));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-27 that verdicts reads"));
}

/// The Round 14 obligations pin. It fails against the
/// tree Round 13 closed on by the absence of every text
/// it reads.
#[test]
fn contract_carries_the_round_14_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-29."));
    assert!(text.contains("- CK-30."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains("CK-25, CK-27, CK-29, and CK-31 in CONTRACT.md"));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-29 that shapes reads"));
}

/// The Round 15 obligations pin. It fails against the
/// tree Round 14 closed on by the absence of every
/// text it reads.
#[test]
fn contract_carries_the_round_15_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-31."));
    assert!(text.contains("- CK-32."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains("CK-25, CK-27, CK-29, and CK-31 in CONTRACT.md"));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-31 that choices reads"));
}

/// The doc list pin of the Round 14 slice, adjusted
/// with the Round 15 slice as the named mechanical
/// consequence of the choices loader standing as the
/// tenth collection loader under the same guard. The
/// module list gains choices and the count in the
/// name and this comment moves from nine to ten, and
/// the pin fails against the tree Round 14 closed on
/// by the tenth module's absence. Each of the ten
/// loader docs carries the one shared sentence
/// naming the shared set entry validation as the one
/// home every collection loader calls, and none of
/// the ten carries a caller enumeration in the
/// loaders both call or loaders all call forms. It
/// passes on any tree whose docs stand restated,
/// including a tree carrying the restatement without
/// the reading, which is correct, because the pin
/// guards the docs and the reading pins guard the
/// reading.
#[test]
fn the_ten_loader_docs_carry_the_one_shared_sentence() {
    for module in [
        "census",
        "rejudged",
        "contested",
        "pattern",
        "files",
        "borderline",
        "outcomes",
        "verdicts_reading",
        "shapes",
        "choices",
    ] {
        let text = read(&format!("rust/src/{module}.rs"));
        assert!(
            text.contains("the one home every collection loader calls"),
            "loader doc {module}"
        );
        assert!(
            !text.contains("loaders both call"),
            "loader doc {module} enumerates no callers"
        );
        assert!(
            !text.contains("loaders all call"),
            "loader doc {module} enumerates no callers"
        );
    }
}

/// The birth count pin of the Round 15 slice. Each of
/// the six reading heads, in files, borderline,
/// outcomes, verdicts_reading, shapes, and choices,
/// carries the one shared sentence naming the census
/// as the same recorded inputs over the same
/// collection loading whole, and none of the six
/// carries the loading whole into all form or an
/// enumeration of fellow readings in that sentence.
/// The doc text is read flattened, the comment
/// markers and line wraps removed, because the
/// sentence wraps across the head's lines. It fails
/// against the tree Round 14 closed on in all six
/// files, five by content, each carrying the count
/// form there, and the sixth by its module's
/// absence, and it passes on any tree whose heads
/// stand restated, including a tree carrying the
/// restatement without the reading, which is
/// correct, because the pin guards the heads and
/// the reading pins guard the reading.
#[test]
fn the_six_reading_heads_carry_the_one_shared_inputs_sentence() {
    for module in [
        "files",
        "borderline",
        "outcomes",
        "verdicts_reading",
        "shapes",
        "choices",
    ] {
        let text = read(&format!("rust/src/{module}.rs"));
        let flattened = text
            .replace("//!", " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            flattened.contains(
                "It exists for the same recorded inputs as the census, \
                 over the same collection loading whole."
            ),
            "reading head {module}"
        );
        assert!(
            !flattened.contains("loading whole into all"),
            "reading head {module} carries no birth count"
        );
    }
}

#[test]
fn readme_reports_the_round_13_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-27 and CK-28 met, with FIX-CK-28. \
        They stand in full form in CONTRACT.md with the FIX-CK-28 \
        repair amendment, and Round 13 closed on the merged slice 14 \
        after three reviews and three delta reviews on the repair, \
        one red team Minor repaired and two record figures corrected.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 14 has landed. It carries the \
        verdicts reading over a recorded collection of change sets, one \
        section per set in collection order, naming each flag of the \
        set carrying at least one recorded verdict with file, line, \
        pattern, and the set it stands in, the verdict pairs of the \
        flag in the standing set review form, and a closing count of \
        the pairs stated per section, with closing counts over the \
        whole collection, with the verdicts example under examples \
        standing over the same collection as the census example, and \
        the verdicts command beside check, render, review, census, \
        rejudged, split, contested, pattern, files, borderline, and \
        outcomes, and the constructed large verdicts collection under \
        examples, with the suite running offline on the recorded \
        verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

/// The Round 14 status pin, in the standing form. It
/// fails on the base README by content, the two lines
/// it holds standing absent there, and passes on the
/// slice README.
#[test]
fn readme_reports_the_round_14_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-29 and CK-30 met. They stand in full form in \
        CONTRACT.md, and Round 14 closed on the merged slice 15 \
        after three reviews, no Major on any axis and no repair \
        owed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 15 has landed. It carries the \
        shapes reading over a recorded collection of change sets, one \
        section per shape in the order the contract states them, naming \
        each judgement of the collection standing under that shape with \
        file, standing number, category, and the set it stands in, with \
        per section judgements read counts and closing counts over the \
        whole collection, with the shapes example under examples \
        standing over the same collection as the census example, and \
        the shapes command beside check, render, review, census, \
        rejudged, split, contested, pattern, files, borderline, \
        outcomes, and verdicts, and the loader doc list restatement \
        in the one drift free form across the nine loader docs, and \
        the constructed shapes collection under examples, with the \
        suite running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

/// The Round 15 status pin, in the standing form. It
/// fails on the base README by content, the two lines
/// it holds standing absent there, and passes on the
/// slice README.
#[test]
fn readme_reports_the_round_15_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-31 and CK-32 met. They stand in full form in \
        CONTRACT.md, and Round 15 closed on the merged slice 16 \
        after three reviews, no Major on any axis and no repair \
        owed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 16 has landed. It carries the \
        choices reading over a recorded collection of change sets, one \
        section per outcome in the order the contract states them, naming \
        each flag of the collection carrying at least one verdict \
        standing under that outcome with file, line, pattern, and the \
        set it stands in, the verdict pairs of the flag standing under \
        the section's outcome in the standing set review form, and a \
        closing count of the pairs stated per section, with closing \
        counts over the whole collection, with the choices example \
        under examples standing over the same collection as the census \
        example, and the choices command beside check, render, review, \
        census, rejudged, split, contested, pattern, files, borderline, \
        outcomes, verdicts, and shapes, and the birth count \
        restatement in the one drift free form across the six reading \
        heads, and the constructed choices collection under examples, \
        with the suite running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 15 of 100"));
}

#[test]
fn ci_runs_the_same_gate_as_the_local_gate() {
    let text = read(".github/workflows/ci.yml");
    let live = live_lines(&text);
    // Adjusted with the removal slice of PORT-9, as the named
    // mechanical consequence of the Python gate leaving CI. The
    // two pins that read the Python commands in CI are replaced
    // by pins that read their absence, so the cargo gate stands
    // alone under guard rather than merely unopposed.
    assert!(!live.contains("python3 scripts/codepoint_scan.py"));
    assert!(!live.contains("python3 -m pytest tests -q"));
    assert!(!live.contains("setup-python"));
}

/// The live text of a file, comment text removed the way the
/// Python pin removes it.
fn live_lines(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn readme_reports_the_rust_port() {
    let text = read("README.md");
    // Adjusted with the removal slice of PORT-9, as the named
    // mechanical consequence of the port landing. The pinned
    // line changes from the in review wording to the landed
    // wording the slice writes.
    let port_line = "- The Rust port has landed. \
        It carries the behavior the Python package carried, \
        checked against the same goldens, and the Python tree \
        stands removed under PORT-9.";
    assert!(
        text.lines().any(|line| line == port_line),
        "the port status line stands"
    );
}

#[test]
fn contract_carries_the_port_obligations() {
    let text = read("CONTRACT.md");
    for oid in [
        "PORT-1", "PORT-2", "PORT-3", "PORT-4", "PORT-5", "PORT-6", "PORT-7", "PORT-8", "PORT-9",
    ] {
        assert!(text.contains(oid), "obligation {oid}");
    }
}

#[test]
fn ci_runs_the_cargo_gate_beside_the_python_gate() {
    let text = read(".github/workflows/ci.yml");
    let live = live_lines(&text);
    assert!(live.contains("cargo test"), "the cargo step stands");
    assert!(live.contains("cargo fmt --check"), "the format step stands");
    assert!(
        live.contains("cargo clippy --all-targets -- -D warnings"),
        "the clippy step stands"
    );
}
