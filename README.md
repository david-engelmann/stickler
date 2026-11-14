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

Seeded in October 2026. Setup has landed, and Rounds 1 to 23 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 23 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
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
- Contract, CK-47 and the FIX-CK-32 repair amendment now stand in full form, the repeated entry standing across the collection readings and the fencing enumeration of CK-46 completed. Round 24 has not closed.
- Slice 25 is built and in review. It carries the repeated entry standing of CK-47, pinned at the command surface over the sixteen collection commands and the rejudged pair of one entry standing twice, and the fencing enumeration of CK-46 completed under FIX-CK-32, with no rendering over any collection moving.
- The Rust port has landed. It carries the behavior the Python package carried, checked against the same goldens, and the Python tree stands removed under PORT-9.
- Local gate, cargo test, cargo fmt --check, and cargo clippy --all-targets -- -D warnings. CI runs the same commands.

## Licence

MIT. See LICENSE.
