# Stickler FAQ

## Does Stickler modify my source files or open network connections?

No. `stickler` is strictly read-only and 100% offline. It never rewrites files on disk and never calls external LLM APIs itself. Model calls happen in your outer harness, which hands recorded envelopes or change set JSON files to `stickler` for deterministic scoring and reporting.

## Why does Stickler have no configuration file or `.stickler.toml`?

By contract (`AGENTS.md`), `stickler` enforces six fixed built-in Python shapes and three fixed borderline patterns rather than a configurable plugin tree. Every repository using `stickler` shares the exact same review bar with zero dotfile drift.

## Can I run `stickler check` on TypeScript, Rust, or Go diffs?

Scope `stickler check` to Python paths (`git diff -- '*.py'`). As `FIX-CK-24` notes in `CONTRACT.md`, the checker inspects Python syntax forms without verifying file extensions, so a non-Python diff either produces a vacuous `accepted` review when no Python token pattern appears or matches surface tokens (`print(...)`) out of context.

## What is the difference between `stickler` and `grandjury`?

`stickler` is purpose-built for Python diff review. It scans unified diffs for six static code smells, detects three line-level borderline patterns, and joins recorded `stickler` vs `clean` verdicts onto raw diff line numbers. `grandjury` is a general-purpose multi-model poll aggregator for arbitrary discrete-choice questions.

## What do the exit codes mean?

- **`0`** Clean diff on `check`, `accepted` review on `render`, clean change set on `review` (zero static judgements and zero borderline flags judged `stickler`), or valid reading on the other eighteen commands.
- **`1`** Findings stand (`sent back` on `check` or `render`, or at least one static judgement or upheld `stickler` borderline flag on `review`), or container depth overflow (`> 9997`) on `render`.
- **`2`** Unreadable, non-UTF-8, invalid-JSON, or schema-malformed input file (printing structured defects on `stdout`), or invalid CLI arguments (printing usage on `stderr`).
- **`120`** Closed output pipe on `stdout` or `stderr`.
