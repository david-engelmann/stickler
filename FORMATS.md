# stickler formats

This reference documents every file format `stickler` reads in one place for external tools, CI hooks, and multi-agent orchestrators. The specification of record stands in `CONTRACT.md` under `## The formats, version 1` (`FIX-CK-24` and `FIX-CK-29`). Where this reference and `CONTRACT.md` differ, `CONTRACT.md` governs.

Every format below stands at version 1 in `CONTRACT.md` (`FIX-CK-24`). No literal `"version"` field is required or inspected in any JSON file, and every JSON loader ignores extra fields not named by its shape. Under `PORT-3` and `FIX-CK-31`, the JSON parser enforces a container nesting limit of `9997` open arrays or objects, rejects integer literals exceeding `4300` digits, and detects unpaired UTF-16 surrogates.

## Unified diff input (`check`)

`stickler check <diff-file>` reads a UTF-8 unified diff text file (`CK-3`, `CK-4`).

- Line numbers are **1-based raw line indices** from the start of the diff text (`line 1`, `line 2`, and so on).
- Hunk headers (`@@ ... @@`) clear the tracked Python `class` indentation stack (`CK-5`) and advance the raw diff line index like every other line rather than resetting line coordinates.
- Added lines begin with `+` (excluding `+++ ` file headers) and are inspected against the six static objection shapes and three borderline patterns.
- Context lines begin with a single space and contribute `class` body indentation context to subsequent added lines in the same hunk.
- Removed lines begin with `-` (excluding `--- ` file headers) and draw no objections.

```diff
--- a/orders.py
+++ b/orders.py
@@ -1,3 +1,6 @@
 def load_orders(items=[]):
+    foo = fetch_all()
+    try:
+        return foo
+    except:
+        return items
```

## The review form (`render`)

`stickler render <review.json>` reads one recorded review object at version 1 (`CK-1`, `CK-2`, `FIX-CK-22`, `FIX-CK-24`).

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `diff_label` | string | required | Non-empty single-line string without unpaired surrogates |
| `objections` | array of objects | required | Zero or more objection entries |
| `objections[].category` | string | required | One of `naming`, `structure`, or `smell` |
| `objections[].flaw` | string | required | Non-empty string without unpaired surrogates, fenced to one line at rendering |
| `objections[].rewrite` | string | required | Non-empty replacement string without unpaired surrogates, fenced to one line at rendering |
| `objections[].standard` | string | required | Non-empty principle string without unpaired surrogates, fenced to one line at rendering |

```json
{
  "diff_label": "orders change, recorded review",
  "objections": [
    {
      "category": "naming",
      "flaw": "The added line binds the order rows to the placeholder name foo",
      "rewrite": "rows = load_rows()",
      "standard": "A name says what its value holds"
    }
  ]
}
```

## The change set form (`review`)

`stickler review <set.json>` reads one recorded change set object at version 1 (`CK-5`, `CK-6`, `CK-7`, `CK-8`, `FIX-CK-24`).

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `files` | array of objects | required | Zero or more file diff entries (`file` names may repeat across entries) |
| `files[].file` | string | required | Non-empty single-line file path without unpaired surrogates |
| `files[].diff` | string | required | Unified diff text for that entry without unpaired surrogates |
| `files[].verdicts` | array of objects | optional | Recorded model verdicts on borderline lines of `diff` (each `(line, model)` pair must be unique within a single `files[]` entry, while distinct models may record verdicts on the same line) |
| `verdicts[].line` | integer | required | 1-based raw line number inside `diff` naming a borderline line |
| `verdicts[].pattern` | string | required | Matching pattern (`near placeholder name`, `broad handler`, or `shared member default`) |
| `verdicts[].model` | string | required | Non-empty single-line model identifier |
| `verdicts[].choice` | string | required | Either `stickler` (flag upheld) or `clean` (flag dismissed) |
| `verdicts[].confidence` | number | required | Finite number in `[0.0, 1.0]` (`< 0.5` abstains on that verdict) |

```json
{
  "files": [
    {
      "file": "harvest.py",
      "diff": "--- a/harvest.py\n+++ b/harvest.py\n@@ -1,3 +1,5 @@\n+data = load_entries()\n def first_entry():\n     return data[0]\n",
      "verdicts": [
        {
          "line": 4,
          "pattern": "near placeholder name",
          "model": "jev-1.13.0",
          "choice": "stickler",
          "confidence": 0.72
        },
        {
          "line": 4,
          "pattern": "near placeholder name",
          "model": "openai/gpt-6-luna",
          "choice": "clean",
          "confidence": 0.81
        }
      ]
    }
  ]
}
```

## The split form (`split`)

`stickler split <split.json>` reads one two-route split wrapper object at version 1 (`CK-15`, `FIX-CK-24`).

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `routes` | array of 2 strings | required | Two distinct non-empty single-line model route identifiers |
| `change_set` | object | required | Valid inline change set object in the shape above where every recorded verdict `model` equals one of the two `routes` strings |

```json
{
  "routes": [
    "jev-1.13.0",
    "openai/gpt-6-luna"
  ],
  "change_set": {
    "files": []
  }
}
```

## The rejudged pair form (`rejudged`)

`stickler rejudged <rejudged.json>` reads one two-set comparison object at version 1 (`CK-14`, `FIX-CK-24`), pairing borderline flags across the two sets by matching `(file, line, pattern)` keys in first-set order.

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `sets` | array of 2 objects | required | Exactly two labelled change set entries |
| `sets[].label` | string | required | Non-empty single-line set label |
| `sets[].change_set` | object | required | Valid inline change set object |

```json
{
  "sets": [
    {
      "label": "pilot run",
      "change_set": {
        "files": []
      }
    },
    {
      "label": "rejudged run",
      "change_set": {
        "files": []
      }
    }
  ]
}
```

## The collection form (`census` and the fifteen collection commands)

Sixteen commands (`census`, `contested`, `pattern`, `files`, `borderline`, `outcomes`, `verdicts`, `shapes`, `categories`, `choices`, `grounds`, `judgements`, `standings`, `states`, `flags`, and `models`) read a collection object at version 1 (`CK-11`, `CK-12`, `FIX-CK-24`, `FIX-CK-29`). Every member change set is embedded inline with no external file paths.

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `sets` | array of objects | required | Zero or more labelled change set entries (`label` strings may repeat) |
| `sets[].label` | string | required | Non-empty single-line set label |
| `sets[].change_set` | object | required | Valid inline change set object |

```json
{
  "sets": [
    {
      "label": "pilot set",
      "change_set": {
        "files": [
          {
            "file": "tidy.py",
            "diff": "--- a/tidy.py\n+++ b/tidy.py\n@@ -1,2 +1,3 @@\n def render(total):\n+    logger.debug(total)\n     return total\n"
          }
        ]
      }
    }
  ]
}
```

## Verdict route envelopes and run records (`stickler::verdicts`)

The library module `stickler::verdicts` parses raw response envelopes returned by two LLM decision routes (`CK-7`), and pilot judging runs record call accounting in the run record form (`FIX-CK-24`).

- **Structured decision route (`answers.decision`).** JSON object carrying `"model"` (non-empty string) and `"answers": {"decision": {"choice": "stickler" | "clean", "confidence": 0.0..1.0, "probabilities": {"stickler": <finite>, "clean": <finite>}}}`.
- **Chat completions route (`choices[0].message.content`).** JSON object carrying `"model"` (non-empty string) and `"choices"` whose first entry holds `"message": {"content": "<serialized JSON decision object with choice, confidence, and probabilities>"}`.
- **Run record form.** JSON object carrying `"routes"`, an array of per-route accounting objects recording attempted calls, recorded verdicts, failed calls, and token or spend totals (`examples/verdicts/run_record.json`).
