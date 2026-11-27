# Stickler User Guide

Stickler checks Python unified diffs against six built-in code and naming rules, isolates three borderline patterns where static rules overreach, and provides twenty offline rendering and analytical commands alongside `check`.

## Installation

### Prebuilt binary (Linux and macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/stickler/main/install.sh | sh
```

### Cargo Binstall

```sh
cargo binstall stickler
```

### Build from source

```sh
git clone https://github.com/david-engelmann/stickler.git
cd stickler
cargo install --path . --bin stickler --locked
```

## 60-Second Walkthrough

Check a Python unified diff, render a single recorded review, inspect a multi-file change set carrying recorded LLM verdicts, compare routes with `split` and `rejudged`, and summarize a multi-set collection.

```sh
stickler check examples/smelly.diff
stickler render examples/recorded_review.json
stickler review examples/borderline_set.json
stickler split examples/split.json
stickler rejudged examples/rejudged.json
stickler contested examples/contested.json
stickler census examples/census.json
```

`stickler check` exits with status `0` when the diff is clean and status `1` when objections stand (`sent back`), making it a drop-in pre-review gate in shell scripts and CI pipelines.

## Six Static Shapes and Three Borderline Patterns

### Deterministic objections (`check` and `review`)

| Category | Shape | What triggers it on an added Python line |
| --- | --- | --- |
| `smell` | `bare except` | `except:` clause without naming an exception type |
| `smell` | `print call` | Added line calling `print(...)` |
| `structure` | `mutable default` | Function parameter defaulting to `[]` or `{}` |
| `structure` | `call default` | Function parameter defaulting to a runtime call expression such as `now()` |
| `structure` | `class attribute` | Mutable list, dict, or set literal bound directly on a `class` body |
| `naming` | `placeholder name` | Assignment, annotated target, tuple target, or `def` bound to `data2`, `temp2`, `foo`, `bar`, `stuff`, or `thing` |

### Borderline patterns escalated to model seats

| Pattern | Exact trigger | Why models arbitrate it (`stickler` vs `clean`) |
| --- | --- | --- |
| `near placeholder name` | Target or `def` named `data`, `temp`, or `tmp` | Clear in small local scopes, vague in wide domain scopes |
| `broad handler` | `except BaseException:` or `except BaseException as <name>:` | Valid at top-level worker harnesses, overly broad inside library helpers (`except Exception:` is accepted cleanly) |
| `shared member default` | Tuple default containing a mutable inner literal such as `cache=([],)` | Outer tuple is immutable while inner element remains shared mutable state |

## Command Reference Matrix (21 Commands)

Every `stickler` subcommand takes **exactly one** file path argument and writes deterministic UTF-8 text to `stdout`. All input shapes stand documented in [../FORMATS.md](../FORMATS.md).

### Single diff, review, change set, and comparison commands

| Command | Argument | Input format | Purpose |
| --- | --- | --- | --- |
| `check` | `<diff>` | Unified `.diff` | Check added Python lines against the six static shapes (exits `1` when `sent back`) |
| `render` | `<review.json>` | Review JSON | Render a recorded review object in objection blocks (exits `1` when `sent back`) |
| `review` | `<set.json>` | Change set JSON | Combine static objections, borderline verdict outcomes, and contested flags over one change set |
| `split` | `<split.json>` | Split JSON (`routes` + `change_set`) | State each borderline flag's outcome under route 1 alone, route 2 alone, and both routes jointly |
| `rejudged` | `<rejudged.json>` | Rejudged pair JSON (`sets` of length 2) | Ledger of borderline flags whose standing shifted or held between two judging passes |

### Multi-set collection commands (all take `<collection.json>`)

| Command | Analytical axis | Section structure |
| --- | --- | --- |
| `census` | Whole-collection overview | Per-set summary blocks (`Files`, `Judgements`, `Flags`, outcomes, `Contested`) plus grand totals |
| `contested` | Inter-model disagreement | One section per set naming flags where recorded model verdicts split `stickler` vs `clean` |
| `borderline` | Raw candidate inspection | One section per set holding borderline flags with raw diff line number, line text, and shape reading |
| `pattern` | Borderline pattern | Three sections (`near placeholder name`, `broad handler`, `shared member default`) |
| `outcomes` | Final flag standing | Three sections (`Judged stickler`, `Judged clean`, `Unjudged`) |
| `files` | Source file | One section per file holding borderline flags in first-appearance order across the collection |
| `verdicts` | Recorded verdict pairs | One section per set listing recorded `(model, choice, confidence)` pairs |
| `shapes` | Static objection shape | Six sections covering each deterministic checker shape |
| `categories` | Static objection category | Three sections covering `naming`, `structure`, and `smell` |
| `choices` | Raw seat choice | Three sections (`Chose stickler`, `Chose clean`, `Abstained`) |
| `grounds` | Verdicts by pattern | Three pattern sections listing flags carrying recorded verdicts alongside their verdict pairs |
| `judgements` | Verdicts by standing | Three outcome sections listing flags carrying recorded verdicts alongside their verdict pairs |
| `standings` | Verdicted files | One section per file carrying recorded verdicts in first-appearance order, holding flag groups and pairs |
| `states` | Pooled file verdict state | Three sections (`Judged stickler`, `Judged clean`, `Unjudged`) holding file groups and verdict pairs |
| `flags` | Flags by pooled file state | Three pooled file state sections holding flag lines and per-section contested counts |
| `models` | Model route participation | One section per model identifier in first-appearance order across recorded verdicts |

## Multi-Agent Pipeline Integration Recipe

In an automated Python code-generation and review workflow, wire `stickler` as a two-tier diff gate so LLM tokens are spent only on genuine ambiguity.

1. **Scope diffs to Python files.** Generate a Python-only unified diff with `git diff origin/main...HEAD -- '*.py' > pr-python.diff` so non-Python files neither draw accidental token matches nor return a vacuous `accepted` review (`FIX-CK-24`).
2. **Tier 1 zero-cost static gate.** Run `stickler check pr-python.diff`. If it exits `1`, return the printed objections and rewrites directly to the authoring agent for immediate repair at zero token cost.
3. **Tier 2 borderline escalation.** Wrap the diff into a version 1 change set JSON (`FORMATS.md`), dispatch any borderline lines (`data`/`temp`/`tmp`, `except BaseException`, or tuple-wrapped mutable defaults) to two independent model seats, parse their raw envelopes with `stickler::verdicts` (`stickler` vs `clean`), and run `stickler review set.json` and `stickler contested collection.json` to escalate only upheld or contested lines.
