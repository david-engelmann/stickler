# stickler

Stickler is a command line tool that reviews code against named rules, checks a diff with a built in checker, and prints readings over verdicts that decision model routes recorded while judging a change set against those rules.

Stickler takes one file per command and prints one rendering. Every command runs offline, and no command calls a model. The recorded verdicts the readings run over were produced in earlier rounds by the routes they name and stand committed under examples as fixtures.

[![ci](https://github.com/david-engelmann/stickler/actions/workflows/ci.yml/badge.svg)](https://github.com/david-engelmann/stickler/actions/workflows/ci.yml)

```text
Pattern

Pattern, near placeholder name
harvest.py, line 6, set pilot set
harvest.py, line 7, set pilot set
sorting.py, line 6, set pilot set
sorting.py, line 11, set pilot set
sorting.py, line 12, set pilot set
ledger.py, line 6, set constructed round 4 example
ledger.py, line 7, set constructed round 4 example
cache.py, line 15, set constructed round 4 example
cache.py, line 19, set constructed round 4 example
Flags read, 9
Contested, 2

Pattern, broad handler
harvest.py, line 12, set pilot set
harvest.py, line 18, set pilot set
sorting.py, line 15, set pilot set
cache.py, line 10, set constructed round 4 example
Flags read, 4
Contested, 0

Pattern, shared member default
windows.py, line 6, set pilot set
windows.py, line 10, set pilot set
windows.py, line 14, set pilot set
windows.py, line 18, set pilot set
cache.py, line 7, set constructed round 4 example
Flags read, 5
Contested, 0

Sets read, 3
Flags read, 18
Contested flags, 2
```

## Install

```sh
git clone https://github.com/david-engelmann/stickler.git
cd stickler
cargo install --path .
```

## Quickstart

Run the pattern command over the pattern example from the root of the clone.

```sh
stickler pattern examples/pattern.json
```

```text
Pattern

Pattern, near placeholder name
harvest.py, line 6, set pilot set
harvest.py, line 7, set pilot set
sorting.py, line 6, set pilot set
sorting.py, line 11, set pilot set
sorting.py, line 12, set pilot set
ledger.py, line 6, set constructed round 4 example
ledger.py, line 7, set constructed round 4 example
cache.py, line 15, set constructed round 4 example
cache.py, line 19, set constructed round 4 example
Flags read, 9
Contested, 2

Pattern, broad handler
harvest.py, line 12, set pilot set
harvest.py, line 18, set pilot set
sorting.py, line 15, set pilot set
cache.py, line 10, set constructed round 4 example
Flags read, 4
Contested, 0

Pattern, shared member default
windows.py, line 6, set pilot set
windows.py, line 10, set pilot set
windows.py, line 14, set pilot set
windows.py, line 18, set pilot set
cache.py, line 7, set constructed round 4 example
Flags read, 5
Contested, 0

Sets read, 3
Flags read, 18
Contested flags, 2
```

## Usage

The twenty one commands each read one file and print one rendering. The examples below run over the files under examples, and every output block is captured from the command it follows.

The input formats are specified in CONTRACT.md, in the section The formats, version 1, which states each form completely with its literal keys, the review form of CK-1 that render reads, the unified diff as text under CK-3 and CK-5 that check reads, the change set form of CK-6 and CK-8 that review reads, the collection form of CK-12 that the collection commands read, the pair form of CK-14 that rejudged reads, the split form of CK-15 that split reads, the collection form under CK-27 that verdicts reads, the collection form under CK-29 that shapes reads, the collection form under CK-31 that choices reads, the collection form under CK-33 that categories reads, the collection form under CK-35 that grounds reads, and the collection form under CK-37 that judgements reads, and the collection form under CK-39 that standings reads, and the collection form under CK-41 that states reads, and the collection form under CK-43 that flags reads, and the collection form under CK-45 that models reads.

The check command reads a unified diff and builds a review with the built in checker. The checker holds a fixed set of rules over the shapes of code a diff can carry, and each objection it raises names the rule the code breaks and the change that would clear the objection. The six rules read Python shaped code, a bare except clause, a def line default, a print call and their kin, and a diff of another language's code raises nothing and reads as an empty accepted review, so the silence a stranger meets is a scope fact and not a judgement of their code.

```sh
stickler check examples/smelly.diff
```

```text
Review of smelly.diff
Verdict, sent back
Objections, 2

Objection 1, smell
Flaw, The added line catches every exception with a bare except clause
Standard, A handler names the failures it expects so the failures nobody expected still rise where they can be seen
Rewrite,     except Exception:

Objection 2, smell
Flaw, The added line sends diagnostics through a print call
Standard, Diagnostics travel through the logging module so they can be levelled, filtered, and silenced
Rewrite,         logger.debug(total)
```

The render command reads a review recorded as JSON and prints the review rendering from the record.

```sh
stickler render examples/recorded_review.json
```

```text
Review of orders change, recorded review
Verdict, sent back
Objections, 2

Objection 1, naming
Flaw, The added line binds the order rows to the placeholder name foo
Standard, A name says what its value holds so the reader never has to hold the assignment in memory to follow the code
Rewrite, rows = load_rows()

Objection 2, smell
Flaw, The added line catches every exception with a bare except clause
Standard, A handler names the failures it expects so the failures nobody expected still rise where they can be seen
Rewrite,     except Exception:
```

The review command reads a change set. A change set is a list of files, each entry naming a file and holding its diff, and the checker computes the judgements and the borderline flags from the diffs. A set may carry verdicts recorded on its flags by decision model routes through the verdict seat. The set review prints the judgement of each flag, the lines where the routes that judged one flag disagree as the contested lines, and a summary tally of the verdicts.

```sh
stickler review examples/change_set.json
```

```text
Change set review
Files, 3

File, basket.py
Judgements, 2

Judgement 1, structure
Flaw, The added line gives the class attribute seen a mutable literal that every instance shares
Standard, A mutable value that belongs to each instance is built in the initializer, so one instance changing it never changes it for every other instance
Rewrite, Move seen into the initializer as self.seen = [] and remove the class level assignment

Judgement 2, structure
Flaw, The added def line gives a parameter a default that is a call, so the call runs once when the definition is read and every call that omits the argument shares the one result
Standard, A default that must be computed on each call is written as None and the value is built inside the body under a None check
Rewrite,     def stamp(self, at=None): and build each replaced default inside the body under a None check

File, report.py
Judgements, 3

Judgement 1, structure
Flaw, The added def line gives a parameter a mutable default that every call shares
Standard, A default that must be fresh on each call is written as None and the value is built inside the body
Rewrite, def render(total, history=None):

Judgement 2, smell
Flaw, The added line catches every exception with a bare except clause
Standard, A handler names the failures it expects so the failures nobody expected still rise where they can be seen
Rewrite,     except Exception:

Judgement 3, smell
Flaw, The added line sends diagnostics through a print call
Standard, Diagnostics travel through the logging module so they can be levelled, filtered, and silenced
Rewrite,         logger.debug(total)

File, tidy.py
Clean

Summary
Judgements, 5
Shape, bare except, 1
Shape, print call, 1
Shape, mutable default, 1
Shape, call default, 1
Shape, class attribute, 1
Files with judgements, worst first
Worst, report.py, 3
Worst, basket.py, 2
```

The census command reads a census file that names a collection of recorded review inputs and prints the judging census. The census gives a tally of the judgements for each set in the collection and a total tally over the whole collection, counting files, judgements, and verdicts.

```sh
stickler census examples/census.json
```

```text
Census
Sets, 3

Set, round 2 set
Files, 3
Judgements, 5
Shape, bare except, 1
Shape, print call, 1
Shape, mutable default, 1
Shape, placeholder name, 0
Shape, call default, 1
Shape, class attribute, 1
Borderline, 0
Pattern, near placeholder name, 0
Pattern, broad handler, 0
Pattern, shared member default, 0
Verdicts, none recorded

Set, pilot set
Files, 3
Judgements, 0
Shape, bare except, 0
Shape, print call, 0
Shape, mutable default, 0
Shape, placeholder name, 0
Shape, call default, 0
Shape, class attribute, 0
Borderline, 12
Pattern, near placeholder name, 5
Pattern, broad handler, 3
Pattern, shared member default, 4
Verdict lines, stickler 11, clean 0
Unjudged, 1
Contested, 0

Set, constructed round 4 example
Files, 2
Judgements, 0
Shape, bare except, 0
Shape, print call, 0
Shape, mutable default, 0
Shape, placeholder name, 0
Shape, call default, 0
Shape, class attribute, 0
Borderline, 6
Pattern, near placeholder name, 4
Pattern, broad handler, 1
Pattern, shared member default, 1
Verdict lines, stickler 3, clean 2
Unjudged, 1
Contested, 2

Total
Sets, 3
Files, 8
Judgements, 5
Shape, bare except, 1
Shape, print call, 1
Shape, mutable default, 1
Shape, placeholder name, 0
Shape, call default, 1
Shape, class attribute, 1
Borderline, 18
Pattern, near placeholder name, 9
Pattern, broad handler, 4
Pattern, shared member default, 5
Verdict lines, stickler 14, clean 2
Unjudged, 2
Contested, 2
```

The rejudged command reads a file that names two recorded sets of verdicts over the same flags and prints the rejudged ledger. The ledger states the judgement of each flag in the first set and in the second set, and whether that judgement held or changed between the two sets.

```sh
stickler rejudged examples/rejudged.json
```

```text
Rejudged
First, pilot set
Second, rejudged set

harvest.py, line 6, near placeholder name
First, unjudged
Second, stickler
Changed

harvest.py, line 7, near placeholder name
First, stickler
Second, stickler
Held

harvest.py, line 12, broad handler
First, stickler
Second, stickler
Held

harvest.py, line 18, broad handler
First, stickler
Second, stickler
Held

windows.py, line 6, shared member default
First, stickler
Second, stickler
Held

windows.py, line 10, shared member default
First, stickler
Second, stickler
Held

windows.py, line 14, shared member default
First, stickler
Second, stickler
Held

windows.py, line 18, shared member default
First, stickler
Second, stickler
Held

sorting.py, line 6, near placeholder name
First, stickler
Second, stickler
Held

sorting.py, line 11, near placeholder name
First, stickler
Second, unjudged
Changed

sorting.py, line 12, near placeholder name
First, stickler
Second, stickler
Held

sorting.py, line 15, broad handler
First, stickler
Second, stickler
Held

Inputs read, 24
Standing in both, 12
Held, 10
Changed, 2
```

The split command reads a file that names one recorded set and two routes and prints the split reading. The split reading states the judgement of each flag under the first route alone, under the second route alone, and under both routes jointly, and it closes with counts of the flags read, the flags where the routes agree, the flags where they differ, and the flags where the joint judgement differs from a route.

```sh
stickler split examples/split.json
```

```text
Split
First route, jev-1.13.0
Second route, openai/gpt-6-luna

harvest.py, line 6, near placeholder name
First route, unjudged
Second route, unjudged
Joint, unjudged

harvest.py, line 7, near placeholder name
First route, stickler
Second route, unjudged
Joint, stickler

harvest.py, line 12, broad handler
First route, stickler
Second route, stickler
Joint, stickler

harvest.py, line 18, broad handler
First route, stickler
Second route, stickler
Joint, stickler

windows.py, line 6, shared member default
First route, stickler
Second route, stickler
Joint, stickler

windows.py, line 10, shared member default
First route, stickler
Second route, stickler
Joint, stickler

windows.py, line 14, shared member default
First route, stickler
Second route, stickler
Joint, stickler

windows.py, line 18, shared member default
First route, stickler
Second route, stickler
Joint, stickler

sorting.py, line 6, near placeholder name
First route, stickler
Second route, stickler
Joint, stickler

sorting.py, line 11, near placeholder name
First route, unjudged
Second route, stickler
Joint, stickler

sorting.py, line 12, near placeholder name
First route, stickler
Second route, stickler
Joint, stickler

sorting.py, line 15, broad handler
First route, stickler
Second route, stickler
Joint, stickler

Flags read, 12
Routes agree, 10
Routes differ, 2
Joint differs from a route, 2
```

The contested command reads a collection in the census form and prints the contested reading. The reading states one section per set in collection order, naming each contested flag with its file, line, and pattern, or the no contest line where a set holds none, and it closes with counts over the whole collection.

```sh
stickler contested examples/contested.json
```

```text
Contested

Set, round 2 set
No contested lines
Flags read, 0
Contested, 0

Set, pilot set
No contested lines
Flags read, 12
Contested, 0

Set, constructed round 4 example
ledger.py, line 6, near placeholder name
cache.py, line 15, near placeholder name
Flags read, 6
Contested, 2

Sets read, 3
Flags read, 18
Contested flags, 2
Sets with contested flags, 1
```

The pattern command reads a collection in the census form and prints the pattern reading. The reading states one section per borderline pattern, naming each flag of the collection standing under that pattern with its file, line, and the set it stands in, and it closes with counts over the whole collection.

```sh
stickler pattern examples/pattern.json
```

```text
Pattern

Pattern, near placeholder name
harvest.py, line 6, set pilot set
harvest.py, line 7, set pilot set
sorting.py, line 6, set pilot set
sorting.py, line 11, set pilot set
sorting.py, line 12, set pilot set
ledger.py, line 6, set constructed round 4 example
ledger.py, line 7, set constructed round 4 example
cache.py, line 15, set constructed round 4 example
cache.py, line 19, set constructed round 4 example
Flags read, 9
Contested, 2

Pattern, broad handler
harvest.py, line 12, set pilot set
harvest.py, line 18, set pilot set
sorting.py, line 15, set pilot set
cache.py, line 10, set constructed round 4 example
Flags read, 4
Contested, 0

Pattern, shared member default
windows.py, line 6, set pilot set
windows.py, line 10, set pilot set
windows.py, line 14, set pilot set
windows.py, line 18, set pilot set
cache.py, line 7, set constructed round 4 example
Flags read, 5
Contested, 0

Sets read, 3
Flags read, 18
Contested flags, 2
```

The files command reads a collection in the census form and prints the file reading. The reading states one section per file standing in the collection, naming each flag standing on that file with its line, its pattern, and the set it stands in, and it closes with counts over the whole collection.

```sh
stickler files examples/files.json
```

```text
Files

File, ledger.py
line 6, near placeholder name, set constructed round 4 example
line 7, near placeholder name, set constructed round 4 example
line 6, shared member default, set constructed round 10 example
line 9, broad handler, set constructed round 10 example
Flags read, 4
Contested, 2

File, cache.py
line 7, shared member default, set constructed round 4 example
line 10, broad handler, set constructed round 4 example
line 15, near placeholder name, set constructed round 4 example
line 19, near placeholder name, set constructed round 4 example
Flags read, 4
Contested, 1

File, tidy.py
line 6, near placeholder name, set constructed round 10 example
Flags read, 1
Contested, 0

Sets read, 2
Files read, 3
Flags read, 9
Contested flags, 3
```

The borderline command reads a collection in the census form and prints the borderline reading. The reading states one section per set holding at least one borderline flag, naming each borderline flag of the set with its file, line, pattern, the flagged line as written, and the shape reading the checker computed for it, and it closes with counts over the whole collection.

```sh
stickler borderline examples/borderline.json
```

```text
Borderline

Set, pilot set
Borderline, 12
harvest.py, line 6, near placeholder name, data = load_entries(), drawn name entries
harvest.py, line 7, near placeholder name, temp = [], drawn name items
harvest.py, line 12, broad handler,     except BaseException:, handler names BaseException
harvest.py, line 18, broad handler,     except BaseException:, handler names BaseException
windows.py, line 6, shared member default, def plan(start, cache=([],)):, default holds a list literal
windows.py, line 10, shared member default, def grid(rows, defaults=({"unit": "m"}, 2)):, default holds a dict literal
windows.py, line 14, shared member default, def pack(items, pair=([], {})):, default holds a list literal
windows.py, line 18, shared member default, def pick(mode, seen=({1, 2},)):, default holds a set literal
sorting.py, line 6, near placeholder name, def temp(rows):, no drawn name
sorting.py, line 11, near placeholder name,     data, extra = left, right, no drawn name
sorting.py, line 12, near placeholder name,     tmp: list = [], drawn name items
sorting.py, line 15, broad handler,     except BaseException as err:, handler names BaseException

Set, constructed round 4 example
Borderline, 6
ledger.py, line 6, near placeholder name, data = load_entries(), drawn name entries
ledger.py, line 7, near placeholder name, temp = {}, drawn name mapping
cache.py, line 7, shared member default, def fetch(key, memo=([],)):, default holds a list literal
cache.py, line 10, broad handler,     except BaseException:, handler names BaseException
cache.py, line 15, near placeholder name,     tmp = [], drawn name items
cache.py, line 19, near placeholder name, def data(rows):, no drawn name

Sets standing, 2
Borderline flags, 18
Sets read, 3
```

The outcomes command reads a collection in the census form and prints the outcomes reading. The reading states one section per judgement, in the order the contract states them, naming each flag of the collection standing under that judgement with its file, line, pattern, and the set it stands in, and it closes with counts over the whole collection.

```sh
stickler outcomes examples/outcomes.json
```

```text
Outcomes

Judged stickler
harvest.py, line 7, near placeholder name, set, pilot set
harvest.py, line 12, broad handler, set, pilot set
harvest.py, line 18, broad handler, set, pilot set
windows.py, line 6, shared member default, set, pilot set
windows.py, line 10, shared member default, set, pilot set
windows.py, line 14, shared member default, set, pilot set
windows.py, line 18, shared member default, set, pilot set
sorting.py, line 6, near placeholder name, set, pilot set
sorting.py, line 11, near placeholder name, set, pilot set
sorting.py, line 12, near placeholder name, set, pilot set
sorting.py, line 15, broad handler, set, pilot set
ledger.py, line 6, near placeholder name, set, constructed round 4 example
cache.py, line 7, shared member default, set, constructed round 4 example
cache.py, line 15, near placeholder name, set, constructed round 4 example
Flags read, 14

Judged clean
ledger.py, line 7, near placeholder name, set, constructed round 4 example
cache.py, line 10, broad handler, set, constructed round 4 example
Flags read, 2

Unjudged
harvest.py, line 6, near placeholder name, set, pilot set
cache.py, line 19, near placeholder name, set, constructed round 4 example
Flags read, 2

Sets read, 3
Flags read, 18
Judged stickler, 14
Judged clean, 2
Unjudged, 2
```

The verdicts command reads a collection in the census form and prints the verdicts reading. The reading states one section per set, in collection order, naming each flag of the set carrying at least one recorded verdict with its file, line, pattern, and the set it stands in, the verdict pairs standing on the flag in the set review form under it, or the none recorded line where no flag of the set carries one, and a closing count of the pairs stated per section, and it closes with counts over the whole collection.

```sh
stickler verdicts examples/verdicts.json
```

```text
Verdicts

Set, round 2 set
Verdicts, none recorded
Verdicts read, 0

Set, pilot set
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 22

Set, constructed round 4 example
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 10

Sets read, 3
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

The shapes command reads a collection in the census form and prints the shapes reading. The reading states one section per shape, in the order the contract states them, naming each judgement of the collection standing under that shape with its file, its standing number among the judgements of its file, its category, and the set it stands in, or the no judgements line where no judgement of the collection stands under a shape, and a closing count of the judgements stated per section, and it closes with counts over the whole collection.

```sh
stickler shapes examples/shapes.json
```

```text
Shapes

Shape, bare except
report.py, judgement 2, smell, set, round 2 set
Judgements read, 1

Shape, print call
report.py, judgement 3, smell, set, round 2 set
Judgements read, 1

Shape, mutable default
report.py, judgement 1, structure, set, round 2 set
Judgements read, 1

Shape, placeholder name
No judgements
Judgements read, 0

Shape, call default
basket.py, judgement 2, structure, set, round 2 set
Judgements read, 1

Shape, class attribute
basket.py, judgement 1, structure, set, round 2 set
Judgements read, 1

Sets read, 3
Judgements read, 5
```

The choices command reads a collection in the census form and prints the choices reading. The reading states one section per outcome, in the order the contract states them, naming each flag of the collection carrying at least one verdict standing under that outcome with its file, its line, its pattern, and the set it stands in, the verdict pairs of the flag standing under the outcome in the standing set review form, or the no verdicts line where no verdict of the collection stands under an outcome, and a closing count of the pairs stated per section, and it closes with counts over the whole collection.

```sh
stickler choices examples/choices.json
```

```text
Choices

Chose stickler
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 24

Chose clean
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdicts read, 5

Abstained
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
Verdicts read, 3

Sets read, 3
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

The categories command reads a collection in the census form and prints the categories reading. The reading states one section per category, in the order the contract states them, naming each judgement of the collection standing under that category with its file, its standing number among the judgements of its file, its shape, and the set it stands in, or the no judgements line where no judgement of the collection stands under a category, and a closing count of the judgements stated per section, and it closes with counts over the whole collection.

```sh
stickler categories examples/categories.json
```

```text
Categories

Category, naming
No judgements
Judgements read, 0

Category, structure
basket.py, judgement 1, class attribute, set, round 2 set
basket.py, judgement 2, call default, set, round 2 set
report.py, judgement 1, mutable default, set, round 2 set
Judgements read, 3

Category, smell
report.py, judgement 2, bare except, set, round 2 set
report.py, judgement 3, print call, set, round 2 set
Judgements read, 2

Sets read, 3
Judgements read, 5
```

The grounds command reads a collection in the census form and prints the grounds reading. The reading states one section per pattern, in the order the contract states them, naming each flag of the collection standing under that pattern and carrying at least one recorded verdict with its file, its line, its pattern, and the set it stands in, the verdict pairs standing on the flag in the standing set review form, or the no verdicts line where no verdict of the collection stands under a pattern, and a closing count of the pairs stated per section, and it closes with counts over the whole collection.

```sh
stickler grounds examples/grounds.json
```

```text
Grounds

Pattern, near placeholder name
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 14

Pattern, broad handler
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
Verdicts read, 8

Pattern, shared member default
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
Verdicts read, 10

Sets read, 3
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

The judgements command reads a collection in the census form and prints the judgements reading. The reading states one section per judgement, in the order the contract states them, naming each flag of the collection standing under that judgement and carrying at least one recorded verdict with its file, its line, its pattern, and the set it stands in, the verdict pairs standing on the flag in the standing set review form, or the no verdicts line where no verdict of the collection stands under a judgement, and a closing count of the pairs stated per section, and it closes with counts over the whole collection.

```sh
stickler judgements examples/judgements.json
```

```text
Judgements

Judged stickler
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 27

Judged clean
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
Verdicts read, 4

Unjudged
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
Verdicts read, 1

Sets read, 3
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

The standings command reads one standings file and prints the standings reading, one section per file carrying recorded verdicts, in first appearance order, each section holding the file's flag groups whole with all their pairs standing in the set review form, and the reading closing with the counts it shares with the verdicts reading and the files reading.

```sh
stickler standings examples/standings.json
```

```text
Standings

File, harvest.py
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 6

File, windows.py
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 8

File, sorting.py
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 8

File, ledger.py
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
Verdicts read, 4

File, cache.py
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 6

Sets read, 3
Files read, 5
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

The states command reads one states file and prints the states reading, one section per file verdict state in the order CK-8 states the judgements, each section holding the files standing in that state with their flag groups whole and all their pairs standing in the set review form, and the reading closing with the counts it shares with the verdicts reading and the standings reading.

```sh
stickler states examples/states.json
```

```text
States

Judged stickler
File, harvest.py
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 6
File, windows.py
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 8
File, sorting.py
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
Verdicts read, 8
File, ledger.py
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
Verdicts read, 4
File, cache.py
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 6
Verdicts read, 32

Judged clean
No files
Verdicts read, 0

Unjudged
No files
Verdicts read, 0

Sets read, 3
Files read, 5
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

The flags command reads one flags file and prints the flags reading, one section per file verdict state in the order CK-8 states the judgements, each section holding the flag lines of the files standing in that state in collection order with the contested count of its lines, and the reading closing with the counts it shares with the files reading and the contested reading.

```sh
stickler flags examples/flags.json
```

```text
Flags

Judged stickler
harvest.py, line 6, near placeholder name, set, pilot set
harvest.py, line 7, near placeholder name, set, pilot set
harvest.py, line 12, broad handler, set, pilot set
harvest.py, line 18, broad handler, set, pilot set
windows.py, line 6, shared member default, set, pilot set
windows.py, line 10, shared member default, set, pilot set
windows.py, line 14, shared member default, set, pilot set
windows.py, line 18, shared member default, set, pilot set
sorting.py, line 6, near placeholder name, set, pilot set
sorting.py, line 11, near placeholder name, set, pilot set
sorting.py, line 12, near placeholder name, set, pilot set
sorting.py, line 15, broad handler, set, pilot set
ledger.py, line 6, near placeholder name, set, constructed round 4 example
ledger.py, line 7, near placeholder name, set, constructed round 4 example
cache.py, line 7, shared member default, set, constructed round 4 example
cache.py, line 10, broad handler, set, constructed round 4 example
cache.py, line 15, near placeholder name, set, constructed round 4 example
cache.py, line 19, near placeholder name, set, constructed round 4 example
Flags read, 18
Contested, 2

Judged clean
No flagged lines
Flags read, 0
Contested, 0

Unjudged
No flagged lines
Flags read, 0
Contested, 0

Sets read, 3
Files read, 5
Flags read, 18
Contested flags, 2
```

The models command reads one models file and prints the models reading, one section per model standing in the collection in first appearance order, each section holding the flag groups of that model in collection order with the one pair each flag carries of that model, and the reading closing with the counts it shares with the verdicts reading and the census.

```sh
stickler models examples/models.json
```

```text
Models

Model, jev-1.13.0
harvest.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.32 and abstained
harvest.py, line 7, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.68
harvest.py, line 12, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
harvest.py, line 18, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 1.0
windows.py, line 6, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.96
windows.py, line 10, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.76
windows.py, line 14, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
windows.py, line 18, shared member default, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.74
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.2 and abstained
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.78
sorting.py, line 15, broad handler, set, pilot set
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.98
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.81
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.9
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, jev-1.13.0, stickler
Reason, jev-1.13.0 chose stickler with confidence 0.92
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, jev-1.13.0, abstained
Reason, jev-1.13.0 chose stickler with confidence 0.35 and abstained
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, jev-1.13.0, clean
Reason, jev-1.13.0 chose clean with confidence 0.74
Verdicts read, 17

Model, openai/gpt-6-luna
harvest.py, line 12, broad handler, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
harvest.py, line 18, broad handler, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 6, shared member default, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 10, shared member default, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 14, shared member default, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
windows.py, line 18, shared member default, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
sorting.py, line 6, near placeholder name, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.97
sorting.py, line 11, near placeholder name, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 12, near placeholder name, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
sorting.py, line 15, broad handler, set, pilot set
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.99
ledger.py, line 6, near placeholder name, set, constructed round 4 example
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.77
ledger.py, line 7, near placeholder name, set, constructed round 4 example
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.66
cache.py, line 7, shared member default, set, constructed round 4 example
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.88
cache.py, line 10, broad handler, set, constructed round 4 example
Verdict, openai/gpt-6-luna, clean
Reason, openai/gpt-6-luna chose clean with confidence 0.83
cache.py, line 15, near placeholder name, set, constructed round 4 example
Verdict, openai/gpt-6-luna, stickler
Reason, openai/gpt-6-luna chose stickler with confidence 0.69
Verdicts read, 15

Sets read, 3
Flags read, 18
Flags with verdicts, 17
Verdicts read, 32
Chose stickler, 24
Chose clean, 5
Abstained, 3
```

## Status

Seeded in October 2026. Setup has landed, and Rounds 1 to 30 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 30 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and CK-1 to CK-6 met. CK-1 to CK-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Contract, CK-5 and CK-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, a repair pass, and an approving re-review.
- Slice 2 has landed. It carries the review core, the built in checker with its four named shapes, the shared rendering, and the check and render commands, all running offline on recorded or local input. The first slice is a strict review of one diff against named standards, objections named with the rewrite each one would accept.
- Slice 3 has landed. It carries the two remaining definition time shapes, the call default and the class attribute with its class context read from the diff itself, and the set level review with the review command, over one synthetic change set under examples.
- Contract, CK-7 and CK-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Slice 4 has landed. It carries the borderline flagging with its three patterns, the verdict seat reading both decision model routes, the recorded verdict path through the set review, and the recorded borderline pilot under examples, with the suite running offline on the recorded envelopes.
- Contract, CK-9 and CK-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendments, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.
- Slice 5 has landed. It carries the contested line reading of the set review, with the contested section before the summary and the contested count in the summary, and the constructed contested example under examples, with the suite running offline on the recorded pilot and the constructed example.
- Contract, CK-11 and CK-12 met. They stand in full form in CONTRACT.md, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving owner gate.
- Slice 6 has landed. It carries the judging census over a stated collection of recorded review inputs, with the census command beside check, render, and review, and the census example under examples standing over the Round 2 set, the pilot set, and the constructed Round 4 example named as constructed, with the suite running offline on the recorded and constructed sets.
- Contract, CK-13 and CK-14 met. They stand in full form in CONTRACT.md with the Round 6 repair amendment, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.
- Slice 7 has landed. It carries the rejudging run over the pilot flags through the verdict seat's Jev route, recorded as the rejudged set under examples, and the rejudged ledger reading over a stated pair of recorded sets, with the rejudged command beside check, render, review, and census, with the suite running offline on the recorded envelopes and sets.
- Contract, CK-15 and CK-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews, a repair pass, and an approving re-review.
- Slice 8 has landed. It carries the split reading over one recorded set, stating the judgement of each of its flags under the first named route alone, under the second named route alone, and under both routes jointly, with the split example over the pilot set under examples, and the split command beside check, render, review, census, and rejudged, with the suite running offline on the recorded verdicts.
- Contract, CK-17 and CK-18 met. They stand in full form in CONTRACT.md with the Round 8 repair amendments, and Round 8 closed on the merged slice 9 after three reviews, a repair pass, and an approving re-review.
- Slice 9 has landed. It carries the contested reading over a recorded collection of change sets, one section per set in collection order, naming each set's contested flags with file, line, and pattern, or the no contest line where a set holds none, with per set flags read and contested counts and closing counts over the whole collection, with the contested example under examples standing over the same collection as the census example, and the contested command beside check, render, review, census, rejudged, and split, with the suite running offline on the recorded verdicts.
- Contract, CK-19 and CK-20 met. They stand in full form in CONTRACT.md with the FIX-CK-21 fencing closure, and Round 9 closed on the merged slice 10 after three reviews approving with no findings, with the red team's one observation on the single review rendering carried to Round 10 as a fencing component.
- Slice 10 has landed. It carries the pattern reading over a recorded collection of change sets, one section per borderline pattern in the order the contract states them, naming each flag of the collection standing under that pattern with file, line, and the set it stands in, with per section flags read and contested counts and closing counts over the whole collection, with the pattern example under examples standing over the same collection as the census example, and the pattern command beside check, render, review, census, rejudged, split, and contested, and the library fencing closure over the census, rejudged, split, and set review renderings, with the suite running offline on the recorded verdicts.
- Contract, CK-21 and CK-22 met. They stand in full form in CONTRACT.md with the FIX-CK-22 fencing closure, and Round 10 closed on the merged slice 11 after three reviews, with the red team's one observation on the set review rendering carried to a later round as a fencing candidate.
- Slice 11 has landed. It carries the file reading over a recorded collection of change sets, one section per file in first appearance order across the collection, naming each flag standing on that file with line, pattern, and the set it stands in, with per file flags read and contested counts and closing counts over the whole collection, with the constructed files example under examples standing over two constructed sets sharing one file, and the files command beside check, render, review, census, rejudged, split, contested, and pattern, and the review rendering fencing closure over the single review rendering, and the README brought to the fleet README standard, with the suite running offline on the recorded verdicts.
- Contract, CK-23 and CK-24 met. They stand in full form in CONTRACT.md with the FIX-CK-23 fencing closure, and Round 11 closed on the merged slice 12 after three reviews approving, with no repair pass.
- Slice 12 has landed. It carries the borderline reading over a recorded collection of change sets, one section per set holding at least one borderline flag in collection order, naming each borderline flag of the set with file, line, pattern, the flagged line as written, and the shape reading the checker computed for it, with per section borderline counts and closing counts over the whole collection, with the borderline example under examples standing over the same collection as the census example, and the borderline command beside check, render, review, census, rejudged, split, contested, pattern, and files, and the set review rendering fencing closure over the judgement, flag, and verdict fields of the set review rendering, with the suite running offline on the recorded verdicts.
- Contract, FIX-CK-24, FIX-CK-25, CK-25, and CK-26 met. They stand in full form in CONTRACT.md with the FIX-CK-24, FIX-CK-25, and FIX-CK-26 repair amendments, and Round 12 closed on the merged slice 13 after three reviews, one standards Minor and one red team Minor repaired.
- Slice 13 has landed. It carries the formats of the repo stated at version 1 in the contract with the consolidated formats section, the static outcome word fencing in the split and rejudged renderings, the outcomes reading over a recorded collection of change sets, one section per judgement in the order the contract states them, naming each flag of the collection standing under that judgement with file, line, pattern, and the set it stands in, with per section flags read counts and closing counts over the whole collection, with the outcomes example under examples standing over the same collection as the census example, and the outcomes command beside check, render, review, census, rejudged, split, contested, pattern, files, and borderline, and the constructed large change set and large diff under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-27 and CK-28 met, with FIX-CK-28. They stand in full form in CONTRACT.md with the FIX-CK-28 repair amendment, and Round 13 closed on the merged slice 14 after three reviews and three delta reviews on the repair, one red team Minor repaired and two record figures corrected.
- Slice 14 has landed. It carries the verdicts reading over a recorded collection of change sets, one section per set in collection order, naming each flag of the set carrying at least one recorded verdict with file, line, pattern, and the set it stands in, the verdict pairs of the flag in the standing set review form, and a closing count of the pairs stated per section, with closing counts over the whole collection, with the verdicts example under examples standing over the same collection as the census example, and the verdicts command beside check, render, review, census, rejudged, split, contested, pattern, files, borderline, and outcomes, and the constructed large verdicts collection under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-29 and CK-30 met. They stand in full form in CONTRACT.md, and Round 14 closed on the merged slice 15 after three reviews, no Major on any axis and no repair owed.
- Slice 15 has landed. It carries the shapes reading over a recorded collection of change sets, one section per shape in the order the contract states them, naming each judgement of the collection standing under that shape with file, standing number, category, and the set it stands in, with per section judgements read counts and closing counts over the whole collection, with the shapes example under examples standing over the same collection as the census example, and the shapes command beside check, render, review, census, rejudged, split, contested, pattern, files, borderline, outcomes, and verdicts, and the loader doc list restatement in the one drift free form across the nine loader docs, and the constructed shapes collection under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-31 and CK-32 met. They stand in full form in CONTRACT.md, and Round 15 closed on the merged slice 16 after three reviews, no Major on any axis and no repair owed.
- Slice 16 has landed. It carries the choices reading over a recorded collection of change sets, one section per outcome in the order the contract states them, naming each flag of the collection carrying at least one verdict standing under that outcome with file, line, pattern, and the set it stands in, the verdict pairs of the flag standing under the section's outcome in the standing set review form, and a closing count of the pairs stated per section, with closing counts over the whole collection, with the choices example under examples standing over the same collection as the census example, and the choices command beside check, render, review, census, rejudged, split, contested, pattern, files, borderline, outcomes, verdicts, and shapes, and the birth count restatement in the one drift free form across the six reading heads, and the constructed choices collection under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-33 and CK-34 met. They stand in full form in CONTRACT.md, and Round 16 closed on the merged slice 17 after three reviews, no Major on any axis and no repair owed.
- Slice 17 has landed. It carries the categories reading over a recorded collection of change sets, one section per category in the order the contract states them, naming each judgement of the collection standing under that category with file, standing number, shape, and the set it stands in, with per section judgements read counts and closing counts over the whole collection, with the categories example under examples standing over the same collection as the census example, and the categories command beside check, render, review, census, rejudged, split, contested, pattern, files, borderline, outcomes, verdicts, shapes, and choices, and the constructed categories collection under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-35 and CK-36 met, with FIX-CK-29. They stand in full form in CONTRACT.md, and Round 17 closed on the merged slice 18 after three reviews, no Major on any axis, with the red team's one Minor repaired in the slice.
- Slice 18 has landed. It carries the grounds reading over a recorded collection of change sets, one section per borderline pattern in the order the contract states them, naming each flag of the collection standing under that pattern and carrying at least one recorded verdict with file, line, pattern, and the set it stands in, the verdict pairs of the flag in the standing set review form, and a closing count of the pairs stated per section, with closing counts over the whole collection, with the grounds example under examples standing over the same collection as the census example, and the grounds command beside check, render, review, census, rejudged, split, contested, pattern, files, borderline, outcomes, verdicts, shapes, choices, and categories, and the constructed grounds collection under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-37 and CK-38 met. They stand in full form in CONTRACT.md, and Round 18 closed on the merged slice 19 after three reviews, no Major on any axis, with the spec lane's one Minor standing against the design record and the red team's two Observations standing as named voids carried to the Round 19 warm-up.
- Slice 19 has landed. It carries the judgements reading over a recorded collection of change sets, one section per judgement in the order CK-8 states them, naming each flag of the collection standing under that judgement and carrying at least one recorded verdict with file, line, pattern, and the set it stands in, the verdict pairs of the flag in the standing set review form, and a closing count of the pairs stated per section, with closing counts over the whole collection, with the judgements example under examples standing over the same collection as the census example, and the judgements command beside check, render, review, census, rejudged, split, contested, pattern, files, borderline, outcomes, verdicts, shapes, choices, categories, and grounds, and the constructed judgements collection under examples, with the suite running offline on the recorded verdicts.
- Contract, CK-39 and CK-40 met. They stand in full form in CONTRACT.md, and Round 19 closed on the merged slice 20 after three reviews, no Major on any axis, with the design record's catcher enumerations corrected in the programme records and the standings header restated to its executed postures in the slice.
- Slice 20 has landed. It carries the standings reading, the eighteenth command, its sections standing per file in first appearance order across the collection, each file's recorded pairs standing whole on their flags, over the collection form of CK-12.
- Contract, CK-41 and CK-42 met. They stand in full form in CONTRACT.md, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving clean on recomputation of the states reading, and the constructed collection's handed word requirement restated in its loadable form under FIX-CK-30.
- Slice 21 has landed. It carries the states reading, the nineteenth command, its sections standing per file verdict state in the order CK-8 states the judgements, each file's recorded pairs standing whole on their flags, over the collection form of CK-12.
- Contract, CK-43 and CK-44 met. They stand in full form in CONTRACT.md, and Round 21 closed on the merged slice 22 after three reviews, with the red team approving on recomputation of the flags golden and the collection figures, the design's MD posture stated in its corrected form from birth, and its one Minor, a header's base posture clause, restated to its true split.
- Slice 22 has landed. It carries the flags reading, the twentieth command, its sections standing per file verdict state in the order CK-8 states the judgements, each flag of a file standing in that state standing as one flag line in the form of CK-20, over the collection form of CK-12.
- Contract, FIX-CK-31 met. It stands in full form in CONTRACT.md, and Round 22 closed on the merged slice 23 after three reviews, with the standards review approving, the spec review approving with findings at one Observation standing against the design record's wording, the red team approving clean on executed attack, and CodeRabbit returning no comments.
- Slice 23 has landed. It carries the depth message repair under FIX-CK-31, the seventeen covered commands naming the depth refusal in their defect, with the render command's nesting fall and the split command's generic defect standing as they stand, and no reading of a file that parses rendering one byte differently.
- Contract, CK-45 and CK-46 met. They stand in full form in CONTRACT.md, and Round 23 closed on the merged slice 24 after three reviews, with the standards review approving with findings at four Observations, the spec review approving clean, the red team approving with findings at three Minors standing against the design record, and CodeRabbit returning one Minor.
- Slice 24 has landed. It carries the models reading, the twenty-first command, its sections standing per model in first appearance order, each section holding one flag group per flag carrying a verdict of that model with the one pair of that model the flag carries, over the collection form of CK-12.
- Contract, CK-47 met, with FIX-CK-32 spent. They stand in full form in CONTRACT.md, and Round 24 closed on the merged slice 25 after three reviews, with the standards review approving with findings at two Observations, the spec review approving with findings at two Observations, the red team approving with findings at two Minors and one Observation, both Minors standing as CodeRabbit's two inline Minors on the slice, adjudicated valid on the code and cured in the slice by the repair at e3ffc300, and CodeRabbit returning two Minors. The wider executed mutation sets stand against the design record's split statements as the spec review adjudicated them, PA at the verdicts seat felling the fix forward pin beside the family pin, through the binding of its models figure to the verdicts reading over the same fixture, PC felling the six standings pins over the standings collection beside the family pin, the once rule's witnesses at the distinct entries grain, and PE in both its forms felling the fix forward pin, whose collection stands as one entry standing twice under a collection loader, and the three duplicate labels and unread fields pins of CK-12, which a loader refusing or skipping repeated labels fells by definition, each set the charged defect acting on a standing witness of the same rule, with the family pin's own discrimination executed exact at every seat. The contested count of the PD leg stands computed at two homes, the builder's field and the closing recompute in render_contested, and the executed leg stands seated at both, where the design record's PD wording names the builder alone. The family pin's outcomes seat stood asserted by its first two lines alone at the slice head, and the repair cured that grain by asserting the closing whole. The build record's PE passed total stands at 1031 against the red team's executed 1033 passed on the final head, the build figure summing to the tests commit census of 1036, with the failing set identical under both counts.
- Slice 25 has landed. It carries the repeated entry standing of CK-47, pinned at the command surface over the sixteen collection commands and the rejudged pair of one entry standing twice, and the fencing enumeration of CK-46 completed under FIX-CK-32, with no rendering over any collection moving.
- Contract, CK-48 met. It stands in full form in CONTRACT.md, and Round 25 closed on the merged slice 26 after three reviews, with the standards review approving with findings at three Observations, the spec review approving with findings at two Observations, the red team approving with findings at one Minor, the Minor standing as CodeRabbit's one inline Minor on the slice, adjudicated valid on the code and cured in the slice by the repair at d2a5355d, and CodeRabbit returning one Minor. The design record's section 4 phrasing names sixteen collection goldens over census.json, where fifteen stand counted over that input and the files golden stands over files.json, as the spec review's Observation records it. The census total stands computed at two homes, the renderer's summed tally standing as its printed home and the builder's Census.total field standing read by the suite as an independent oracle and never by the renderer, and the QC leg stands seated at both homes in the posture of the Round 24 PD leg, the design record naming the tally's behaviour and no seat. The standards review's anchoring Observation stands against the obligations pin, whose two phrases stand asserted by whole file contains where the Round 24 pin anchors its guarded phrase within its bullet, equivalent in effect today, the cure priced at the next contract opening and not charged to this slice. The design record's sweep ran on the Round 24 red team binary, its provenance proven with every rust/src blob standing identical between that tree and the base, and the slice's warm-up on the slice tree stands as the execution of record. The section exchange mutant passed the whole suite against the pin as first built, and stands executed again against the repaired tree, failing the repaired pin alone after the section scoped restatement.
- Slice 26 has landed. It carries the checker judgement standing of a repeated entry under CK-48, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- Contract, CK-49 met. It stands in full form in CONTRACT.md, and Round 26 closed on the merged slice 27 after three reviews, with the spec review approving with findings at three Observations, the standards review approving with findings at one Minor and three Observations, the red team approving with findings at one Minor and one Observation, and CodeRabbit returning one inline Minor on the slice, the Minor standing as one defect across the three reads, the categories builder's lines grouped by set label passing the whole suite against the pin as first built, its counts whole and its line order unwitnessed at the categories seat. The defect stands cured in the slice by the repair at abac376d, one order assertion over the smell section's eight lines inside the existing pin, the categories grouping mutant failing the repaired pin alone at 1043 passed and 1 failed and passing whole on the unrepaired head at 1044 passed and 0 failed, both directions executed by the repair lane. The design record's RB and RC computed splits stand corrected by execution under the owner's two rulings, the CK-48 pin falling beside the new pin at its categories and census closings in the same witness form the design record charged at the shapes seat, both executed sets adopted whole as the corrected expectation, with RC at the builder seat executed at the null set as computed, and RF's construction reseated in the QE genus form after the loop header form failed to compile, its executed set standing exact at the seven computed pins. The anchoring cure stands proven in both directions by the red team, the cured pin failing on a guarded phrase moved out of its bullet's line while the phrase stands elsewhere in the contract, where the uncured whole file pin passes over the equivalently moved contract. The red team's files read Observation stands as its own model's correction and no charge against the slice, the per file readings counting distinct files, the posture the CK-47 family pin itself asserts. The new pin's section totals stand as whole output multiplicity counts while its lines stand section scoped, identically the CK-48 pin's repaired Round 25 form, and the tests commit's entry builder reads its separator from the second span with no length guard, standing called at one seat over a three file entry with its removal count asserted first, a panic failing the pin so no silent pass stands. No amendment stands spent by the round, FIX-CK-33 standing free and FIX-CK-27 unconsumed.
- Slice 27 has landed. It carries the checker judgement standing of two judgement bearing entries each standing twice under CK-49, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- Contract, CK-50 met. It stands in full form in CONTRACT.md, and Round 27 closed on the merged slice 28 after three reviews, with the spec review approving with findings at one Observation, the standards review approving with findings at one Minor and three Observations, the red team approving with no findings, and CodeRabbit returning one substantive comment on the slice in its review body and no inline comments, the comment standing as the standards review's Minor across the two reads, the census block order CK-11 states standing unasserted by the round's pin as first built, the pin asserting the two per set blocks whole at exactly two occurrences each and the total whole and no sequence over the blocks, so a census rendering walking the standings grouped or permuted printed each block whole in a wrong order under every assertion the pin carried. The defect stands cured in the slice by the repair at d79c28fe, one ordered sequence assertion over the four census blocks inside the existing pin, the grouped census walk passing the unrepaired head whole at 1047 passed and 0 failed and failing the repaired pin alone at 1046 passed and 1 failed, both directions executed by the repair lane, and the design family's SA and SF standing re-executed on the repaired tree name for name. The design executed its whole mutation family at design time and the build re-executed it name for name, SA, SB, and SC falling at the round's pin alone, SD falling at the round's pin with the pins of CK-49 and CK-48 at the renderer's seat and at the null set at the builder's seat, SE at the same three pins, and SF at eight. The red team's own witness mutation SG, the census renderer keeping the first tally it prints under each label and printing it at every later standing of that label, falls at the round's pin alone at a seat the design's family does not name, and its fifteen edge drives over collections of its own devising stand in byte agreement between the slice and base binaries. The one Observation the spec review and the standards review share, the design note's section 7 naming the manifest as rust/Cargo.toml where the repo carries it at its root as Cargo.toml, stands for the errata queue, no slice text resting on it, and the standards review dispositions CodeRabbit's summary remark on the categories ordering check as not valid, the structure section's lines standing distinguishable by their files and asserted as an exact standing order sequence. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1047. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.
- Slice 28 has landed. It carries the checker judgement standing of two distinct entries under one label under CK-50, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- Contract, CK-51 met. It stands in full form in CONTRACT.md, and Round 28 closed on the merged slice 29 after three reviews, with the spec review approving with findings at two Observations, the standards review approving with findings at one Observation, and the red team approving with no findings against the slice and one Observation, every Observation standing against the design record and none charged to the slice, and CodeRabbit returning no actionable comments, no inline comments, and no submitted reviews on the slice. CK-51 stands met by one companion pin over the pin of CK-49's own fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry relabelled alpha set and the second the round 2 set entry itself, the pin driving the sixteen collection commands and asserting the census blocks in standing order, each block asserted whole at two occurrences and the four blocks ordered by the advancing cursor form the Round 27 repair set, each carrying shapes section's lines as an ordered whole at all five sections, and the categories structure and smell sections' lines as ordered wholes, with no standing pin restated, no file under rust/src moving, and the build step standing expressly empty. The slice stands as pull request 69 merged at bce4fd1c, with main's tree standing verified equal to the slice tree c777988c, and the census standing at 1050. The design executed its whole mutation family at design time and the build re-executed it name for name in the warm-up, the warm-up standing whole with no parting and the build's and the design's eighteen line leg ledgers standing byte identical, the four gap legs passing whole on the base at 1047 passed and 0 failed and every gap leg falling at the new pin on the form. The red team re-executed the four base gap legs whole at 1047 passed and 0 failed and TA at the renderer and the grouped tally walk TG on the head name for name, and its own disturbance, a stable sort of the categories structure section's lines by file name, a field no leg of the design's family names, falls at the pin of CK-50 and the new pin as it predicted before its run, the new pin falling at its structure section assertion, and its differential over 208 drives between the base and head binaries standing at no parting. The standards review's own label grouping disturbance at the census renderer fails the new pin alone at its census block order assertion, the pin of CK-49 passing under the same disturbance. The one parting the build found stands settled by execution and recorded in the open, the design note's section 4 print of the pin grouping the shapes call default and class attribute vecs by label, alpha, alpha, round 2, round 2, where the executed text alternates them by standing, alpha, round 2, alpha, round 2, the note's own section 5 prose standing with the executed text against the print, and the print standing named for the errata queue. The design report's section 5 claim, that the pin text printed in the note stands verified byte for byte against the executed scratch text, stands refuted at those two vecs by the spec review's three way comparison of the note's print, the design lane's form tree, and the committed blob, both items standing as records items with no cure charged. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1050. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.
- Slice 29 has landed. It carries the standing order of two judgement bearing entries each standing twice under CK-51, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- Contract, CK-52 met. It stands in full form in CONTRACT.md, and Round 29 closed on the merged slice 30 after three reviews, with the spec review approving with findings at two Observations, the standards review approving with findings at one Observation, and the red team approving with findings at three Observations, every Observation standing uncharged to the slice and the spec review's two standing as records items against the build report, and CodeRabbit returning no actionable comments, no inline comments, and no submitted reviews on the slice. CK-52 stands met by one companion pin in the form of the pin of CK-51 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry relabelled alpha set and the second the round 2 set entry itself, the pin's surgery restating the one added line of tidy.py in the round 2 entry as the placeholder assignment and so seating one placeholder name judgement in tidy.py where the standing collection carries none, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks at two occurrences each in standing order, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 71 merged at 2326d5b8, with main's tree standing verified equal to the slice tree b763be1e, and the census standing at 1053. The design executed its whole mutation family at the desk, the gap legs PH and NM passing whole on the base at 1050 passed and 0 failed and each falling at the new pin alone on the form at 1052 passed and 1 failed, the controls BE and SM falling at 1048 passed and 2 failed at the pins of CK-49 and CK-51, the candidate legs standing at 1051 passed and 0 failed for the pin alone on the base and at 1049 passed and 3 failed for the status texts on the base, and the form gate standing whole at 1053 passed and 0 failed with its format and clippy legs at exit 0. The build re-executed the base gate whole at 1050 passed and 0 failed and the sixteen command sweep over the surgical fixture matching the design's figures name for name, and its formatting proof stands executed over the whole form tree at exit 0 and corroborated on the head blobs by the standards review. The red team's independent oracle, built from the fixture and the shipped rules alone, reproduced the companion pin's asserted lines whole at every section of both readings, and its break attempts by construction found no construction that moves a line of either seat while the pin passes. Nine legs of the build's warm-up stand carried unexecuted, b1-ph-shapes, b2-nm-categories, b3-tc-bareexcept, b4-tb-smell, c1-pin-on-base, c2-status-on-base, f1-ph-shapes, f2-nm-categories, and the local form gate run, after twenty nine counted attempts refused by the box write fault in the build lane and a fault shut seating in the red team lane, the red team's differential over the standing command set standing carried with them, and the hosted gate on the slice head standing as the verdict of record in their place. The build report's two records items stand named in the open, the first construction attempt's two commits standing unreferenced with trees identical to the branch chain's stage trees, and the warm-up runner's defect, a text value passed where bytes were required on the first b1 attempt, standing cured in the runner before any leg read a result from it. A sort of either seat's lines by file name stands as the identity over this fixture and invisible to the pin, the design pricing the decline in the open and no asserted line moving under it, and the categories section head order stands unwitnessed by this pin's extraction form, standing under CK-34 and its pins, the shapes head order standing asserted by this pin. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1053. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.
- Slice 30 has landed. It carries the standing order of two judgement bearing entries each standing twice at the naming seats under CK-52, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- Contract, CK-53 met. It stands in full form in CONTRACT.md, and Round 30 closed on the merged slice 31 after three reviews, with the spec review approving with findings at two Observations, the standards review approving with no findings, and the red team approving with findings at two Observations, every Observation standing against the design record and none charged to the slice, and CodeRabbit returning no actionable comments, no inline comments, and no submitted reviews on the slice. CK-53 stands met by one companion pin in the form of the pin of CK-52 over the surgical fixture, one collection carrying two judgement bearing entries of the standing census collection at two standings each in interleaved order under their own labels, the first a surgical derivative of the round 2 set entry relabelled alpha set and the second the round 2 set entry itself, the pin's surgery restating the one added line of tidy.py in the round 2 entry as the placeholder assignment and the print call line of report.py in the same entry as the same assignment at its own indent, and so seating the placeholder name judgements of the collection in two files where the fixture of CK-52 carries them in one, the pin driving the sixteen collection commands and asserting the closings whole, the six shapes sections' lines as ordered wholes, the three categories sections' lines as ordered wholes, and the census blocks byte for byte from the derived counts, with no file under rust/src moving, the build step standing expressly empty, and the slice's one restatement of a standing pin standing as the round 4 standing pin's chain segment with its count moving from zero to one. The slice stands as pull request 73 merged at 3191cd75, with main's tree standing verified equal to the slice tree 723771f6, and the census standing at 1056. The design proved the gap by computation over the hash verified texts, the placeholder name section of the shapes reading and the naming section of the categories reading standing empty over the standing census collection and over the fixtures of CK-49, CK-50, and CK-51, and carrying four lines each over the fixture of CK-52 with every line standing in tidy.py, so a stable sort of either section's lines by file name stands as the identity there and passes the whole suite at 1053 passed and 0 failed as computed, and over the new fixture the sort parting from the standing order at the second line of each seat, so the new pin falls there and there alone as computed, the gap's execution standing charged to the build's warm-up as legs g1 and g2. The build verified the premise whole over all 171 files of the base tree, executed its formatting proof whole in memory over both touched Rust files with the base texts standing as controls, and cured in the slice the round 4 segment's indent splice the design note's printed form caused, no word of the comment and no substance of the assertion moving under the cure. The red team's independent oracle, built from the fixture and the shipped rules alone and validated against the pin of CK-52 whole before it stood believed anywhere, reproduced the gap computation whole, deriving twenty judgements over the new fixture with the eight lines of each seat alternating report.py and tidy.py at every line and the print call population at zero, and its fuller enumeration over every collection a standing pin drives found the computation's conclusion standing whole, the constructed collections carrying lines of both seats across files while no standing pin asserts a line of either seat over them and the large collection's eleven lines at each seat standing already in file order, and its break attempts by construction found no construction that moves a line of either seat while the pin passes, a stable sort by judgement number parting at the first line of each seat and a stable sort by set label at the third line, each falling at the new pin by the same computation that convicted the file name sort. The warm-up family stands in two legs beside them. The first leg, the nine legs and the differential the Round 29 closure carried, stands executed at b1-ph-shapes alone, the red team's resumed session running that leg whole in the window after the rate reset at 1050 passed and 0 failed, matching its computed expectation exactly, its executed verdict resting on the runner's printed line of the run itself with the run's log standing overwritten by a later refused attempt, and its remaining eight legs, b2-nm-categories, b3-tc-bareexcept, b4-tb-smell, c1-pin-on-base, c2-status-on-base, f1-ph-shapes, f2-nm-categories, and the local form gate run, stand carried unexecuted with the differential over the standing command set, after twenty nine counted attempts in the Round 29 build, forty five counted attempts and a persistent run of four hundred ninety nine at the Round 30 design, ten leg attempts at the Round 30 build, ten attempts in the red team's detached b2 retry loop, seven driver attempts at b1-ph-shapes and eighty refused window probes in the red team's completion lane, and thirty attempts at b2-nm-categories and three attempts at every other carried leg in the red team's resumed session, the one b2 attempt that reached compilation in that session dying on the fault at seven os errors with no assertion failure, and the hosted gate on the slice head standing as the verdict of record in their place. The second leg, this round's own ten legs and its differential, stands carried whole and unexecuted, its gap legs g1-fn-shapes and g2-fn-categories standing derived by the oracle and nowhere executed, its controls and candidate legs standing at their computed figures, its form gate's format leg over the two touched files standing discharged by the build's formatting proof, and the build's landings of its two source trees standing refused after eight counted write attempts at each tree's first file. The design record's items stand named in the open, the note's third section stating the word naming at one place in checker.rs where the substring stands at two places, the naming category site at line 769 and a doc comment at line 836, its fourth section gap computation standing incomplete as a proof against the fuller enumeration the red team executed, its conclusion standing whole, and its printed round 4 segment standing flush left where the segment's seat carries the pin body's indent. The categories section head order stands under CK-34 and its pins with no gap proven, a file name ordering at the sections beside the two seats stands already witnessed by the Round 28 red team's executed disturbance at the structure section, and a judgement number ordering at the two seats stands covered by the new pin without a second obligation. The round's obligations pin stands written in the anchored form from its first writing, and the suite stands at 1056. No amendment stands spent by the round, FIX-CK-33 standing free and unspent and FIX-CK-27 standing unconsumed.
- Slice 31 has landed. It carries the standing order of two judgement bearing entries each standing twice at the naming seats under CK-53, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- Contract, CK-54 now stands in full form, the standing order of the checker judgement standing of two judgement bearing set entries each standing twice at the print call, mutable default and call default sections and at the smell section, the print call judgements of the collection standing in two files and the mutable default and call default judgements of the collection each standing in two files, across the shapes and categories readings, Round 31 has not closed.
- Slice 32 is built and in review. It carries the standing order of two judgement bearing entries each standing twice at the print call, mutable default, call default and smell seats under CK-54, pinned at the command surface over the sixteen collection commands, with no rendering over any collection moving.
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-9.
- Local gate, cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings. CI runs the same commands.

## Licence

MIT. See LICENSE.
