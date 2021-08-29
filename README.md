# jljoin

Read two [NDJSON](http://ndjson.org/) files and combine lines with common member values.

**Important: FILE1 and FILE2 must be sorted by the value of the join member. (e.g. [jlsort](https://github.com/winebarrel/jlsort))**

[![Build Status](https://github.com/winebarrel/jljoin/workflows/CI/badge.svg)](https://github.com/winebarrel/jljoin/actions)

## Installation

```
brew install winebarrel/jl/jljoin
```

## Usage

```
Usage: jljoin [OPTIONS] FILE1 FILE2

Options:
    -k, --key KEY       JSON key to join
    -1, --key1 KEY1     JSON key to join of FILE1
    -2, --key2 KEY2     JSON key to join of FILE2
    -m, --merge PRIORITY_FILENUM (1 or 2)
                        Merge the paired JSON
        --allow-no-key  Allow no key
    -v, --version       Print version and exit
    -h, --help          Print usage and exit
```

```
% cat a.ndjson
{"id": 1, "sub_id": 11, "file":"ndjson1", "val":"both"}
{"id": 2, "sub_id": 21, "file":"ndjson1", "val":"both/multi 1"}
{"id": 2, "sub_id": 22, "file":"ndjson1", "val":"both/multi 1"}
{"id": 3, "sub_id": 31, "file":"ndjson1", "val":"both/multi 2"}
{"id": 4, "sub_id": 41, "file":"ndjson1", "val":"only 1"}
{"id": 6, "sub_id": 62, "file":"ndjson1", "val":"both/multi"}
{"id": 6, "sub_id": 61, "file":"ndjson1", "val":"both/multi"}

% cat b.ndjson
{"id": 1, "sub_id": 11, "file":"ndjson2", "val":"both"}
{"id": 2, "sub_id": 21, "file":"ndjson2", "val":"both/multi 1"}
{"id": 3, "sub_id": 31, "file":"ndjson2", "val":"both/multi 2"}
{"id": 3, "sub_id": 32, "file":"ndjson2", "val":"both/multi 2"}
{"id": 5, "sub_id": 51, "file":"ndjson2", "val":"only 2"}
{"id": 6, "sub_id": 62, "file":"ndjson2", "val":"both/multi"}
{"id": 6, "sub_id": 61, "file":"ndjson2", "val":"both/multi"}

% jljoin -k id a.ndjson b.ndjson
[{"file":"ndjson1","id":1,"sub_id":11,"val":"both"},{"file":"ndjson2","id":1,"sub_id":11,"val":"both"}]
[{"file":"ndjson1","id":2,"sub_id":21,"val":"both/multi 1"},{"file":"ndjson2","id":2,"sub_id":21,"val":"both/multi 1"}]
[{"file":"ndjson1","id":2,"sub_id":22,"val":"both/multi 1"},{"file":"ndjson2","id":2,"sub_id":21,"val":"both/multi 1"}]
[{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"},{"file":"ndjson2","id":3,"sub_id":31,"val":"both/multi 2"}]
[{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"},{"file":"ndjson2","id":3,"sub_id":32,"val":"both/multi 2"}]
[{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":62,"val":"both/multi"}]
[{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":61,"val":"both/multi"}]
[{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":62,"val":"both/multi"}]
[{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"},{"file":"ndjson2","id":6,"sub_id":61,"val":"both/multi"}]

% jljoin -k id -m 1 a.ndjson b.ndjson
{"file":"ndjson1","id":1,"sub_id":11,"val":"both"}
{"file":"ndjson1","id":2,"sub_id":21,"val":"both/multi 1"}
{"file":"ndjson1","id":2,"sub_id":22,"val":"both/multi 1"}
{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"}
{"file":"ndjson1","id":3,"sub_id":31,"val":"both/multi 2"}
{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"}
{"file":"ndjson1","id":6,"sub_id":62,"val":"both/multi"}
{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"}
{"file":"ndjson1","id":6,"sub_id":61,"val":"both/multi"}
```

## Related Links

* https://github.com/winebarrel/jlsort
* https://github.com/winebarrel/jluniq
