//! The stickler library. A strict reviewer of code and naming.
//!
//! The library carries the loaders, the checker, the seat readers,
//! the readings, and the renderers as pure functions. The binary
//! carries the command line surface over the library. The Python
//! package under src is the specification this port reproduces.

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
