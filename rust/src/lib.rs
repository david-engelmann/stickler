//! Stickler reviews unified diffs against six built in Python smell rules,
//! separates borderline candidates for LLM verdict arbitration, and renders
//! deterministic readings over recorded change sets and collections.
//!
//! All loaders, checks, seat readers, readings, and renderers stand as pure
//! functions in this library. Nothing in the crate writes to inspected files
//! or opens a network socket. The command line binary in `cli` is a thin
//! dispatch shell over these modules.

pub mod borderline;
pub mod categories;
pub mod census;
pub mod changeset;
pub mod checker;
pub mod choices;
pub mod cli;
pub mod contested;
pub mod files;
pub mod flags;
pub mod grounds;
pub mod json;
pub mod judgements;
pub mod models;
pub mod outcomes;
pub mod pattern;
pub mod pycompat;
pub mod rejudged;
pub mod render;
pub mod review;
pub mod scan;
pub mod sets;
pub mod shapes;
pub mod split;
pub mod standings;
pub mod states;
pub mod verdicts;
pub mod verdicts_reading;
