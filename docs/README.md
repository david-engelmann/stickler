<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/stickler-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/stickler-lockup-light.svg">
    <img alt="stickler" src="media/stickler-lockup-light.svg" width="380">
  </picture>
</p>

# Stickler Documentation

Deterministic Python unified-diff smell checker and recorded multi-model verdict analytics.

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [FAQ.md](FAQ.md) | Scope boundaries against sibling Spectacle tools, exit codes, determinism, and common troubleshooting |
| [../FORMATS.md](../FORMATS.md) | Complete version 1 JSON and text input schemas, field rules, and minimal examples |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `stickler` makes zero network calls, spawns no background daemons, and mutates none of the diffs or JSON files handed to it.
- **Exact rational thresholds.** Verdict confidence floors (`0.5`) and collection counts run without floating-point rounding drift.
- **Whole-input defect reporting.** Malformed JSON inputs return structured defect lists naming every reachable schema violation before printing a single report line.
- **Single-line output fencing.** Checked string fields reject unpaired UTF-16 surrogates and line breaks (`\n` and `\r`) so untrusted strings cannot forge extra report or summary lines, though operators displaying untrusted text directly in a terminal should still pipe output through a pager or sanitizer.
