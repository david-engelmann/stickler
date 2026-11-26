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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    // zero. Adjusted again with the Round 16 slice, as
    // the named mechanical consequence of slice 17
    // standing in review, so the count stood at one
    // while Round 16 was built, until the Round 16
    // closure landed slice 17 and the count returned to
    // zero. Adjusted again with the Round 17 slice, as
    // the named mechanical consequence of slice 18
    // standing in review, so the count stood at one
    // while Round 17 was built, until the Round 17
    // closure landed slice 18 and the count returned to
    // zero. Adjusted again with the Round 18 slice, as
    // the named mechanical consequence of slice 19
    // standing in review, so the count stood at one
    // while Round 18 was built, until the Round 18
    // closure landed slice 19 and the count returned to
    // zero. Adjusted again with the Round 19 slice, as
    // the named mechanical consequence of slice 20
    // standing in review, so the count stood at one
    // while Round 19 was built, until the Round 19
    // closure landed slice 20 and the count returned to
    // zero. Adjusted again with the Round 20 slice, as
    // the named mechanical consequence of slice 21
    // standing in review, so the count stood at one
    // while Round 20 was built, until the Round 20
    // closure landed slice 21 and the count returned to
    // zero. Adjusted again with the Round 21 slice, as
    // the named mechanical consequence of slice 22
    // standing in review, so the count stood at one
    // while Round 21 was built, until the Round 21
    // closure landed slice 22 and the count returned to
    // zero. Adjusted again with the Round 22 slice, as
    // the named mechanical consequence of slice 23
    // standing in review, so the count stood at one
    // while Round 22 was built, until the Round 22
    // closure landed slice 23 and the count returned to
    // zero. Adjusted again with the Round 23 slice, as
    // the named mechanical consequence of slice 24
    // standing in review, so the count stood at one
    // while Round 23 stood built, until the Round 23
    // closure landed slice 24 and the count returned to
    // zero. Adjusted again with the Round 24 slice, as
    // the named mechanical consequence of slice 25
    // standing in review, so the count stood at one
    // while Round 24 stood built, until the Round 24
    // closure landed slice 25 and the count returned to
    // zero. Adjusted again with the Round 25 slice, as
    // the named mechanical consequence of slice 26
    // standing in review, so the count stood at one
    // while Round 25 stood built, until the Round 25
    // closure landed slice 26 and the count returned to
    // zero. Adjusted again with the Round 26 slice, as
    // the named mechanical consequence of slice 27
    // standing in review, so the count stood at one
    // while Round 26 stood built, until the Round 26
    // closure landed slice 27 and the count returned to
    // zero. Adjusted again with the Round 27 slice, as
    // the named mechanical consequence of slice 28
    // standing in review, so the count stood at one
    // while Round 27 stood built, until the Round 27
    // closure landed slice 28 and the count returned to
    // zero. Adjusted again with the Round 28 slice, as
    // the named mechanical consequence of slice 29
    // standing in review, so the count stood at one
    // while Round 28 stood built, until the Round 28
    // closure landed slice 29 and the count returned to
    // zero. Adjusted again with the Round 29 slice, as
    // the named mechanical consequence of slice 30
    // standing in review, so the count stood at one
    // while Round 29 stood built, until the Round 29
    // closure landed slice 30 and the count returned to
    // zero. Adjusted again with the Round 30 slice, as
    // the named mechanical consequence of slice 31
    // standing in review, so the count stood at one
    // while Round 30 stood built, until the Round 30
    // closure landed slice 31 and the count returned to
    // zero. Adjusted again with the Round 31 slice, as
    // the named mechanical consequence of slice 32
    // standing in review, so the count stood at one
    // while Round 31 stood built, until the Round 31
    // closure landed slice 32 and the count returned to
    // zero. Adjusted again with the Round 32 slice, as
    // the named mechanical consequence of slice 33
    // standing in review, so the count stood at one
    // while Round 32 stood built, until the Round 32
    // closure landed slice 33 and the count returned to
    // zero. Adjusted again with the Round 33 slice, as
    // the named mechanical consequence of slice 34
    // standing in review, so the count stood at one
    // while Round 33 stood built, until the Round 33
    // closure landed slice 34 and the count returned to
    // zero. Adjusted again with the Round 34 slice, as
    // the named mechanical consequence of slice 35
    // standing in review, so the count stood at one
    // while Round 34 stood built, until the Round 34
    // closure landed slice 35 and the count returned to
    // zero. Adjusted again with the Round 35 slice, as
    // the named mechanical consequence of slice 36
    // standing in review, so the count stood at one
    // while Round 35 stood built, until the Round 35
    // closure landed slice 36 and the count returned to
    // zero. Adjusted again with the Round 36 slice, as
    // the named mechanical consequence of slice 37
    // standing in review, so the count stood at one
    // while Round 36 stood built, until the Round 36
    // closure landed slice 37 and the count returned to
    // zero. Adjusted again with the Round 37 slice, as
    // the named mechanical consequence of slice 38
    // standing in review, so the count stood at one
    // while Round 37 stood built, until the Round 37
    // closure landed slice 38 and the count returned to
    // zero. Adjusted again with the Round 38 slice, as
    // the named mechanical consequence of slice 39
    // standing in review, so the count stood at one
    // while Round 38 stood built, until the Round 38
    // closure landed slice 39 and the count returned to
    // zero. Adjusted again with the Round 39 slice, as
    // the named mechanical consequence of slice 40
    // standing in review, so the count stood at one
    // while Round 39 stood built, until the Round 39
    // closure landed slice 40 and the count returned to
    // zero. Adjusted again with the Round 40 slice, as
    // the named mechanical consequence of slice 41
    // standing in review, so the count stood at one
    // while Round 40 stood built, until the Round 40
    // closure landed slice 41 and the count returned to
    // zero.
    assert_eq!(text.matches("built and in review").count(), 1);
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    // Adjusted with the Round 19 slice, as the named mechanical
    // consequence of CK-39 joining the obligations the sets.rs
    // sentence names. The sentence gains CK-39, so the pinned
    // substring gains it too, in the form the Round 14 pin
    // below reads as well.
    // Adjusted again with the Round 20 slice, as the named
    // mechanical consequence of CK-41 joining the obligations
    // the sets.rs sentence names. The sentence gains CK-41,
    // so the pinned substring gains it too, in the form the
    // Round 14 pin below reads as well.
    // Adjusted again with the Round 21 slice, as the named
    // mechanical consequence of CK-43 joining the obligations
    // the sets.rs sentence names. The sentence gains CK-43,
    // so the pinned substring gains it too, in the form the
    // Round 14 pin below reads as well.
    assert!(sets.contains(
        "CK-25, CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
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
    assert!(sets.contains(
        "CK-25, CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
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
    assert!(sets.contains(
        "CK-25, CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-31 that choices reads"));
}

/// The Round 16 obligations pin. It fails against the
/// tree Round 15 closed on by the absence of every
/// text it reads.
#[test]
fn contract_carries_the_round_16_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-33."));
    assert!(text.contains("- CK-34."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains(
        "CK-25, CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-33 that categories reads"));
}

/// The Round 17 obligations pin. It fails against the
/// tree Round 16 closed on by the absence of every
/// text it reads.
#[test]
fn contract_carries_the_round_17_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-35."));
    assert!(text.contains("- CK-36."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains(
        "CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-35 that grounds reads"));
}

/// The Round 18 obligations pin. It fails against the
/// tree Round 17 closed on by the absence of every
/// text it reads.
#[test]
fn contract_carries_the_round_18_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-37."));
    assert!(text.contains("- CK-38."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains(
        "CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-37 that judgements reads"));
}

/// The Round 19 pins. The obligations pin fails against
/// the tree Round 18 closed on by the absence of every
/// text it reads, and the status pin fails on that
/// tree's README by the absence of the two lines it
/// holds, each line read whole as the standing pins
/// read theirs.
#[test]
fn contract_carries_the_round_19_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-39."));
    assert!(text.contains("- CK-40."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains(
        "CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-39 that standings reads"));
}

/// The formats posture pin of the Round 19 slice. The
/// section The formats, version 1 carries no
/// enumeration of the collection form's readers and
/// no text of CK-39, the readers standing definitional
/// under FIX-CK-29 and this round taking no amendment
/// there. It passes against the tree Round 18 closed
/// on by design, and the plan names it with its
/// reason, because it guards a restatement no part of
/// this round makes, and it fails on any build that
/// amends the section or seats an enumeration in it.
#[test]
fn the_formats_section_carries_no_readers_enumeration_and_no_ck_39() {
    let text = read("CONTRACT.md");
    let formats = text.split("## The formats, version 1").nth(1).unwrap_or("");
    assert!(
        !formats.contains("CK-39"),
        "the formats section names no CK-39"
    );
    assert!(
        formats.contains(
            "The readers of the collection form are the commands whose \
             obligations state a file of this form, each named in its own \
             obligation, and no list of them stands in this section."
        ),
        "the definitional readers sentence stands in the formats section"
    );
    assert!(
        !formats.contains(
            "The census, contested, pattern, files, borderline, and \
             outcomes commands read the collection form."
        ),
        "the enumeration stands nowhere in the formats section"
    );
}

/// The formats posture pin of the Round 18 slice. The
/// section The formats, version 1 carries no
/// enumeration of the collection form's readers and
/// no text of CK-37, the readers standing definitional
/// under FIX-CK-29 and this round taking no amendment
/// there. It passes against the tree Round 17 closed
/// on by design, and the plan names it with its
/// reason, because it guards a restatement no part of
/// this round makes, and it fails on any build that
/// amends the section or seats an enumeration in it.
#[test]
fn the_formats_section_carries_no_readers_enumeration_and_no_ck_37() {
    let text = read("CONTRACT.md");
    let formats = text.split("## The formats, version 1").nth(1).unwrap_or("");
    assert!(
        !formats.contains("CK-37"),
        "the formats section names no CK-37"
    );
    assert!(
        formats.contains(
            "The readers of the collection form are the commands whose \
             obligations state a file of this form, each named in its own \
             obligation, and no list of them stands in this section."
        ),
        "the definitional readers sentence stands in the formats section"
    );
    assert!(
        !formats.contains(
            "The census, contested, pattern, files, borderline, and \
             outcomes commands read the collection form."
        ),
        "the enumeration stands nowhere in the formats section"
    );
}

/// The FIX-CK-29 pin of the Round 17 slice. The
/// FIX-CK-29 paragraph stands in the Round 17 section
/// of the contract, the readers of the collection
/// form stand restated in definitional form inside
/// The formats, version 1, and the enumeration the
/// restatement replaces stands nowhere in that
/// section. The absence is scoped to the formats
/// section because the FIX-CK-29 paragraph itself
/// quotes the enumeration it replaces, so an
/// unscoped absence would fail on the quotation. It
/// fails against the tree Round 16 closed on by the
/// absence of the paragraph and of the restated
/// sentence, the enumeration standing in the formats
/// section there.
#[test]
fn fix_ck_29_restates_the_collection_form_readers() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- FIX-CK-29."));
    let formats = text.split("## The formats, version 1").nth(1).unwrap_or("");
    assert!(
        formats.contains(
            "The readers of the collection form are the commands whose \
             obligations state a file of this form, each named in its own \
             obligation, and no list of them stands in this section."
        ),
        "the restated readers sentence stands in the formats section"
    );
    assert!(
        !formats.contains(
            "The census, contested, pattern, files, borderline, and \
             outcomes commands read the collection form."
        ),
        "the enumeration stands nowhere in the formats section"
    );
}

/// The twenty one commands pin of the Round 21
/// slice, adjusted with the Round 23 slice as the
/// named mechanical consequence of the models
/// command standing as the twenty-first command.
/// The enumeration in the module head of
/// rust/src/cli.rs names the twenty one commands
/// ending in models with its count word at twenty
/// one, the head line of the README Usage section
/// stands at twenty one, and the README Usage
/// section carries the models paragraph in the form
/// of the standings paragraph. It fails against the
/// tree Round 22 closed on by content, twenty
/// standing in the cli head and the README head
/// line there and the paragraph absent. The cli
/// head is read flattened, the comment markers
/// and line wraps removed, because the enumeration
/// wraps across the head's lines.
#[test]
fn the_twenty_one_commands_stand_named_in_the_cli_head_and_the_readme_usage() {
    let cli = read("rust/src/cli.rs");
    let flattened = cli
        .replace("//!", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        flattened.contains(
            "The stickler command line. Twenty one commands, check, render, \
             review, census, rejudged, split, contested, pattern, files, \
             borderline, outcomes, verdicts, shapes, choices, categories, \
             grounds, judgements, standings, states, flags, and models."
        ),
        "the cli head names the twenty one commands"
    );
    let readme = read("README.md");
    assert!(readme.contains("The twenty one commands each read one file and print one rendering."));
    let paragraph = "The categories command reads a collection in the census \
        form and prints the categories reading. The reading states one \
        section per category, in the order the contract states them, naming \
        each judgement of the collection standing under that category with \
        its file, its standing number among the judgements of its file, its \
        shape, and the set it stands in, or the no judgements line where no \
        judgement of the collection stands under a category, and a closing \
        count of the judgements stated per section, and it closes with \
        counts over the whole collection.";
    assert!(
        readme.lines().any(|line| line == paragraph),
        "the categories paragraph stands in the Usage section"
    );
    let grounds_paragraph = "The grounds command reads a collection in the census \
        form and prints the grounds reading. The reading states one \
        section per pattern, in the order the contract states them, naming \
        each flag of the collection standing under that pattern and \
        carrying at least one recorded verdict with its file, its line, \
        its pattern, and the set it stands in, the verdict pairs standing \
        on the flag in the standing set review form, or the no verdicts \
        line where no verdict of the collection stands under a pattern, \
        and a closing count of the pairs stated per section, and it closes \
        with counts over the whole collection.";
    assert!(
        readme.lines().any(|line| line == grounds_paragraph),
        "the grounds paragraph stands in the Usage section"
    );
    let judgements_paragraph = "The judgements command reads a collection in the census \
        form and prints the judgements reading. The reading states one \
        section per judgement, in the order the contract states them, naming \
        each flag of the collection standing under that judgement and \
        carrying at least one recorded verdict with its file, its line, \
        its pattern, and the set it stands in, the verdict pairs standing \
        on the flag in the standing set review form, or the no verdicts \
        line where no verdict of the collection stands under a judgement, \
        and a closing count of the pairs stated per section, and it closes \
        with counts over the whole collection.";
    assert!(
        readme.lines().any(|line| line == judgements_paragraph),
        "the judgements paragraph stands in the Usage section"
    );
    let standings_paragraph = "The standings command reads one standings file and prints \
        the standings reading, one section per file carrying recorded \
        verdicts, in first appearance order, each section holding the \
        file's flag groups whole with all their pairs standing in the \
        set review form, and the reading closing with the counts it \
        shares with the verdicts reading and the files reading.";
    assert!(
        readme.lines().any(|line| line == standings_paragraph),
        "the standings paragraph stands in the Usage section"
    );

    let states_paragraph = "The states command reads one states file and prints \
        the states reading, one section per file verdict state in the order CK-8 \
        states the judgements, each section holding the files standing in that \
        state with their flag groups whole and all their pairs standing in the \
        set review form, and the reading closing with the counts it shares \
        with the verdicts reading and the standings reading.";
    assert!(
        readme.lines().any(|line| line == states_paragraph),
        "the states paragraph stands in the Usage section"
    );
    let flags_paragraph = "The flags command reads one flags file and prints \
        the flags reading, one section per file verdict state in the order CK-8 \
        states the judgements, each section holding the flag lines of the files \
        standing in that state in collection order with the contested count of \
        its lines, and the reading closing with the counts it shares with the \
        files reading and the contested reading.";
    assert!(
        readme.lines().any(|line| line == flags_paragraph),
        "the flags paragraph stands in the Usage section"
    );
    let models_paragraph = "The models command reads one models file and prints \
        the models reading, one section per model standing in the collection in \
        first appearance order, each section holding the flag groups of that \
        model in collection order with the one pair each flag carries of that \
        model, and the reading closing with the counts it shares with the \
        verdicts reading and the census.";
    assert!(
        readme.lines().any(|line| line == models_paragraph),
        "the models paragraph stands in the Usage section"
    );
}

/// The doc list pin of the Round 14 slice, adjusted
/// with the Round 20 slice as the named mechanical
/// consequence of the states loader standing as
/// the fifteenth collection loader under the same
/// guard, and adjusted again with the Round 21 slice
/// as the named mechanical consequence of the flags
/// loader standing as the sixteenth collection loader
/// under the same guard, and adjusted again with the
/// Round 23 slice as the named mechanical consequence
/// of the models loader standing as the seventeenth
/// collection loader under the same guard. The module
/// list gains models and the count in the name and
/// this comment moves from sixteen to seventeen, and
/// the pin fails against the tree Round 22 closed on
/// by the seventeenth module's absence. Each of the
/// seventeen loader docs
/// carries the one shared sentence naming the shared
/// set entry validation as the one home every
/// collection loader calls, and none of the seventeen
/// carries a caller enumeration in the loaders both
/// call or loaders all call forms. It passes on any
/// tree whose docs stand restated, including a tree
/// carrying the restatement without the reading,
/// which is correct, because the pin guards the docs
/// and the reading pins guard the reading.
#[test]
fn the_seventeen_loader_docs_carry_the_one_shared_sentence() {
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
        "categories",
        "grounds",
        "judgements",
        "standings",
        "states",
        "flags",
        "models",
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

/// The birth count pin of the Round 15 slice, adjusted
/// with the Round 20 slice as the named mechanical
/// consequence of the states module head standing
/// as the eleventh reading head under the same guard,
/// and adjusted again with the Round 21 slice as the
/// named mechanical consequence of the flags module
/// head standing as the twelfth reading head under
/// the same guard, and adjusted again with the Round
/// 23 slice as the named mechanical consequence of
/// the models module head standing as the thirteenth
/// reading head under the same guard. The module list
/// gains models and the count in the name and this
/// comment moves from twelve to thirteen. Each of
/// the thirteen reading heads, in files, borderline,
/// outcomes, verdicts_reading, shapes, choices,
/// categories, grounds, judgements, standings,
/// states, flags, and models, carries the one shared
/// sentence naming the census as the same recorded
/// inputs over the same collection loading whole,
/// and none of the thirteen carries the loading
/// whole into all form or an enumeration of fellow
/// readings in that sentence.
/// The doc text is read flattened, the comment markers
/// and line wraps removed, because the sentence wraps
/// across the head's lines. It fails against the tree
/// Round 22 closed on by the thirteenth module's
/// absence alone, the twelve standing heads already
/// carrying
/// the sentence there in its drift free form, and it
/// passes on any tree whose heads stand restated,
/// including a tree carrying the restatement without
/// the reading, which is correct, because the pin
/// guards the heads and the reading pins guard the
/// reading.
#[test]
fn the_thirteen_reading_heads_carry_the_one_shared_inputs_sentence() {
    for module in [
        "files",
        "borderline",
        "outcomes",
        "verdicts_reading",
        "shapes",
        "choices",
        "categories",
        "grounds",
        "judgements",
        "standings",
        "states",
        "flags",
        "models",
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 16 status pin in slice form. It fails
/// against the tree Round 15 closed on by content,
/// the standing line and the slice line being absent
/// there.
#[test]
fn readme_reports_the_round_16_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-33 and CK-34 met. They stand in full form in \
        CONTRACT.md, and Round 16 closed on the merged slice 17 \
        after three reviews, no Major on any axis and no repair \
        owed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 17 has landed. It carries the \
        categories reading over a recorded collection of change sets, one \
        section per category in the order the contract states them, naming \
        each judgement of the collection standing under that category with \
        file, standing number, shape, and the set it stands in, with per \
        section judgements read counts and closing counts over the whole \
        collection, with the categories example under examples standing \
        over the same collection as the census example, and the categories \
        command beside check, render, review, census, rejudged, split, \
        contested, pattern, files, borderline, outcomes, verdicts, shapes, \
        and choices, and the constructed categories collection under \
        examples, with the suite running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 17 status pin in slice form. It fails
/// against the tree Round 16 closed on by content,
/// the standing line and the slice line being absent
/// there.
#[test]
fn readme_reports_the_round_17_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-35 and CK-36 met, with FIX-CK-29. They stand \
        in full form in CONTRACT.md, and Round 17 closed on the merged \
        slice 18 after three reviews, no Major on any axis, with the \
        red team's one Minor repaired in the slice.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 18 has landed. It carries the \
        grounds reading over a recorded collection of change sets, one \
        section per borderline pattern in the order the contract states \
        them, naming each flag of the collection standing under that \
        pattern and carrying at least one recorded verdict with file, \
        line, pattern, and the set it stands in, the verdict pairs of \
        the flag in the standing set review form, and a closing count \
        of the pairs stated per section, with closing counts over the \
        whole collection, with the grounds example under examples \
        standing over the same collection as the census example, and \
        the grounds command beside check, render, review, census, \
        rejudged, split, contested, pattern, files, borderline, \
        outcomes, verdicts, shapes, choices, and categories, and the \
        constructed grounds collection under examples, with the suite \
        running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 18 status pin in slice form. It fails
/// against the tree Round 17 closed on by content,
/// the standing line and the slice line being absent
/// there.
#[test]
fn readme_reports_the_round_18_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-37 and CK-38 met. They stand in full form in \
        CONTRACT.md, and Round 18 closed on the merged slice 19 after three \
        reviews, no Major on any axis, with the spec lane's one Minor \
        standing against the design record and the red team's two \
        Observations standing as named voids carried to the Round 19 \
        warm-up.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 19 has landed. It carries the \
        judgements reading over a recorded collection of change sets, one \
        section per judgement in the order CK-8 states them, naming \
        each flag of the collection standing under that judgement and \
        carrying at least one recorded verdict with file, line, pattern, \
        and the set it stands in, the verdict pairs of the flag in the \
        standing set review form, and a closing count of the pairs stated \
        per section, with closing counts over the whole collection, with \
        the judgements example under examples standing over the same \
        collection as the census example, and the judgements command beside \
        check, render, review, census, rejudged, split, contested, pattern, \
        files, borderline, outcomes, verdicts, shapes, choices, categories, \
        and grounds, and the constructed judgements collection under \
        examples, with the suite running offline on the recorded verdicts.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

#[test]
fn readme_reports_the_round_19_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-39 and CK-40 met. They stand in full form in \
        CONTRACT.md, and Round 19 closed on the merged slice 20 after \
        three reviews, no Major on any axis, with the design record's \
        catcher enumerations corrected in the programme records and the \
        standings header restated to its executed postures in the slice.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 20 has landed. It carries the \
        standings reading, the eighteenth command, its sections standing \
        per file in first appearance order across the collection, each \
        file's recorded pairs standing whole on their flags, over the \
        collection form of CK-12.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
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

/// The Round 20 pins. The obligations pin fails against
/// the tree Round 19 closed on by the absence of every
/// text it reads, and the status pin fails on that
/// tree's README by the absence of the two lines it
/// holds, each line read whole as the standing pins
/// read theirs.
#[test]
fn contract_carries_the_round_20_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-41."));
    assert!(text.contains("- CK-42."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains(
        "CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-41 that states reads"));
}

#[test]
fn readme_reports_the_round_20_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-41 and CK-42 met. They stand in full form in CONTRACT.md, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving clean on recomputation of the states reading, and the constructed collection's handed word requirement restated in its loadable form under FIX-CK-30.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 21 has landed. It carries the \
        states reading, the nineteenth command, its sections standing per file verdict \
        state in the order CK-8 states the judgements, each file's recorded pairs \
        standing whole on their flags, over the collection form of CK-12.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The formats posture pin of the Round 20 slice. The
/// section The formats, version 1 carries no
/// enumeration of the collection form's readers and
/// no text of CK-41, the readers standing definitional
/// under FIX-CK-29 and this round taking no amendment
/// there. It passes against the tree Round 19 closed
/// on by design, and the plan names it with its
/// reason, because it guards a restatement no part of
/// this round makes, and it fails on any build that
/// amends the section or seats an enumeration in it.
#[test]
fn the_formats_section_carries_no_readers_enumeration_and_no_ck_41() {
    let text = read("CONTRACT.md");
    let formats = text.split("## The formats, version 1").nth(1).unwrap_or("");
    assert!(
        !formats.contains("CK-41"),
        "the formats section names no CK-41"
    );
    assert!(
        formats.contains(
            "The readers of the collection form are the commands whose \
             obligations state a file of this form, each named in its own \
             obligation, and no list of them stands in this section."
        ),
        "the definitional readers sentence stands in the formats section"
    );
    assert!(
        !formats.contains(
            "The census, contested, pattern, files, borderline, and \
             outcomes commands read the collection form."
        ),
        "the enumeration stands nowhere in the formats section"
    );
}

/// The Round 21 pins. The obligations pin fails against
/// the tree Round 20 closed on by the absence of every
/// text it reads, and the status pin fails on that
/// tree's README by the absence of the two lines it
/// holds, each line read whole as the standing pins
/// read theirs.
#[test]
fn contract_carries_the_round_21_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- CK-43."));
    assert!(text.contains("- CK-44."));
    let sets = read("rust/src/sets.rs");
    assert!(sets.contains(
        "CK-27, CK-29, CK-31, CK-33, CK-35, CK-37, CK-39, CK-41, CK-43, and CK-45 in CONTRACT.md"
    ));
    let readme = read("README.md");
    assert!(readme.contains("the collection form under CK-43 that flags reads"));
}

#[test]
fn readme_reports_the_round_21_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-43 and CK-44 met. They stand in full form in CONTRACT.md, and Round 21 closed on the merged slice 22 after three reviews, with the red team approving on recomputation of the flags golden and the collection figures, the design's MD posture stated in its corrected form from birth, and its one Minor, a header's base posture clause, restated to its true split.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 22 has landed. It carries the \
        flags reading, the twentieth command, its sections standing per file verdict \
        state in the order CK-8 states the judgements, each flag of a file standing in \
        that state standing as one flag line in the form of CK-20, over the \
        collection form of CK-12.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The formats posture pin of the Round 21 slice. The
/// section The formats, version 1 carries no
/// enumeration of the collection form's readers and
/// no text of CK-43, the readers standing definitional
/// under FIX-CK-29 and this round taking no amendment
/// there. It passes against the tree Round 20 closed
/// on by design, and the plan names it with its
/// reason, because it guards a restatement no part of
/// this round makes, and it fails on any build that
/// amends the section or seats an enumeration in it.
#[test]
fn the_formats_section_carries_no_readers_enumeration_and_no_ck_43() {
    let text = read("CONTRACT.md");
    let formats = text.split("## The formats, version 1").nth(1).unwrap_or("");
    assert!(
        !formats.contains("CK-43"),
        "the formats section names no CK-43"
    );
    assert!(
        formats.contains(
            "The readers of the collection form are the commands whose \
             obligations state a file of this form, each named in its own \
             obligation, and no list of them stands in this section."
        ),
        "the definitional readers sentence stands in the formats section"
    );
    assert!(
        !formats.contains(
            "The census, contested, pattern, files, borderline, and \
             outcomes commands read the collection form."
        ),
        "the enumeration stands nowhere in the formats section"
    );
}

/// The Round 22 pins. The amendment pin fails against
/// the tree Round 21 closed on by the absence of the
/// amendment it reads, and the status pin fails on that
/// tree's README by the absence of the two lines it
/// holds, each line read whole as the standing pins
/// read theirs.
#[test]
fn contract_carries_the_round_22_amendment() {
    let text = read("CONTRACT.md");
    assert!(text.contains("- FIX-CK-31."));
    assert!(text.contains(
        "the defect it draws in the rendering of that command is the one sentence, the file's JSON nesting is too deep to read"
    ));
}

#[test]
fn readme_reports_the_round_22_status() {
    let text = read("README.md");
    let standing = "- Contract, FIX-CK-31 met. It stands in full form in CONTRACT.md, and Round 22 closed on the merged slice 23 after three reviews, with the standards review approving, the spec review approving with findings at one Observation standing against the design record's wording, the red team approving clean on executed attack, and CodeRabbit returning no comments.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 23 has landed. It carries the depth message repair under FIX-CK-31, the seventeen covered commands naming the depth refusal in their defect, with the render command's nesting fall and the split command's generic defect standing as they stand, and no reading of a file that parses rendering one byte differently.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}
/// The Round 23 pins. The obligations pin fails
/// against the tree Round 22 closed on by the
/// absence of the obligations it reads, the status
/// pin fails on that tree's README by the absence of
/// the two lines it holds, each line read whole as
/// the standing pins read theirs, and the formats
/// pin passes against that tree by design, in the
/// family's standing posture for that pin kind.
#[test]
fn contract_carries_the_round_23_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 23 obligations, full form"));
    assert!(text.contains("- CK-45."));
    assert!(text.contains("- CK-46."));
    assert!(text.contains("The command line gains a twenty-first command"));
    assert!(text.contains("this loader is its seventeenth caller"));
}

#[test]
fn readme_reports_the_round_23_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-45 and CK-46 met. They stand in full form in CONTRACT.md, and Round 23 closed on the merged slice 24 after three reviews, with the standards review approving with findings at four Observations, the spec review approving clean, the red team approving with findings at three Minors standing against the design record, and CodeRabbit returning one Minor.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 24 has landed. It carries the models reading, the twenty-first command, its sections standing per model in first appearance order, each section holding one flag group per flag carrying a verdict of that model with the one pair of that model the flag carries, over the collection form of CK-12.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 24 pins. The obligations pin fails
/// against the tree Round 23 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_24_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 24 obligations, full form"));
    assert!(text.contains("- CK-47."));
    assert!(text.contains(
        "A collection of the form CK-12 states may carry the same set entry at two standings"
    ));
    assert!(text.contains("labels need not be unique"));
    assert!(text.contains("## Round 24 repair amendment"));
    assert!(text.contains("- FIX-CK-32."));
    let ck46 = text
        .lines()
        .find(|line| line.starts_with("- CK-46."))
        .unwrap_or("");
    assert!(ck46.contains("no handed model name, file name, pattern, label, choice, or reason"));
}

#[test]
fn readme_reports_the_round_24_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-47 met, with FIX-CK-32 spent. They stand in full form in CONTRACT.md, and Round 24 closed on the merged slice 25 after three reviews, with the standards review approving with findings at two Observations, the spec review approving with findings at two Observations, the red team approving with findings at two Minors and one Observation, both Minors standing as CodeRabbit's two inline Minors on the slice, adjudicated valid on the code and cured in the slice by the repair at e3ffc300, and CodeRabbit returning two Minors. The wider executed mutation sets stand against the design record's split statements as the spec review adjudicated them, PA at the verdicts seat felling the fix forward pin beside the family pin, through the binding of its models figure to the verdicts reading over the same fixture, PC felling the six standings pins over the standings collection beside the family pin, the once rule's witnesses at the distinct entries grain, and PE in both its forms felling the fix forward pin, whose collection stands as one entry standing twice under a collection loader, and the three duplicate labels and unread fields pins of CK-12, which a loader refusing or skipping repeated labels fells by definition, each set the charged defect acting on a standing witness of the same rule, with the family pin's own discrimination executed exact at every seat. The contested count of the PD leg stands computed at two homes, the builder's field and the closing recompute in render_contested, and the executed leg stands seated at both, where the design record's PD wording names the builder alone. The family pin's outcomes seat stood asserted by its first two lines alone at the slice head, and the repair cured that grain by asserting the closing whole. The build record's PE passed total stands at 1031 against the red team's executed 1033 passed on the final head, the build figure summing to the tests commit census of 1036, with the failing set identical under both counts.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 25 has landed. It carries the repeated entry standing of CK-47, pinned at the command surface over the sixteen collection commands and the rejudged pair of one entry standing twice, and the fencing enumeration of CK-46 completed under FIX-CK-32, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 25 pins. The obligations pin fails
/// against the tree Round 24 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_25_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 25 obligations, full form"));
    assert!(text.contains("- CK-48."));
    let ck48 = text
        .lines()
        .find(|line| line.starts_with("- CK-48."))
        .unwrap_or("");
    assert!(ck48.contains("The checker judgements of CK-30 and CK-34 stand at each standing"));
    assert!(ck48.contains("No judgement gathering and no judgement count deduplicates a standing"));
}

#[test]
fn readme_reports_the_round_25_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-48 met. It stands in full form in CONTRACT.md, and Round 25 closed on the merged slice 26 after three reviews, with the standards review approving with findings at three Observations, the spec review approving with findings at two Observations, the red team approving with findings at one Minor, the Minor standing as CodeRabbit's one inline Minor on the slice, adjudicated valid on the code and cured in the slice by the repair at d2a5355d, and CodeRabbit returning one Minor. The design record's section 4 phrasing names sixteen collection goldens over census.json, where fifteen stand counted over that input and the files golden stands over files.json, as the spec review's Observation records it. The census total stands computed at two homes, the renderer's summed tally standing as its printed home and the builder's Census.total field standing read by the suite as an independent oracle and never by the renderer, and the QC leg stands seated at both homes in the posture of the Round 24 PD leg, the design record naming the tally's behaviour and no seat. The standards review's anchoring Observation stands against the obligations pin, whose two phrases stand asserted by whole file contains where the Round 24 pin anchors its guarded phrase within its bullet, equivalent in effect today, the cure priced at the next contract opening and not charged to this slice. The design record's sweep ran on the Round 24 red team binary, its provenance proven with every rust/src blob standing identical between that tree and the base, and the slice's warm-up on the slice tree stands as the execution of record. The section exchange mutant passed the whole suite against the pin as first built, and stands executed again against the repaired tree, failing the repaired pin alone after the section scoped restatement.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 26 has landed. It carries the checker judgement standing of a repeated entry under CK-48, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 26 pins. The obligations pin fails
/// against the tree Round 25 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_26_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 26 obligations, full form"));
    assert!(text.contains("- CK-49."));
    let ck49 = text
        .lines()
        .find(|line| line.starts_with("- CK-49."))
        .unwrap_or("");
    assert!(ck49.contains("two judgement bearing set entries each standing twice"));
    assert!(ck49.contains("no gathering orders a section's lines by label"));
}

#[test]
fn readme_reports_the_round_26_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-49 met. It stands in full form in CONTRACT.md, and Round 26 closed on the merged slice 27 after three reviews, with the spec review approving with findings at three Observations, the standards review approving with findings at one Minor and three Observations, the red team approving with findings at one Minor and one Observation, and CodeRabbit returning one inline Minor on the slice, the Minor standing as one defect across the three reads, the categories builder's lines grouped by set label passing the whole suite against the pin as first built, its counts whole and its line order unwitnessed at the categories seat. The defect stands cured in the slice by the repair at abac376d, one order assertion over the smell section's eight lines inside the existing pin, the categories grouping mutant failing the repaired pin alone at 1043 passed and 1 failed and passing whole on the unrepaired head at 1044 passed and 0 failed, both directions executed by the repair lane. The design record's RB and RC computed splits stand corrected by execution under the owner's two rulings, the CK-48 pin falling beside the new pin at its categories and census closings in the same witness form the design record charged at the shapes seat, both executed sets adopted whole as the corrected expectation, with RC at the builder seat executed at the null set as computed, and RF's construction reseated in the QE genus form after the loop header form failed to compile, its executed set standing exact at the seven computed pins. The anchoring cure stands proven in both directions by the red team, the cured pin failing on a guarded phrase moved out of its bullet's line while the phrase stands elsewhere in the contract, where the uncured whole file pin passes over the equivalently moved contract. The red team's files read Observation stands as its own model's correction and no charge against the slice, the per file readings counting distinct files, the posture the CK-47 family pin itself asserts. The new pin's section totals stand as whole output multiplicity counts while its lines stand section scoped, identically the CK-48 pin's repaired Round 25 form, and the tests commit's entry builder reads its separator from the second span with no length guard, standing called at one seat over a three file entry with its removal count asserted first, a panic failing the pin so no silent pass stands. No amendment stands spent by the round, FIX-CK-33 standing free and FIX-CK-27 unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 27 has landed. It carries the checker judgement standing of two judgement bearing entries each standing twice under CK-49, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 27 pins. The obligations pin fails
/// against the tree Round 26 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_27_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 27 obligations, full form"));
    assert!(text.contains("- CK-50."));
    let ck50 = text
        .lines()
        .find(|line| line.starts_with("- CK-50."))
        .unwrap_or("");
    assert!(ck50.contains("two distinct set entries standing under one label"));
    assert!(ck50.contains("reads one entry's judgements at another entry's standing"));
}

#[test]
fn readme_reports_the_round_27_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-50 met. It stands in full form in CONTRACT.md, and Round 27 closed on the merged slice 28 after three reviews, with the spec review approving with findings at one Observation, the standards review approving with findings at one Minor and three Observations, the red team approving with no findings, and CodeRabbit returning one substantive comment on the slice in its review body and no inline comments, the comment standing as the standards review's Minor across the two reads, the census block order CK-11 states standing unasserted by the round's pin as first built, the pin asserting the two per set blocks whole at exactly two occurrences each and the total whole and no sequence over the blocks, so a census rendering walking the standings grouped or permuted printed each block whole in a wrong order under every assertion the pin carried. The defect stands cured in the slice by the repair at d79c28fe, one ordered sequence assertion over the four census blocks inside the existing pin, the grouped census walk passing the unrepaired head whole at 1047 passed and 0 failed and failing the repaired pin alone at 1046 passed and 1 failed, both directions executed by the repair lane, and the design family's SA and SF standing re-executed on the repaired tree name for name. The design executed its whole mutation family at design time and the build re-executed it name for name, SA, SB, and SC falling at the round's pin alone, SD falling at the round's pin with the pins of CK-49 and CK-48 at the renderer's seat and at the null set at the builder's seat, SE at the same three pins, and SF at eight. The red team's own witness mutation SG, the census renderer keeping the first tally it prints under each label and printing it at every later standing of that label, falls at the round's pin alone at a seat the design's family does not name, and its fifteen edge drives over collections of its own devising stand in byte agreement between the slice and base binaries. The one Observation the spec review and the standards review share, the design note's section 7 naming the manifest as rust/Cargo.toml where the repo carries it at its root as Cargo.toml, stands for the errata queue, no slice text resting on it, and the standards review dispositions CodeRabbit's summary remark on the categories ordering check as not valid, the structure section's lines standing distinguishable by their files and asserted as an exact standing order sequence. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1047. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 28 has landed. It carries the checker judgement standing of two distinct entries under one label under CK-50, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The formats posture pin of the Round 23 slice.
/// The section The formats, version 1 carries no
/// enumeration of the collection form's readers and
/// no text of CK-45, the readers standing
/// definitional under FIX-CK-29 and this round
/// taking no amendment there. It passes against the
/// tree Round 22 closed on by design, and the plan
/// names it with its reason, because it guards a
/// restatement no part of this round makes, and it
/// fails on any build that amends the section or
/// seats an enumeration in it.
#[test]
fn the_formats_section_carries_no_readers_enumeration_and_no_ck_45() {
    let text = read("CONTRACT.md");
    let formats = text.split("## The formats, version 1").nth(1).unwrap_or("");
    assert!(
        !formats.contains("CK-45"),
        "the formats section names no CK-45"
    );
    assert!(
        formats.contains(
            "The readers of the collection form are the commands whose \
             obligations state a file of this form, each named in its own \
             obligation, and no list of them stands in this section."
        ),
        "the definitional readers sentence stands in the formats section"
    );
    assert!(
        !formats.contains(
            "The census, contested, pattern, files, borderline, and \
             outcomes commands read the collection form."
        ),
        "the enumeration stands nowhere in the formats section"
    );
}

/// The Round 28 pins. The obligations pin fails
/// against the tree Round 27 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_28_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 28 obligations, full form"));
    assert!(text.contains("- CK-51."));
    let ck51 = text
        .lines()
        .find(|line| line.starts_with("- CK-51."))
        .unwrap_or("");
    assert!(ck51.contains("in the form the pins of those obligations cannot witness"));
    assert!(ck51.contains("groups the block of a standing with the blocks of its label mates"));
}

#[test]
fn readme_reports_the_round_28_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-51 met. It stands in full form in CONTRACT.md, and Round 28 closed on the merged slice 29 after three reviews, with the spec review approving with findings at two Observations, the standards review approving with findings at one Observation, and the red team approving with no findings against the slice and one Observation, every Observation standing against the design record and none charged to the slice, and CodeRabbit returning no actionable comments, no inline comments, and no submitted reviews on the slice. CK-51 stands met by one companion pin over the pin of CK-49's own fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry relabelled alpha set and the second the round 2 set entry itself, the pin driving the sixteen collection commands and asserting the census blocks in standing order, each block asserted whole at two occurrences and the four blocks ordered by the advancing cursor form the Round 27 repair set, each carrying shapes section's lines as an ordered whole at all five sections, and the categories structure and smell sections' lines as ordered wholes, with no standing pin restated, no file under rust/src moving, and the build step standing expressly empty. The slice stands as pull request 69 merged at bce4fd1c, with main's tree standing verified equal to the slice tree c777988c, and the census standing at 1050. The design executed its whole mutation family at design time and the build re-executed it name for name in the warm-up, the warm-up standing whole with no parting and the build's and the design's eighteen line leg ledgers standing byte identical, the four gap legs passing whole on the base at 1047 passed and 0 failed and every gap leg falling at the new pin on the form. The red team re-executed the four base gap legs whole at 1047 passed and 0 failed and TA at the renderer and the grouped tally walk TG on the head name for name, and its own disturbance, a stable sort of the categories structure section's lines by file name, a field no leg of the design's family names, falls at the pin of CK-50 and the new pin as it predicted before its run, the new pin falling at its structure section assertion, and its differential over 208 drives between the base and head binaries standing at no parting. The standards review's own label grouping disturbance at the census renderer fails the new pin alone at its census block order assertion, the pin of CK-49 passing under the same disturbance. The one parting the build found stands settled by execution and recorded in the open, the design note's section 4 print of the pin grouping the shapes call default and class attribute vecs by label, alpha, alpha, round 2, round 2, where the executed text alternates them by standing, alpha, round 2, alpha, round 2, the note's own section 5 prose standing with the executed text against the print, and the print standing named for the errata queue. The design report's section 5 claim, that the pin text printed in the note stands verified byte for byte against the executed scratch text, stands refuted at those two vecs by the spec review's three way comparison of the note's print, the design lane's form tree, and the committed blob, both items standing as records items with no cure charged. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1050. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 29 has landed. It carries the standing order of two judgement bearing entries each standing twice under CK-51, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 29 pins. The obligations pin fails
/// against the tree Round 28 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_29_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 29 obligations, full form"));
    assert!(text.contains("- CK-52."));
    let ck52 = text
        .lines()
        .find(|line| line.starts_with("- CK-52."))
        .unwrap_or("");
    assert!(ck52.contains(
        "at the placeholder name section of the shapes reading and the naming section of the categories reading"
    ));
    assert!(ck52.contains("by the pin's own restatement of the one added line"));
}

#[test]
fn readme_reports_the_round_29_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-52 met. It stands in full form in CONTRACT.md, and Round 29 closed on the merged slice 30 after three reviews, with the spec review approving with findings at two Observations, the standards review approving with findings at one Observation, and the red team approving with findings at three Observations, every Observation standing uncharged to the slice and the spec review's two standing as records items against the build report, and CodeRabbit returning no actionable comments, no inline comments, and no submitted reviews on the slice. CK-52 stands met by one companion pin in the form of the pin of CK-51 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry relabelled alpha set and the second the round 2 set entry itself, the pin's surgery restating the one added line of tidy.py in the round 2 entry as the placeholder assignment and so seating one placeholder name judgement in tidy.py where the standing collection carries none, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks at two occurrences each in standing order, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 71 merged at 2326d5b8, with main's tree standing verified equal to the slice tree b763be1e, and the census standing at 1053. The design executed its whole mutation family at the desk, the gap legs PH and NM passing whole on the base at 1050 passed and 0 failed and each falling at the new pin alone on the form at 1052 passed and 1 failed, the controls BE and SM falling at 1048 passed and 2 failed at the pins of CK-49 and CK-51, the candidate legs standing at 1051 passed and 0 failed for the pin alone on the base and at 1049 passed and 3 failed for the status texts on the base, and the form gate standing whole at 1053 passed and 0 failed with its format and clippy legs at exit 0. The build re-executed the base gate whole at 1050 passed and 0 failed and the sixteen command sweep over the surgical fixture matching the design's figures name for name, and its formatting proof stands executed over the whole form tree at exit 0 and corroborated on the head blobs by the standards review. The red team's independent oracle, built from the fixture and the shipped rules alone, reproduced the companion pin's asserted lines whole at every section of both readings, and its break attempts by construction found no construction that moves a line of either seat while the pin passes. Nine legs of the build's warm-up stand carried unexecuted, b1-ph-shapes, b2-nm-categories, b3-tc-bareexcept, b4-tb-smell, c1-pin-on-base, c2-status-on-base, f1-ph-shapes, f2-nm-categories, and the local form gate run, after twenty nine counted attempts refused by the box write fault in the build lane and a fault shut seating in the red team lane, the red team's differential over the standing command set standing carried with them, and the hosted gate on the slice head standing as the verdict of record in their place. The build report's two records items stand named in the open, the first construction attempt's two commits standing unreferenced with trees identical to the branch chain's stage trees, and the warm-up runner's defect, a text value passed where bytes were required on the first b1 attempt, standing cured in the runner before any leg read a result from it. A sort of either seat's lines by file name stands as the identity over this fixture and invisible to the pin, the design pricing the decline in the open and no asserted line moving under it, and the categories section head order stands unwitnessed by this pin's extraction form, standing under CK-34 and its pins, the shapes head order standing asserted by this pin. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1053. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 30 has landed. It carries the standing order of two judgement bearing entries each standing twice at the naming seats under CK-52, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 30 pins. The obligations pin fails
/// against the tree Round 29 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_30_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 30 obligations, full form"));
    assert!(text.contains("- CK-53."));
    let ck53 = text
        .lines()
        .find(|line| line.starts_with("- CK-53."))
        .unwrap_or("");
    assert!(ck53.contains(
        "at the placeholder name section of the shapes reading and the naming section of the categories reading"
    ));
    assert!(ck53.contains("in place of its print call judgement"));
}

#[test]
fn readme_reports_the_round_30_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-53 met. It stands in full form in CONTRACT.md, and Round 30 closed on the merged slice 31 after three reviews, with the spec review approving with findings at two Observations, the standards review approving with no findings, and the red team approving with findings at two Observations, every Observation standing against the design record and none charged to the slice, and CodeRabbit returning no actionable comments, no inline comments, and no submitted reviews on the slice. CK-53 stands met by one companion pin in the form of the pin of CK-52 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry relabelled alpha set and the second the round 2 set entry itself, the pin's surgery restating the one added line of tidy.py in the round 2 entry as the placeholder assignment and the print call line of report.py in the same entry as the same assignment at its own indent, and so seating the placeholder name judgements of the collection in two files where the fixture of CK-52 carries them in one, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks byte for byte from the derived counts, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 73 merged at 3191cd75, with main's tree standing verified equal to the slice tree 723771f6, and the census standing at 1056. The design proved the gap by computation over the hash verified texts, the placeholder name section of the shapes reading and the naming section of the categories reading standing empty over the standing census collection and over the fixtures of CK-49, CK-50, and CK-51, and carrying four lines each over the fixture of CK-52 with every line standing in tidy.py, so a stable sort of either section's lines by file name stands as the identity there and passes the whole suite at 1053 passed and 0 failed as computed, and over the new fixture the sort parting from the standing order at the second line of each seat, so the new pin falls there and there alone as computed, the gap's execution standing charged to the build's warm-up as legs g1 and g2. The build verified the premise whole over all 171 files of the base tree, executed its formatting proof whole in memory over both touched Rust files with the base texts standing as controls, and cured in the slice the round 4 segment's indent splice the design note's printed form caused, no word of the comment and no substance of the assertion moving under the cure. The red team's independent oracle, built from the fixture and the shipped rules alone and validated against the pin of CK-52 whole before it stood believed anywhere, reproduced the gap computation whole, deriving twenty judgements over the new fixture with the eight lines of each seat alternating report.py and tidy.py at every line and the print call population at zero, and its fuller enumeration over every collection a standing pin drives found the computation's conclusion standing whole, the constructed collections carrying lines of both seats across files while no standing pin asserts a line of either seat over them and the large collection's eleven lines at each seat standing already in file order, and its break attempts by construction found no construction that moves a line of either seat while the pin passes, a stable sort by judgement number parting at the first line of each seat and a stable sort by set label at the third line, each falling at the new pin by the same computation that convicted the file name sort. The warm-up family stands in two legs beside them. The first leg, the nine legs and the differential the Round 29 closure carried, stands executed at b1-ph-shapes alone, the red team's resumed session running that leg whole in the window after the rate reset at 1050 passed and 0 failed, matching its computed expectation exactly, its executed verdict resting on the runner's printed line of the run itself with the run's log standing overwritten by a later refused attempt, and its remaining eight legs, b2-nm-categories, b3-tc-bareexcept, b4-tb-smell, c1-pin-on-base, c2-status-on-base, f1-ph-shapes, f2-nm-categories, and the local form gate run, stand carried unexecuted with the differential over the standing command set, after twenty nine counted attempts in the Round 29 build, forty five counted attempts and a persistent run of four hundred ninety nine at the Round 30 design, ten leg attempts at the Round 30 build, ten attempts in the red team's detached b2 retry loop, seven driver attempts at b1-ph-shapes and eighty refused window probes in the red team's completion lane, and thirty attempts at b2-nm-categories and three attempts at every other carried leg in the red team's resumed session, the one b2 attempt that reached compilation in that session dying on the fault at seven os errors with no assertion failure, and the hosted gate on the slice head standing as the verdict of record in their place. The second leg, this round's own ten legs and its differential, stands carried whole and unexecuted, its gap legs g1-fn-shapes and g2-fn-categories standing derived by the oracle and nowhere executed, its controls and candidate legs standing at their computed figures, its form gate's format leg over the two touched files standing discharged by the build's formatting proof, and the build's landings of its two source trees standing refused after eight counted write attempts at each tree's first file. The design record's items stand named in the open, the note's third section stating the word naming at one place in checker.rs where the substring stands at two places, the naming category site at line 769 and a doc comment at line 836, its fourth section gap computation standing incomplete as a proof against the fuller enumeration the red team executed, its conclusion standing whole, and its printed round 4 segment standing flush left where the segment's seat carries the pin body's indent. The categories section head order stands under CK-34 and its pins with no gap proven, a file name ordering at the sections beside the two seats stands already witnessed by the Round 28 red team's executed disturbance at the structure section, and a judgement number ordering at the two seats stands covered by the new pin without a second obligation. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1056. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 31 has landed. It carries the standing order of two judgement bearing entries each standing twice at the naming seats under CK-53, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 31 pins. The obligations pin fails
/// against the tree Round 30 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_31_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 31 obligations, full form"));
    assert!(text.contains("- CK-54."));
    let ck54 = text
        .lines()
        .find(|line| line.starts_with("- CK-54."))
        .unwrap_or("");
    assert!(ck54.contains(
        "at the print call section, the mutable default section and the call default section of the shapes reading"
    ));
    assert!(ck54.contains(
        "at the smell section of the categories reading, under an ordering of that section's lines by file name"
    ));
}

#[test]
fn readme_reports_the_round_31_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-54 met. It stands in full form in CONTRACT.md, and Round 31 closed on the merged slice 32 after three reviews, with the spec review approving with findings at one Minor and one Observation, the Minor standing charged to the slice on the contract section's seat and the Observation to the build record for its silence on the seat the splice in fact took, the standards review approving with no findings charged on any axis, its formatting proof standing unexecuted under its rate limit stop and charged forward to this closure's fresh extraction gate, and the red team approving with findings at two Minors, both standing against the design record and none charged to the slice, and CodeRabbit returning one submitted review holding one nitpick on the standing count assertion's repeated form and no inline comments, the nitpick dismissed as addressing the family's standing status pin form and naming no defect in the slice. The Minor stands cured in slice before the landing by the repair lane, the Round 31 section reseated at the end of the Round 30 body directly before the formats heading, the formats heading standing at char 328,107 in the reseated contract, and the branch reparented onto main at the resulting trees, the new head's tree verified blob for blob against the reviewed head's map with the contract at its reseated blob. CK-54 stands met by one pin in the form of the pin of CK-53 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry without basket.py under the label alpha set and the second the round 2 set entry itself, the pin's surgery restating the def line of basket.py as a def line carrying a mutable default beside its call default, the def line of report.py as a def line carrying a call default beside its mutable default, and the one added line of tidy.py as an assignment of a print call's result to a placeholder name, and so seating the print call judgements of the collection in two files and the mutable default and call default judgements of the collection each in two files, where the fixtures of CK-49 to CK-53 seat the lines of the three shapes sections in one file and at one judgement number and the lines of the smell section in one file, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks byte for byte from the derived counts, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 77 merged at 1ce0c8b5, with main's tree standing verified equal to the repaired slice tree af776185, and the census standing at 1059. The design proved the gap by computation over the hash verified texts, the file name ordering standing as the identity at six sections and the judgement number ordering at five sections over every asserted drive, and the pin's figures standing computed three ways, by hand from the checker text, by the transliteration the Round 30 red team built, and by the matches rule of the standing pins. The build verified the premise whole over all 171 files of the base tree, re-derived the pin's figures independently by running that transliteration over the fixture surgery in memory, reproducing the design note's figures line for line, and executed its formatting proof whole in memory over both touched Rust files with the base texts standing as controls, three assertion wraps in the pin's surgery standing adopted from the formatter's own output where the lane's first wrapping parted from it. The spec review's completion seating verified the census ladder whole at 1056, 1056, 1057 and 1059 by its own counts, the inserted contract span byte identical to the note's drafted section, and the pin's figures figure for figure against its own hand derivation from the checker text. The red team's adopted oracle, validated against the pins of CK-52 and CK-53 whole and against both goldens before it stood believed anywhere, reproduced the gap computation whole over the new fixture at thirty judgements with its census blocks and matches pairs, confirmed the completeness claim over every asserted drive, and priced the cure's boundary inside the design's own stated radius, the same file tie at the mutable default section standing at the fixture's heart rather than at its edge. The form gate's format leg stands discharged by proof at three seats, the build's formatting proof, the red team's independent reproduction over the base and head texts of both touched files, and this closure's fresh extraction gate, which fetched the head tree's 171 blobs whole with every SHA-1 asserted, recounted the census over the assembled texts at 1059, and proved both touched files byte identical to the installed rustfmt's output at edition 2021 with the base texts standing as controls. The warm-up family stands in three legs beside them. The first leg, the nine legs and the differential the Round 29 closure carried, stands executed at b1-ph-shapes alone, the red team's resumed session running that leg whole in the window after the rate reset at 1050 passed and 0 failed, matching its computed expectation exactly, its executed verdict resting on the runner's printed line of the run itself with the run's log standing overwritten by a later refused attempt, and its remaining eight legs, b2-nm-categories, b3-tc-bareexcept, b4-tb-smell, c1-pin-on-base, c2-status-on-base, f1-ph-shapes, f2-nm-categories, and the local form gate run, stand carried unexecuted with the differential over the standing command set, after twenty nine counted attempts in the Round 29 build, forty five counted attempts and a persistent run of four hundred ninety nine at the Round 30 design, ten leg attempts at the Round 30 build, ten attempts in the red team's detached b2 retry loop, seven driver attempts at b1-ph-shapes and eighty refused window probes in the red team's completion lane, thirty attempts at b2-nm-categories and three attempts at every other carried leg in the red team's resumed session, the one b2 attempt that reached compilation in that session dying on the fault at seven os errors with no assertion failure, eight attempts at every carried leg in the Round 31 build against the source trees standing intact on disk, four attempts at every carried leg in the Round 31 red team's campaign, and three attempts at every carried leg at this closure, every attempt of the last three seatings refused at its tree copy's first file by the write fault, and the hosted gate on the slice head standing as the verdict of record in their place. The second leg, the ten legs and the differential of the Round 30 round, stands carried whole and unexecuted, its gap legs g1-fn-shapes and g2-fn-categories standing derived by the oracle and nowhere executed, its controls and candidate legs standing at their computed figures, its form gate's format leg over the two touched files standing discharged by the Round 30 build's formatting proof, after the Round 31 build's attempts at its base tree landing standing refused at eight counted writes at the first file, four attempts at every leg in the Round 31 red team's campaign, and three attempts at every leg at this closure. The third leg, this round's own twenty nine legs and its differential, stands carried whole and unexecuted, its eleven gap legs standing as the identity over every asserted drive as computed, its three control legs and two candidate legs standing at their computed figures, its seven form legs standing desk proven at the pin's own literals by the standards review at exactly the parting indices the design states, its four boundary legs standing priced inside the design's stated radius by the red team, and its form gate's format leg standing discharged by the proofs above, after the build's landings of its two source trees standing refused at the base tree's first twelve files across eight attempts each and at the form tree's first file across eight attempts, two hundred sixty six leg attempts in the red team's campaign, and three attempts at every leg at this closure, the hosted gate on the slice head standing as the verdict of record in their place. The design record's items stand named in the open, its fourth section's structure row stating the judgement number first parting at index 1 over every fixture drive where it stands at index 2 over the fixtures of CK-52 and CK-53, the census, CK-49 and CK-50 drives standing at index 1 as stated, and its CK-48 sentence stating set label partings at index 1 at the print call, mutable default and call default sections and at index 2 at structure and seating that fixture's placeholder name and naming lines in tidy.py, where over the fixture as constructed no set label parting stands anywhere and those two sections stand empty, both items riding the errata queue and neither moving a verdict, a gap claim, or a leg expectation, and the build record's one Observation standing beside them, its silence on the seat the splice in fact took against its own premise section's recital of the charged seat. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1059. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 32 has landed. It carries the standing order of two judgement bearing entries each standing twice at the print call, mutable default, call default and smell seats under CK-54, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 32 pins. The obligations pin fails
/// against the tree Round 31 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_32_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 32 obligations, full form"));
    assert!(text.contains("- CK-55."));
    let ck55 = text
        .lines()
        .find(|line| line.starts_with("- CK-55."))
        .unwrap_or("");
    assert!(ck55.contains("at the bare except section of the shapes reading"));
    assert!(ck55.contains("as a bare except clause at the same indent"));
}

#[test]
fn readme_reports_the_round_32_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-55 met. It stands in full form in CONTRACT.md, and Round 32 closed on the merged slice 33 after three reviews, with the spec review approving with no findings charged on any axis, the standards review approving with no findings charged on any axis and its formatting proof executed whole over the slice's two touched Rust files, and the red team approving with no findings charged on any axis, its gap re-derived whole by an implementation sharing no code with the design's oracle and its completeness enumeration standing whole over the head tree, and CodeRabbit returning one summary comment stating that no actionable comments were generated, with no inline comments and no submitted review standing anywhere on the pull request. The slice's one in-slice cure stands cured before the reviews, its first head failing both hosted gates at clippy on the type inference error at the CK-55 pin's naming assertion, the bare empty vector literal of that assertion restated in typed form by the repair commit with the assertion's substance unmoved, and the repaired head standing green on both hosted gates. CK-55 stands met by one pin in the form of the pin of CK-54 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry without basket.py under the label alpha set and the second the round 2 set entry itself, the pin's surgery restating the one added line of tidy.py as a bare except clause at the same indent in place of its logging call, raising a bare except judgement and no other, the section then carrying eight lines, report.py at judgement 2 and tidy.py at judgement 1 under the alpha label and the same pair under the round 2 label, and so seating the bare except judgements of the collection in two files and at two judgement numbers, where the fixtures of CK-49 to CK-54 seat the lines of their sections in one file and at one judgement number, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks byte for byte from the derived counts, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 81 merged at 4b248de0, with main's tree standing verified equal to the slice tree 8aa5014a, and the census standing at 1062. The design recomputed the enumeration whole over six asserted drives, the gap standing at exactly bare except and class attribute under both orderings with class attribute declined on its bound, and the pin's figures standing derived three ways in agreement. The build's completion seating verified the premise whole over all 171 files of the base tree and cross-verified the pin's figures against the design's oracle run over the fixture surgery in memory, its formatting proof standing executed in memory over both touched Rust files with the base texts standing as controls, and one formatter joining of the obligations pin's two containment assertions standing adopted, whitespace alone at two sites. The form gate's format leg stands discharged by proof at three seats, the build's formatting proof, the standards review's independent execution of the same proof whole, and this closure's fresh extraction gate, which fetched the head tree's 171 blobs whole with every SHA-1 asserted, recounted the census over the assembled texts at 1062, proved both touched files byte identical to the installed rustfmt's output at edition 2021 with the Round 31 closure tree's texts standing as controls, and landed both the head tree and the Round 32 base tree whole on disk at 171 of 171 blobs each with every landed file read back against its listing hash. The warm-up family stands in four legs beside them. The first leg, the Round 29 family's nine legs and its differential, stands executed whole, b1-ph-shapes at 1050 passed and 0 failed, b2-nm-categories at 1050 passed and 0 failed, b3-tc-bareexcept and b4-tb-smell at 1048 passed and 2 failed each at the pins of CK-49 and CK-51 alone, and c1-pin-on-base at 1051 passed and 0 failed standing executed in the earlier seatings of the round, and its remaining units standing executed at this closure, c2-status-on-base at 1049 passed and 3 failed at exactly the three Round 29 status pins, f1-ph-shapes at 1052 passed and 1 failed at the pin of CK-52 alone, f2-nm-categories at the same figures at the same pin, the local form gate run at 1053 passed and 0 failed with its format and clippy legs at exit 0, and the differential standing byte identical at every drive. The second leg, the Round 30 family's nine legs and its form gate and differential, stands executed whole, its gap legs g1-fn-shapes and g2-fn-categories at 1053 passed and 0 failed standing executed at the build's completion seating, and its remaining units standing executed at this closure, g3-tc-bareexcept and g4-tb-smell at 1050 passed and 3 failed each at the pins of CK-49, CK-51 and CK-52 under the design's corrected expectations, c1-pin-on-base at 1054 passed and 0 failed, c2-status-on-base at 1052 passed and 3 failed at the three Round 30 status pins, f1-fn-shapes and f2-fn-categories at 1055 passed and 1 failed each at the pin of CK-53 alone, the form gate at 1056 passed and 0 failed, and the differential standing byte identical at every drive. The third leg, the Round 31 family's twenty nine legs and its differential, stands executed at its base control alone at this closure, at 1056 passed and 0 failed, its first gap leg standing attempted at three full executions in this closure's hunt, every one refused at the build's write stage by the disk quota with no assertion failure standing in any of them, a fault failure standing beside its set and never inside it, and its remaining twenty seven legs and its differential standing carried unattempted at this closure behind that refusal, beside the attempts the earlier seatings counted in their own records, its candidate status leg standing defined in the corrected setup only form the build's machinery correction states, its seven form legs standing desk proven at the pin's own literals by the standards review of its round, and its form gate's format leg standing discharged by the proofs of its round. The fourth leg, this round's own sixteen legs and its differential, stands carried whole and unattempted at this closure, its gap legs standing as the identity over every asserted drive as computed, its control and candidate legs standing at the design's computed figures, its form legs standing at the pin of CK-55 alone as computed, its boundary legs standing priced inside the design's stated radius by the red team, and its form gate's format leg standing discharged by the proofs above, the hosted gate on the slice head standing as the verdict of record in the carried legs' place. The records items stand named in the open, the Round 30 design note's two fall statements for the control legs g3-tc-bareexcept and g4-tb-smell standing corrected to three falls each and the correction standing executed at this closure at the corrected figures, and the Round 31 red team record's two file form of the candidate status leg standing corrected to the family's setup only form by the build's machinery correction, both items riding the errata queue and neither moving a verdict, a gap claim, or a leg expectation beyond the correction each states. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1062. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 33 has landed. It carries the standing order of two judgement bearing entries each standing twice at the bare except seat under CK-55, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 33 pins. The obligations pin fails
/// against the tree Round 32 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_33_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 33 obligations, full form"));
    assert!(text.contains("- CK-56."));
    let ck56 = text
        .lines()
        .find(|line| line.starts_with("- CK-56."))
        .unwrap_or("");
    assert!(ck56.contains("at the class attribute section of the shapes reading"));
    assert!(ck56.contains("seating every line after it in that file in a class body"));
}

#[test]
fn readme_reports_the_round_33_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-56 met. It stands in full form in CONTRACT.md, and Round 33 closed on the merged slice 34 after three reviews and the owner's verification, with the spec review approving with no findings charged against the slice, one Minor and one Observation charged against the design record, and one Minor and one Observation charged against the build record, the standards review approving with findings, one Minor charged against the slice at the section's numbering clause and two Observations on the footer totals form, one Minor charged against the design record at the same sentence's root in its draft, and two Observations charged against the build record, and the red team approving with no findings charged against the slice, its enumeration re-derived independently and standing closed under both field orderings over every asserted drive and the fixture's sixteen judgements re-derived from the production whole, with one Minor charged against the design record at its carried expectations for the structure judgement number leg in two families and three Observations charged against the build record, and CodeRabbit returning one COMMENTED review with three inline comments, one nitpick, and one summary comment, the contract numbering comment standing valid as the standards Minor, the two footer comments standing dismissed against the slice at the price the reviews state, the nitpick standing dismissed, and the summary's sort remark naming a mutation the design's split does not charge. The slice's one in-slice cure stands cured before the merge, the numbering clause of the CK-56 section standing restated from the number of its line among the judged lines of its file to its standing number among the judgements of its file, in the contract's own standing words at two earlier seats of the file, one sentence moving and the file standing at 340,061 characters against 340,066 at the reviewed head, the repaired head 4bd745db standing green on both hosted gates, and no code, no pin, and no figure moving anywhere in the repair. CK-56 stands met by one pin in the form of the pin of CK-55 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry without basket.py under the label alpha set and the second the round 2 set entry itself, the pin's surgery restating the def line of report.py as a class line at its own indent, seating every line after it in that file in a class body under the walk that reads its class context from the context lines and added lines of the diff itself, and restating the print line of report.py as an assignment of a single literal to a placeholder name at its own indent, that one line raising a placeholder name judgement and a class attribute judgement both in the production's order at judgements 2 and 3 of its file, the section then carrying six lines, report.py at judgement 3 under the alpha label, basket.py at judgement 1 and report.py at judgement 3 under the round 2 label, and the same three again at the second standings, with basket.py standing second and fifth, and so seating the class attribute judgements of the collection in two files and at two judgement numbers, where the fixtures of CK-49 to CK-55 seat the lines of their sections in one file and at one judgement number, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks byte for byte from the derived counts, the population standing at sixteen judgements and the totals at Sets 4, Files 10, and Judgements 16, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 85 merged at f8ac8cc3, with main's tree standing verified equal to the slice tree add64d0a, and the census standing at 1065. The design recomputed the enumeration whole over seven asserted drives, the gap standing at exactly the class attribute section under both field orderings and at no section under the set label or printed word orderings, the route standing found in the walk's reading of added lines that the earlier pricing never enumerated, the categories head order thread standing declined a fourth time on its unmoved evidence, FIX-CK-27 standing declined again at the price of a producer that does not exist, and the fixture's figures standing derived three ways in agreement. The build verified the premise whole over all 171 files of the base tree, its assembly standing deterministic over two runs, the drafted contract section standing rendered into the family's heading and bullet form with no drafted word moving, a deviation of mechanism the build names in the open and the reviews price in the open, one formatter wrap inside the pin standing adopted from the formatter's own output, and the seven stage construction standing read back at every stage with the fresh extraction gate executed against the pushed head. The form gate's format leg stands discharged by proof at three seats, the build's formatting proof, the standards review's independent execution of the same proof whole, and this closure's fresh extraction gate, which fetched the head tree's 171 blobs whole with every SHA-1 asserted, recounted the census over the assembled texts at 1065, and proved both touched files byte identical to the installed rustfmt's output at edition 2021 with the Round 32 closure tree's texts standing as controls. The warm up stands executed whole over the round's three legs beside them, no unit standing carried at this closure. The first leg, the Round 31 family's twenty nine legs and its differential, stands executed whole, the eleven gap legs at 1056 passed and 0 failed each, ctl-label-be at 1052 passed and 4 failed at the pins of CK-49, CK-51, CK-52, and CK-53, ctl-jn-structure at 1050 passed and 6 failed at the categories golden pin's two tests and the pins of CK-50 to CK-53, ctl-fn-pn at 1055 passed and 1 failed at the pin of CK-53 alone, cand-pin31 at 1057 passed and 0 failed, cand-status31 in the setup only form at 1055 passed and 3 failed at its three named status pins, the seven form legs at 1058 passed and 1 failed at the pin of CK-54 alone each, the four boundary legs at 1059 passed and 0 failed each, the form gate at 1059 passed and 0 failed, and the differential standing byte identical at every drive, the leg standing executed by the build's detached driver past its session's close and verified from its logs by the red team's recount and by this closure's recount of every leg log, its one lost log standing restored by the red team's solo re-execution at its stated set. The second leg, the Round 32 family's seventeen units, stands executed whole, its first five units standing at their computed seats by the build's driver, its structure judgement number leg at 1052 passed and 7 failed at the golden pin's two tests and the pins of CK-50 to CK-54, and its remaining eleven units standing executed by the red team, ctl-fn-pn at 1058 passed and 1 failed at the pin of CK-53 alone after three executions stood carried by the write fault beside its set, cand-pin32 at 1060 passed and 0 failed, cand-status32 in the setup only form at 1058 passed and 3 failed at its three named status pins, the two form legs at 1061 passed and 1 failed at the pin of CK-55 alone each, the class attribute boundary legs at 1062 passed and 0 failed each, the corrected boundary legs bnd-fn-ph and bnd-jn-nm at 1061 passed and 1 failed at the pin of CK-53 alone each under the design's corrected expectations, the form gate at 1062 passed and 0 failed, and the differential standing byte identical at every drive. The third leg, this round's own seventeen units, stands executed whole on the same division, its first five units standing at their computed seats, its structure judgement number leg at 1054 passed and 8 failed at the golden pin's two tests and the pins of CK-50 to CK-55, ctl-fn-pn at 1061 passed and 1 failed at the pin of CK-53 alone, cand-pin33 at 1063 passed and 0 failed, cand-status33 at 1061 passed and 3 failed at its three named status pins on its third attempt, the two form legs at 1064 passed and 1 failed at the pin of CK-56 alone each, the bare except boundary legs at 1064 passed and 1 failed at the pin of CK-55 alone each, the call default boundary legs at the same figures at the pin of CK-54 alone, the form gate at 1065 passed and 0 failed, and the differential standing byte identical at every drive. The Round 29 and Round 30 families stand executed whole in the earlier record and stand not re-charged at this round, their g3 and g4 legs standing executed at the Round 32 closure at their corrected expectations of three falls each. The records items stand named in the open, the walk sentences of the Round 31 and Round 32 design notes standing corrected by this round's note to the production reading its class context from the context lines and added lines of the diff itself, the Round 32 note's boundary expectations for bnd-fn-ph and bnd-jn-nm standing corrected to one fall each at the pin of CK-53 and the correction standing confirmed by execution at this round, the design note's carried expectations for the structure judgement number leg in the Round 32 and Round 33 families standing corrected to 1052 passed and 7 failed and 1054 passed and 8 failed, the library route golden falling beside the command route in both families, the numbering sentence of the note's drafted section standing as the root of the slice's cured clause with the note standing unedited and its correction riding the errata queue, and the build record's missing warm up ledger at the reviews' reads standing cured in the record by the build's later sections, the red team's hunt, and this closure's statement of the ledger whole, none of the items moving a verdict or a leg's executed set beyond the corrections each states. The round's obligations pin and its two status pins stand in the family's form as charged, and the suite stands at 1065. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 34 has landed. It carries the standing order of two judgement bearing entries each standing twice at the class attribute seat under CK-56, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 34 pins. The obligations pin fails
/// against the tree Round 33 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_34_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 34 obligations, full form"));
    assert!(text.contains("- CK-57."));
    let ck57 = text
        .lines()
        .find(|line| line.starts_with("- CK-57."))
        .unwrap_or("");
    assert!(ck57.contains("stands bound within that section's own slice of the printed output"));
    assert!(ck57.contains("an empty section's closing standing asserted at zero within its slice"));
}

#[test]
fn readme_reports_the_round_34_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-57 met. It stands in full form in CONTRACT.md, and Round 34 closed on the merged slice 35 after three reviews and the owner's verification, with the spec review approving with no findings charged against the slice, one Major, two Minors and one Observation charged against the design record, and one Observation charged against the build record, the standards review approving with findings, one Observation charged against the slice at the count pins' categories closing assertion riding the line entries loop in the standing loop's inherited shape, two Observations charged against the design record, and one Observation charged against the build record, and the red team approving with findings, one Observation charged against the slice at the CodeRabbit occurrence form item, three Minors charged against the design record, and one Observation charged against the build record, its population re-derived independently at nine pins and thirty eight assertions by an extraction sharing no code with the design's and its two gap legs and form shapes leg executed firsthand at their corrected expectations, and CodeRabbit returning one COMMENTED review with one inline comment, one summary comment and one administrative reply, the inline comment standing dismissed by all three axes at the prices the reviews state, the slice carrying exactly one line of the judgements form under the shipped grammar, its own section's closing, and the summary standing as a walkthrough with no actionable item beyond the one inline comment. CK-57 stands met by the restatement of the nine standing pins of rust/tests/test_cli.rs, the thirty eight whole output matches assertions over the judgements footer standing removed and the within slice closing assertions standing in their place at thirty seven assertion sites covering the eighty one section seats, nine sections in each of the nine pins, each section's printed judgement total standing bound within that section's own slice of the printed output and at no other seat, the eight empty section seats carrying a slice cut beside their unmoved containment blocks with the closing asserted at zero within the slice, the tenth pin's count over the verdicts closing standing untouched outside the obligation, and no production code, no fixture and no golden moving anywhere in the round, with the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 89 merged at 41916fc6, with main's tree standing verified equal to the slice tree d67e360a, and the census standing at 1067. The design recomputed the footer population whole by two independent extractions agreeing at nine pins and thirty eight assertions, priced the exchange mutation as the discriminating leg the whole output form cannot see, and charged the construction in the family's commit order, its gap proof standing corrected at the four containment seats and its form leg statements at their range under the owner's two rulings, the corrections standing charged to the design record and moving no text the slice ships, the categories head order thread standing declined a fifth time on its unmoved evidence, and FIX-CK-27 standing declined again with its trigger unmet. The build verified the premise whole over all 171 files of the base tree, stopped twice under CK-57's own stopping rule with each parting reported before any repair, and constructed the slice in seven commits only under the corrected expectations the rulings adopted, the expressly empty steps standing proven by tree hash identity at every stage, and the seven stage construction standing read back at every stage. The form gate's format leg stands discharged by proof at four seats, the build's formatting proof executed at two of its seatings, the standards review's independent execution of the same proof whole, and this closure's fresh extraction gate, which fetched the head tree's 171 blobs whole with every SHA-1 asserted, recounted the census over the assembled texts at 1067, and proved both touched files byte identical to the installed rustfmt's output at edition 2021. The warm up stands executed whole over the round's six legs at the build, no unit standing carried from the build itself. Leg gap-shapes stands at two falls at the in-place binders of the shapes reading, leg gap-categories at eleven falls at the seven categories binders and four carrying pins at their naming containment assertions, leg form-shapes at eight falls whole over the suite at its six restated pins and the two shapes binders, leg form-categories at fifteen falls whole over the suite at its eight restated pins and the seven categories binders with the CK-53 pin passing where the pair stands equal, leg cand-pin34 passing whole at the standing census, and leg cand-status34 at its three named status pins. The red team executed the two gap legs and leg form-shapes firsthand at the same sets and carried leg form-categories under the write fault, and that leg stands executed whole at this closure on its sixth counted attempt at 1052 passed and 15 failed with its fall set exact by name, four of its attempts standing refused whole by the fault with zero tests executed and its fourth standing executed over thirty three of the suite's thirty four binaries with the fall set exact and one binary's death standing as a fault failure beside the set. The carried family of the earlier rounds stands empty, the Round 33 closure having executed its sixty three units whole and carried nothing. The records items stand named in the open, the design note's gap categories proof standing corrected to the eleven falls the execution stands at, its form leg statements standing corrected to the whole suite sets of eight and fifteen, its naming of the shapes golden pin among the gap shapes falls standing corrected to the two in-place binders alone, the design note's pricing of the eighty one standing corrected to section seats over thirty seven assertion sites, and the build record's first seating figure for the base setup file standing corrected in its own record from 124,168 characters to the true 134,275, none of the items moving a verdict or a leg's corrected set beyond the corrections each states. The round's obligations pin and its status pin stand in the family's form as charged, and the suite stands at 1067. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 35 has landed. It carries the footer bound under CK-57, the nine standing pins restated to bind each section's printed judgement total within that section's own slice, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 35 pins. The obligations pin fails
/// against the tree Round 34 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_35_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 35 obligations, full form"));
    assert!(text.contains("- CK-58."));
    let ck58 = text
        .lines()
        .find(|line| line.starts_with("- CK-58."))
        .unwrap_or("");
    assert!(ck58.contains(
        "stands asserted by the line standing at the closing seat of that section's own slice"
    ));
    assert!(ck58.contains("each section's closing stands asserted once in its pin"));
}

#[test]
fn readme_reports_the_round_35_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-58 met. It stands in full form in CONTRACT.md, and Round 35 closed on the merged slice 36 after three reviews and the owner's verification, with the spec review approving with no findings charged against the slice, two Majors and two Minors charged against the design record, and one Minor and one Observation charged against the build record, the standards review approving with findings, one Observation charged against the slice at the new pin's fourfold slice and assert loop standing in the family's self contained pin form, and two Observations charged against the build record, and the red team approving with no findings charged against the slice and one Observation charged against the design record at the closing line duplicated inside a section's slice, a shape the selected form passes where the count form fell and no family mutation produces, the seat standing bound by the golden whole text pins at other seats of the suite, its population re-derived independently at nine pins, thirty seven closing count sites and eighty one section seats by instruments sharing no code with the design's or the build's, and CodeRabbit returning no actionable comment, its summary standing as a walkthrough of the slice as it stands and no inline comment standing on the pull request. CK-58 stands met by the restatement of the nine standing pins of CK-57 in rust/tests/test_cli.rs, the thirty seven occurrence count closing assertions standing removed and the selected closing assertions standing in their place at the same thirty seven assertion sites covering the eighty one section seats, nine sections in each of the nine pins, each section's closing standing asserted by the line at the closing seat of its own slice and once in its pin, the thirteen empty section seats standing asserted at zero on the same form, and one new pin standing in the same file over two collections it constructs in its own text, seating the section whose own total carries the whole population of its reading at both readings, its thirty six section seats asserted on the same selected form, and no production code, no fixture and no golden moving anywhere in the round beyond the two collections the new pin constructs. The slice stands as pull request 93 merged at fed155d1, with main's tree standing verified equal to the slice tree c514f069 at this closure firsthand, and the census standing at 1070. The design recomputed the closing population whole by three independent extractions agreeing at nine pins, thirty seven assertion sites and eighty one section seats, computed the masking seat absent at all eighteen last section seats of the standing fixtures, and charged the construction in the family's commit order, the categories head order thread standing declined a sixth time on its unmoved evidence, and FIX-CK-27 standing declined again with its trigger unmet. The build verified the premise whole over all 171 files of the base tree and stopped at the premise rule on a parting at the design's status step alone, the owner ruling for the shipped family's form whole on all four questions, the status pin standing under the family name readme_reports_the_round_35_status with its three assertions, the standing line standing in the family open form with its words at exactly one place in the slice README, that place the slice line alone and the round 4 standing pin's assertion standing restated from zero to one with its chain segment seated, and leg cand-status35's fall set standing corrected to the c2 setup only form's executable set, and the owner's correction adopting the two gap legs' executed truth whole at nine falls each on the base tree against the design's stated sets, the containment seats its computation did not charge standing charged to the design record. The slice then stands constructed in seven commits in the family's order under the corrected expectations, the expressly empty steps standing proven by tree hash identity at every stage. The formatting proof stands executed at the build over the three constructed Rust texts with the base texts as controls, confirmed at the standards review in check mode over the same texts, and executed at this closure over the constructed setup text against its head text as control. The warm up stands executed whole over five of the round's six legs in every charged form and at the base form of the sixth. Leg gap-moved-shapes stands executed at nine falls on the base tree and at fifteen falls in its slice form at this closure, the nine pins, the new pin, both shapes golden pins and the same three whole text pins the base execution felled, the derivation of the slice form standing corrected at the new pin's seat. Leg gap-moved-categories stands executed at nine falls on the base tree, and its slice form stands carried at this closure under the write fault at seven counted attempts, one partial run recording the same fifteen falls over the tests that reported before a disk quota failure killed a binary in flight beside the set, the hosted gate standing as the carried form's verdict of record. Leg gap-masked-drop stands executed whole at this closure at thirty three falls against the design's stated twelve, the nine pins, the new pin, and the readings' whole text and golden pins the dropped closing moves, leg form-exchange-shapes at nine falls against its stated six, the six pins whose pair differs with the new pin and two shapes whole text pins beside them, leg cand-pin35 at thirty two falls on its fourth counted attempt, its set standing as the masked drop set with the new pin's fall alone removed and proving the new pin the masking seats' only binder, and leg cand-status35 at exactly its corrected three falls, the obligations pin, the round 35 status pin and the round 4 standing pin. The red team's own masking only drop probe stands executed whole at this closure at five falls against its stated one, the new pin with the empty collection pin at both readings at the degenerate population of zero and two categories pins whose drives seat the last section at the whole population. The carried family of the earlier rounds stands empty. The records items stand named in the open, the design note's section 9 status texts standing corrected under the owner's four rulings, its two gap legs' stated sets standing corrected to the executed nine falls on the base tree, its stated sets for legs gap-masked-drop, form-exchange-shapes and cand-pin35 standing corrected to the executed thirty three, nine and thirty two, the build record's derivation of the gap legs' slice forms standing corrected at the new pin's seat, the red team's probe price standing corrected to five, and the build record's warm up standing partially unfulfilled at the slice gate with four legs carried named, the carrying standing as a deviation from the spec's letter under the spec review's plain judgement, discharged on counted fault evidence with the hosted gate as verdict of record and not admitted by CK-58's own terms, none of the items moving a verdict or the slice's texts. The round's obligations pin and its status pin stand in the family's form as charged, and the suite stands at 1070. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 36 has landed. It carries the closing line selected as such under CK-58, the nine standing pins restated to assert each section's closing by the line at its closing seat, asserted once per section, and one new pin seating the section whose own total carries the whole population of its reading, with no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 36 pins. The obligations pin fails
/// against the tree Round 35 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_36_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 36 obligations, the closing line standing once in its slice"));
    assert!(text.contains("- CK-59."));
    let ck59 = text
        .lines()
        .find(|line| line.starts_with("- CK-59."))
        .unwrap_or("");
    assert!(ck59.contains("stands asserted to stand once in its slice"));
    assert!(ck59.contains("at the forty one assertion sites where the closing seat stands read"));
}

#[test]
fn readme_reports_the_round_36_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-59 met. It stands in full form in CONTRACT.md, and Round 36 closed on the merged slice 37 after three reviews and the owner's verification, with the spec review approving with no findings charged against the slice, one Observation charged against the design record, and one Minor charged against the build record, the standards review approving with no findings charged against the slice, one Observation charged against the design record, and one Observation charged against the build record, and the red team approving with no findings charged against the slice, one Observation charged against the design record, and one Minor charged against the build record, its prefix computation re-derived sound for arbitrary fixture data, its computed fall sets corroborated pin by pin from the pins' own texts, and its hunt carried at three counted attempts, and CodeRabbit standing with one inline comment on the slice line graded Major in its own form, its substance the carrying of the warm up, dismissed by the owner at step 0 with its reason recorded at the gate, the slice line standing as the family's standing status form for a constructed slice under review and the carrying standing disclosed in the open in the build record with the legs charged to this closure. CK-59 stands met by the restatement of the ten standing pins in rust/tests/test_cli.rs, beside each seat assertion at the forty one closing_seat_line call sites each pin asserting the section's closing line stands among the lines of the helper's own prefix at exactly one line, the prefix standing as the part of the section's slice standing before the reading's closing counts under the helper's own split, covering the eighty one section seats of the nine pins, the thirteen empty among them, and the thirty six seats of the pin over the whole population, the masking seats standing among the family's seats at the places the red team names seat by seat, the assertion standing as a named deviation of form from the count assertion it extends, and no new pin standing seated and no production code, no fixture and no golden moving anywhere in the round. The slice stands as pull request 97 merged at 359a9075, with main's tree standing verified equal to the slice tree 83a7e845 at this closure firsthand, and the census standing at 1072. The design took the one failure the Round 35 red team named against the selected form as its whole subject, computed the prefix scope whole at the masking seats, priced the duplicated closing family pin by pin over the whole suite, desk checked the status step against the shipped family form at the slice head, and charged the construction in the family's commit order, the categories head order standing declined a seventh time on its verified unmoved evidence, the tenth pin's verdicts closing count standing declined again on the Round 33 computation, FIX-CK-27 standing declined again with its triggers unmet, and the design time executions standing attempted and refused in the open with its splits standing computed. The build verified the premise whole over all 171 files of the base tree at two seatings, the first ending after its premise with nothing pushed and the completion seating adopting its banked sections and scratch whole, and constructed the slice in seven commits in the family's order, the expressly empty steps standing proven by tree hash identity at every stage, the status pin standing in the family form with its three assertions, the slice phrase standing at exactly one place in the slice README, that place the slice line alone, its warm up standing carried whole under the write fault in its wholesale refusal form at counted attempts, and the four form texts standing proven by their blob hashes against the slice head's own blobs where the fault refused their persistence. The formatting proof stands executed at the build over the two constructed Rust texts in stdin mode at the crate's edition with the base texts as controls, confirmed at the standards review over the same texts, and executed at this closure over the constructed setup text with its head text as control. The warm up stands carried whole at the slice and stands carried at this closure in the form the evidence leaves it. carried-moved-categories stands carried on a partial run at ten counted attempts, its seventh run reporting 948 of the census of 1,070 at 933 passed and 15 failed with its fifteen falls standing exactly at the leg's computed seats and its one operating system error standing beside the set, its whole run standing owed. dup-shapes-base, dup-categories-base, dup-shapes-slice, dup-categories-slice, cand-pin36 and cand-status36 stand carried at one tree assembly attempt each, every assembly refused at its first file through the verified write retry budget under the quota refusal, and the fresh extraction gate at the slice head stands carried beside them on the same refusals, the hosted gates standing as the verdict of record for every carried leg. The records items stand named in the open, the design note's section 9 wording on the slice line's seat standing at one Observation from each of the three reviews, the executed form seating the closure's two lines beside the landed slice 36 line which stands unmoved under its own round's pin, the build record's verified write retry budget standing unnumbered at one Observation from the standards review, and the spec review's plain judgement on the carrying standing recorded at this closure, none of the items moving a verdict or the slice's texts. The round's obligations pin and its status pin stand in the family's form as charged, and the suite stands at 1072. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 37 has landed. It carries the closing line standing once in its slice under CK-59, the ten standing pins restated to assert each section's closing standing once among the lines standing before the reading's closing counts in that section's own slice, asserted once per section beside the seat assertion of the same section, and no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 37 pins. The obligations pin fails
/// against the tree Round 36 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_37_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains("## Round 37 obligations, the verdicts closing bound in its slice"));
    assert!(text.contains("- CK-60."));
    let ck60 = text
        .lines()
        .find(|line| line.starts_with("- CK-60."))
        .unwrap_or("");
    assert!(ck60.contains("stands asserted at the closing seat of its own slice"));
    assert!(ck60.contains("the round 2 set's closing at zero"));
}

#[test]
fn readme_reports_the_round_37_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-60 met. It stands in full form in CONTRACT.md, and Round 37 closed on the merged slice 38 after three reviews and the owner's verification, with the spec review approving with one Observation charged against the slice at the pin's reading of its seat over the section block's own lines in place of the closing prefix helper, and two Minors charged against the build record, the standards review approving with no findings charged against the slice or the design record, one Minor and one Observation charged against the build record, and the red team approving with no findings charged against the slice or the design record, and two Minors charged against the build record, its uniqueness argument re-derived from the shipped format literals alone in a form stronger than the design states it, its discrimination splits corroborated pin by pin from the pins' own texts, the shipped block form standing proven to bind every shape the charged prefix form binds, and its hunt carried whole on the fleet rate limit and not on the fault, and CodeRabbit standing with no inline comments at the spec and standards reviews' reads, its summary stating no actionable comments were generated and its merge risk at Minimal. CK-60 stands met by the restatement of the tenth pin in rust/tests/test_cli.rs in the block form over the four section blocks of its pilot fixture's verdicts output in standing order at totals zero, twenty two, twenty two and ten, each block closing by the last line of its own block, each closing asserted to stand at the closing seat of its own block and to stand among the lines of its block standing before the reading's closing counts at exactly one line, the round 2 set's none recorded line asserted at the line directly before its closing seat, the pin reading its seat and its count over the section block's own lines rather than through the closing prefix helper, standing as a named deviation of form from the sibling rounds' helper alone, the block's lines and the prefix scope's lines standing as the same set at every seat, and no other assertion in the pin moving, and no production code, no fixture and no golden moving anywhere in the round. The slice stands as pull request 101 merged at d565b7d2, with main's tree standing verified equal to the slice tree d8252ece at this closure firsthand, and the census standing at 1074. The design took the verdicts form whole, the candidate standing declined at Rounds 34, 35 and 36 in the words a later round remaining free to take it whole, computed the uniqueness of the four closings from the shipped line forms whole, priced its discrimination splits pin by pin over the whole suite, desk checked the status step against the shipped family form at the Round 36 slice head, and charged the construction in the family's commit order, the categories head order standing declined an eighth time on its verified unmoved evidence, FIX-CK-27 standing declined again with all three of its reopen triggers standing unmet and verified, and no design time mutation execution standing attempted at its seating, its splits standing computed and charged to the build's warm up. The build verified the premise whole over all 171 files of the base tree and constructed the slice in seven commits in the family's order, the expressly empty steps standing proven by tree hash identity at every stage, the status pin standing in the family form with its three assertions, the slice phrase standing at exactly one place in the slice README, that place the slice line alone, and its warm up standing carried whole under the write fault in its wholesale refusal form, its thirteen legs and the fresh extraction gate standing attempted once each in the design's order with every tree copy refused across thirty verified write attempts and no tree standing assembled at its hand. The formatting proof stands executed at the build over the two constructed Rust texts in stdin mode at the crate's edition with the base texts as controls, confirmed at the standards review over the same texts, and executed at this closure over the constructed setup text with its head text as control. The warm up stands carried whole at the slice and stands executed whole at this closure in the form the evidence leaves it. The fresh extraction gate at the Round 36 slice head stands executed whole at 1,072 passed and 0 failed. carried-moved-categories stands executed whole at 1,057 passed and 15 failed over the census of 1,072, its fifteen falls standing at the leg's computed seats, the whole run the Round 36 closure owed standing discharged. dup-shapes-base stands executed whole at 1,055 passed and 15 failed, dup-categories-base at 1,054 passed and 16 failed, dup-shapes-slice at 1,051 passed and 21 failed, dup-categories-slice at 1,050 passed and 22 failed, cand-pin36 at exactly its fifteen falls and cand-status36 at exactly its three falls by name. moved-verdicts-base stands executed whole at 1,065 passed and 7 failed, its seven falls standing seat for seat at the design's enumeration with the tenth pin passing whole, dup-verdicts-base at 1,061 passed and 11 failed at its enumerated seats, moved-verdicts-slice at 1,066 passed and 8 failed with the tenth pin falling at its restated seat assertions, dup-verdicts-slice at 1,063 passed and 11 failed, cand-pin37 at exactly its seven falls with the tenth pin passing whole and cand-status37 at exactly its three falls by name, and the fresh extraction gate at this slice head stands executed whole at 1,074 passed and 0 failed, every set standing at its computed seat and the stopping rule standing unfired. The records items stand named in the open, the build record's section 2 standing and slice line figures standing at one Minor from each of the three reviews, its section 4 assembly lengths standing at one Observation from the standards review, the FIX-CK-27 record's two call site sentence standing overtaken at five call sites at this head, the hunt machinery's moved verdicts form standing corrected at this closure to the design's words after the carried form stood proven inert at its first execution, and the spec review's plain judgement on the carrying standing recorded at this closure, none of the items moving a verdict or the slice's texts. The round's obligations pin and its status pin stand in the family's form as charged, and the suite stands at 1074. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 38 has landed. It carries the verdicts closing bound in its slice under CK-60, the one standing pin restated to assert each set section's closing of the verdicts reading at the closing seat of its own slice and standing once in its slice, asserted once per section over the four sections of its pilot fixture beside the closing counts the pin already asserts, and no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 38 pins. The obligations pin fails
/// against the tree Round 37 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_38_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains(
        "## Round 38 obligations, the remaining verdicts closings bound in their slices"
    ));
    assert!(text.contains("- CK-61."));
    let ck61 = text
        .lines()
        .find(|line| line.starts_with("- CK-61."))
        .unwrap_or("");
    assert!(ck61.contains("stands asserted at the closing seat of its own slice"));
    assert!(ck61.contains("sixteen, sixteen, four and six in the order the files first stand"));
}

#[test]
fn readme_reports_the_round_38_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-61 met. It stands in full form in CONTRACT.md, and Round 38 closed on the merged slice 39 after three reviews and the owner's verification, with the spec review approving the slice with no findings charged against it, one Major charged against the design record and one Minor and three Observations charged against the build record, the standards review approving the slice with one Observation and withholding on the design record at the same Major, and the red team approving the slice with no findings charged against it, one Major and two Observations charged against the design record and one Minor charged against the build record, its tables re-derived seat by seat at all thirty two sections, its executed sets re-parsed name by name from the banked logs, and three of the legs the build carried standing executed at its own hand at exactly their stated sets, and CodeRabbit standing with no inline comments at any of the three reviews' reads, its one nitpick standing in its summary alone and standing declined at the standards review on the pin's standing form. CK-61 stands met by the restatement of the one standing pin in rust/tests/test_cli.rs, each section's closing of the choices, grounds, judgements, standings and models readings standing asserted at the closing seat of its own slice and to stand once among the lines of its own slice, asserted once per section over the sixteen sections of each of its two fixtures beside the closing counts the pin already asserts, sixty four assertions standing added and no assertion the pin already carries moving, the pin's comment standing restated at its witness sentence alone, and no rendering over any collection moving, and no production code, no fixture and no golden moving anywhere in the round. The slice stands as pull request 105 merged at bc12fbf0, with main's tree standing verified equal to the slice tree e686efbc at this closure firsthand, and the census standing at 1076. The design took the closings of the five remaining readings of the verdicts form at the one standing pin over both of its fixtures, computed the section totals of the five readings from the verified census example and cross checked them whole against the pin's own closing counts at three independent seats, priced its discrimination splits pin by pin over the whole suite over the base tree, the slice tree and the candidate legs, desk checked the status step against the shipped family form at the base head, and charged the construction in the family's commit order, the categories head order standing declined a ninth time on its verified unmoved evidence, the states reading standing priced outside the obligation on the computed parting of its two grains, FIX-CK-27 standing declined again with all three of its reopen triggers standing unmet and verified at five call sites at the head, and no design time mutation execution standing attempted at its seating, its splits standing computed and charged to the build's warm up. The build stopped at its premise on the design's dropped dash, the owner ruled the family's shipped form governs, and the slice stands seated dash led at the form contract of 355,679 characters, the build then verifying the premise whole over all 171 files of the base tree and constructing the slice in seven commits in the family's order, the expressly empty steps standing proven by tree hash identity at every stage, the status pin standing in the family form with its three assertions, the slice phrase standing at exactly one place in the slice README, that place the slice line alone, and its warm up standing executed in part and carried in part under the write fault, nine legs standing executed at their computed seats and seventeen standing carried named with their attempts counted. The formatting proof stands executed at the build over the constructed Rust texts in stdin mode at the crate's edition with the base texts as controls, confirmed at the standards review over the same texts, and executed at this closure over the constructed setup text with its head text as control. The warm up stands executed in part and carried in part at the slice and stands executed whole at this closure in the form the evidence leaves it. The fresh extraction gate at this slice head stands executed whole at 1,076 passed and 0 failed. moved-choices-base stands executed whole at 1,069 passed and 5 failed, dup-choices-base at 1,064 passed and 10 failed, moved-grounds-base at 1,068 passed and 6 failed, dup-grounds-base at 1,060 passed and 14 failed, moved-judgements-base at 1,068 passed and 6 failed, dup-judgements-base at 1,060 passed and 14 failed, moved-standings-base at 1,069 passed and 5 failed, dup-standings-base at 1,062 passed and 12 failed, moved-models-base at 1,069 passed and 5 failed and dup-models-base at 1,062 passed and 12 failed, moved-choices-slice at 1,070 passed and 6 failed, dup-choices-slice at 1,065 passed and 11 failed, moved-grounds-slice at 1,069 passed and 7 failed, dup-grounds-slice at 1,061 passed and 15 failed, moved-judgements-slice at 1,069 passed and 7 failed, dup-judgements-slice at 1,061 passed and 15 failed, moved-standings-slice at 1,070 passed and 6 failed, dup-standings-slice at 1,063 passed and 13 failed, moved-models-slice at 1,070 passed and 6 failed and dup-models-slice at 1,063 passed and 13 failed, cand-pin38-choices at exactly its five falls with the one standing pin passing whole, cand-pin38-grounds at exactly its six falls, cand-pin38-judgements at exactly its six falls, cand-pin38-standings at exactly its five falls, cand-pin38-models at exactly its five falls and cand-status38 at exactly its three falls by name, every executed set standing at its computed seat, no executed set parting from its stated set at any seat, and the stopping rule standing unfired. The records items stand named in the open, the design record's dropped dash and its form figure of 355,677 standing corrected under the owner's ruling to the dash led form at 355,679 and riding the errata queue, the design record's first section file split standing at one Observation from the red team, the pin's tables carrying heads the shipped assertions bind by total alone standing at one Observation from the red team, the build record's carried name list standing one name short of its own count at one Minor from the spec review, the omitted leg standing taken from the design's list at this closure and standing executed at its stated set, the build record's carrying attribution of the five candidate legs standing at one Minor from the red team, the emitter defect it names standing cured at this closure by the red team's corrected copy with its one line exemption of the swapped path, the witness sentence's seating by tail rewrap and the parser and counter defects standing at Observations from the reviews, and the standards review's withhold standing against the design record as written and not against the round, none of the items moving a verdict or the slice's texts. The round's obligations pin and its status pin stand in the family's form as charged, and the suite stands at 1076. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 39 has landed. It carries the remaining verdicts closings bound in their slices under CK-61, the one standing pin restated to assert each section's closing of the choices, grounds, judgements, standings and models readings at the closing seat of its own slice and standing once in its slice, asserted once per section over the sixteen sections of each of its two fixtures beside the closing counts the pin already asserts, and no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 39 pins. The obligations pin fails
/// against the tree Round 38 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_39_obligations() {
    let text = read("CONTRACT.md");
    assert!(
        text.contains("## Round 39 obligations, the states closings bound at both of their grains")
    );
    assert!(text.contains("- CK-62."));
    let ck62 = text
        .lines()
        .find(|line| line.starts_with("- CK-62."))
        .unwrap_or("");
    assert!(ck62.contains("stands asserted at the closing seat of its own slice"));
    assert!(
        ck62.contains("twelve, sixteen, sixteen, four and six in the order the files first stand")
    );
}

#[test]
fn readme_reports_the_round_39_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-62 met. It stands in full form in CONTRACT.md, and Round 39 closed on the merged slice 40 after three reviews and the owner's verification, with the spec review approving the slice at one Minor, the warm up carried whole standing discharged on the build record's counted evidence with its charge passing whole to this closure, one Observation standing against the design record and one Minor against the build records, the standards review approving the slice at two Observations with no finding against the design record and one Minor and one Observation against the build records, the formatting proof standing re-executed whole at its hand in stdin mode, and the red team approving the slice at one Observation, the round's discrimination splits standing unexecuted at every hand with their first executions charged to this closure, one Minor standing against the design record by adoption and one Minor and one Observation against the build records, its grain claim standing re-derived at its own hand at fifty four, none and none over the pilot fixture and forty two, none and none over the constructed, and CodeRabbit standing with exactly one inline comment, a Minor proposing each file slice end at the next actual file head, standing dismissed at both the spec and the standards reviews on the obligation's own text and the pin's standing drive form. CK-62 stands met by the restatement of the one standing pin in rust/tests/test_cli.rs recursively in the family's two assertion form, each section's closing of the states reading standing asserted at the closing seat of its own slice and standing among the lines of its own slice at its stated count, one as the rule and two where one file alone stands in the section, and each file's closing standing asserted at the closing seat of its own file slice and standing once in its file slice, asserted once per section and once per file over the sections and files of each of its two fixtures beside the closing counts the pin already asserts, thirty four assertions standing added and no assertion the pin already carries moving, and no rendering over any collection moving, and no production code, no fixture and no golden moving anywhere in the round. The slice stands as pull request 109 merged at bce02b35, with main's tree standing verified equal to the repaired slice tree dc26792d at the owner firsthand, and the census standing at 1078. The design took the states closings bound at both of their grains, the states reading printing the same closing at two grains, the per file closing inside each section and the section closing after it, so the standing once form of the earlier rounds cannot hold over its slices, computed the section totals of the states reading at fifty four, none and none over the pilot fixture and forty two, none and none over the constructed, priced its discrimination splits pin by pin over the whole suite over the base tree, the slice tree and the candidate legs, desk checked the status step against the shipped family form at the slice head, and charged the construction in the family's commit order, the categories head order standing declined a tenth time on its verified unmoved evidence, FIX-CK-27 standing declined again with all three of its reopen triggers standing unmet and verified at five call sites at the head, and no design time mutation execution standing attempted at its seating, its splits standing computed and charged to the build's warm up. The build constructed the slice in seven commits in the family's order, the expressly empty steps standing proven by tree hash identity at every stage, the status pin standing in the family form with its three assertions, the slice phrase standing at exactly one place in the slice README, that place the slice line alone, and its warm up standing carried whole under the write fault at one hundred and twenty counted attempts with cargo never reached on any attempt, charged whole to this closure. The slice's first head stood uncompilable at three missing lifetime errors at the one type alias of the restated pin, the cure standing taken in slice at one line, the alias carrying one named lifetime, no assertion, count, figure or fixture moving and the census standing unmoved, the repaired head standing proven locally at format, clippy and the whole suite and standing green at both hosted gates. The formatting proof stands re-executed at the standards review over the slice's constructed texts in stdin mode at the crate's edition with the base texts as controls, and executed at this closure over the constructed setup text with its head text as control. The warm up stands carried whole at the slice and stands executed whole at this closure in the form the evidence leaves it, no leg standing carried out of this closure. The fresh extraction gate at this slice head stands executed whole at 1,078 passed and 0 failed. plain-base stands executed whole at 1,076 passed and 0 failed and plain-slice at 1,078 passed and 0 failed, moved-section-base stands executed whole at 1,069 passed and 7 failed, dup-section-base at 1,063 passed and 13 failed, moved-file-base at 1,071 passed and 5 failed and dup-file-base at 1,064 passed and 12 failed, moved-section-slice at 1,070 passed and 8 failed, dup-section-slice at 1,064 passed and 14 failed, moved-file-slice at 1,072 passed and 6 failed and dup-file-slice at 1,065 passed and 13 failed, cand-pin39-moved-section at exactly its seven falls with the one standing pin passing whole, cand-pin39-dup-section at exactly its thirteen falls, cand-pin39-moved-file at exactly its five falls, cand-pin39-dup-file at exactly its twelve falls and cand-status39 at exactly its three falls by name, one fault failure standing beside its set, a fixture write refused inside a verdicts reading test with no assertion of that test standing reached, its assertion set standing read from assertion failures alone at the computed three, every executed set standing at its computed seat, no executed set parting from its stated set at any seat in substance, and the stopping rule standing unfired. The records items stand named in the open, the Round 38 design record's eighth section sentence stating the states section closings at the judgements partition's totals standing refuted by this round's computation and standing at one Minor against that record from the design and the red team, the design record's seventh section figure for the slice phrase standing at one Observation from the spec review, the build record's third section figure for the in memory cli form standing at one Minor from the spec and the standards reviews, the build's first head standing shipped uncompilable at one Minor against the build record from the red team, and the two judge instrument items of this closure, the filename extension standing unstripped at the moved section leg's first reading and the operating system error prose standing unread at the cand-status39 leg's first reading, standing cured and read by the standing rule at this closure and standing named whole in its report, none of the items moving a verdict or the slice's texts. The round's obligations pin and its status pin stand in the family's form as charged, and the suite stands at 1078. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 40 has landed. It carries the states closings bound at both of their grains under CK-62, the one standing pin restated to assert each section's closing of the states reading at the closing seat of its own slice and standing among the lines of its own slice at its stated count, and each file's closing at the closing seat of its own file slice and standing once in its file slice, asserted once per section and once per file over the sections and files of each of its two fixtures beside the closing counts the pin already asserts, and no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}

/// The Round 40 pins. The obligations pin fails
/// against the tree Round 39 closed on by the
/// absence of the obligations it reads, and the
/// status pin fails on that tree's README by the
/// absence of the two lines it holds, each line
/// read whole as the standing pins read theirs.
#[test]
fn contract_carries_the_round_40_obligations() {
    let text = read("CONTRACT.md");
    assert!(text.contains(
        "## Round 40 obligations, the flags and outcomes closings bound in their slices"
    ));
    assert!(text.contains("- CK-63."));
    let ck63 = text
        .lines()
        .find(|line| line.starts_with("- CK-63."))
        .unwrap_or("");
    assert!(ck63.contains("stands asserted at the closing seat of its own slice"));
    assert!(ck63.contains("thirty flagged lines and two contested"));
}

#[test]
fn readme_reports_the_round_40_status() {
    let text = read("README.md");
    let standing = "- Contract, CK-63 now stands in full form in CONTRACT.md, and Round 40 stands open on it, its slice standing charged in the family's commit order and not yet built, the categories head order standing declined an eleventh time on its verified unmoved evidence, the flags and outcomes closings standing taken into the obligation together on the computed sections of the pin's two fixtures, FIX-CK-27 standing declined again with all three of its reopen triggers standing unmet and verified at five call sites at the head, and Round 40 has not closed.";
    assert!(text.lines().any(|line| line == standing));
    let slice_line = "- Slice 41 stands built and in review. It carries the flags and outcomes closings bound in their slices under CK-63, the one standing pin restated to assert each section's closing pair of the flags reading standing as the last two lines of its own slice in its printed order and each of the pair's two lines standing once in its slice, and each section's closing of the outcomes reading standing at the closing seat of its own slice and standing once in its slice, asserted once per section over the sections of each of its two fixtures beside the closing counts the pin already asserts, and no rendering over any collection moving.";
    assert!(text.lines().any(|line| line == slice_line));
    assert!(text.contains("Closed perfection rounds, 39 of 100"));
}
