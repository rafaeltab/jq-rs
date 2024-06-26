# jq-rs

[![crates.io](https://img.shields.io/crates/v/jq-rs.svg)](https://crates.io/crates/jq-rs)
[![crates.io](https://img.shields.io/crates/d/jq-rs.svg)](https://crates.io/crates/jq-rs)
[![docs.rs](https://docs.rs/jq-rs/badge.svg)](https://docs.rs/jq-rs)
[![Build Status](https://github.com/onelson/jq-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/onelson/jq-rs/actions/workflows/test.yaml)

## Overview

> Prior to v0.4.0 this crate was named [json-query].

This rust crate provides access to [jq] 1.6 via the `libjq` C API (rather than
"shelling out").

By leveraging [jq] we can extract data from json strings using `jq`'s dsl.

This crate requires Rust **1.32** or above.

## Usage

The interface provided by this crate is very basic. You supply a jq program
string and a string to run the program over.

```rust
use jq_rs;
// ...

let res = jq_rs::run(".name", r#"{"name": "test"}"#);
assert_eq!(res.unwrap(), "\"test\"\n".to_string());
```

In addition to running one-off programs with `jq_rs::run()`, you can also
use `jq_rs::compile()` to compile a jq program and reuse it with
different inputs.

```rust
use jq_rs;

let tv_shows = r#"[
 {"title": "Twilight Zone"},
 {"title": "X-Files"},
 {"title": "The Outer Limits"}
]"#;

let movies = r#"[
 {"title": "The Omen"},
 {"title": "Amityville Horror"},
 {"title": "The Thing"}
]"#;

let mut program = jq_rs::compile("[.[].title] | sort").unwrap();

assert_eq!(
 &program.run(tv_shows).unwrap(),
 "[\"The Outer Limits\",\"Twilight Zone\",\"X-Files\"]\n"
);

assert_eq!(
 &program.run(movies).unwrap(),
 "[\"Amityville Horror\",\"The Omen\",\"The Thing\"]\n",
);
```

## A Note on Performance

While the benchmarks are far from exhaustive, they indicate that much of the
runtime of a simple jq program goes to the compilation. In fact, the compilation
is _quite expensive_.

```text
run one off             time:   [48.594 ms 48.689 ms 48.800 ms]
Found 6 outliers among 100 measurements (6.00%)
3 (3.00%) high mild
3 (3.00%) high severe

run pre-compiled        time:   [4.0351 us 4.0708 us 4.1223 us]
Found 15 outliers among 100 measurements (15.00%)
6 (6.00%) high mild
9 (9.00%) high severe
```

If you have a need to run the same jq program multiple times it is
_highly recommended_ to retain a pre-compiled `JqProgram` and reuse it.

## Handling Output

The return values from jq are _strings_ since there is no certainty that the
output will be valid json. As such the output will need to be parsed if you want
to work with the actual data types being represented.

In such cases you may want to pair this crate with [serde_json] or similar.

For example, here we want to extract the numbers from a set of objects:

```rust
use jq_rs;
use serde_json::{self, json};

// ...

let data = json!({
 "movies": [
     { "title": "Coraline", "year": 2009 },
     { "title": "ParaNorman", "year": 2012 },
     { "title": "Boxtrolls", "year": 2014 },
     { "title": "Kubo and the Two Strings", "year": 2016 },
     { "title": "Missing Link", "year": 2019 }
 ]
});

let query = "[.movies[].year]";
// program output as a json string...
let output = jq_rs::run(query, &data.to_string()).unwrap();
// ... parse via serde
let parsed: Vec<i64> = serde_json::from_str(&output).unwrap();

assert_eq!(vec![2009, 2012, 2014, 2016, 2019], parsed);
```
## Options

Jq has flags to alter the way in which data is input or output, some of these flags are supported.
The supported flags are available throught the _advanced varients of the run functions.

```rust
use jq_rs;
use serde_json::{self, json};

let data = json!({ "title": "Coraline", "year": 2009 });
let query = ".title";

// program output as a raw string, without quotes
let output = jq_rs::run_advanced(query, &data.to_string(), jq_rs::JqOptions::default().with_raw_output(true));

let output_raw = jq_rs::run_advanced(query, &data.to_string());

assert_eq!("\"Coraline\"", output);
```

### Raw input and raw output

jq-rs supports the `-R, --raw-input` and `-r, --raw-output` flags through the following options:

```rust
use jq_rs;
let options = jq_rs::JqOptions::default()
    .with_raw_output(true)
    .with_raw_input(true);
```

These are disabled by default.

### Compact output

jq-rs supports the `-c, --compact-output`, `--tabs` and `--indent n` flags through the following options:

```rust
use jq_rs;
let compact = jq_rs::JqOptions::default()
    .with_indentation(jq_rs::JqIndentation::Compact);

let tabs = jq_rs::JqOptions::default()
    .with_indentation(jq_rs::JqIndentation::Tabs);

let spaces_2 = jq_rs::JqOptions::default()
    .with_indentation(jq_rs::JqIndentation::Spaces(2));
```

Compact is the default for this option.

### Sorting

jq-rs supports the `-S, --sort-keys` flag using the following option:

```rust
use jq_rs;
let sorted = jq_rs::JqOptions::default()
    .with_sort_keys(true);
```

Sorting is disabled by default.

### Colorization

jq-rs supports the `-C, --color-output` and `-M, --monochrome-output` flags.
jq-rs also supports custom colors, which are normally available through the `JQ_COLORS` environment variable.

```rust
use jq_rs;

let monochrome = jq_rs::JqOptions::default()
    .with_colorization(jq_rs::JqColorization::Monochrome);

let colorize = jq_rs::JqOptions::default()
    .with_colorization(jq_rs::JqColorization::Colorize),

let all_blue = jq_rs::JqOptions::default()
    .with_colorization(jq_rs::JqColorization::Custom(
        "0;34:0;34:0;34:0;34:0;34:0;34:0;34:0;34",
    ));
```

The default option is monochrome, refer to the [jq documentation](https://jqlang.github.io/jq/manual/#colors) for using custom colors.

## Linking to libjq

This crate requires access to `libjq` at build and/or runtime depending on the
your choice.

When the `bundled` feature is enabled (**off by default**) `libjq` is provided
and linked statically to your crate by [jq-sys] and [jq-src]. Using this feature
requires having autotools and gcc in `PATH` in order for the to build to work.

Without the `bundled` feature, _you_ will need to ensure your crate
can link to `libjq` in order for the bindings to work.

You can choose to compile `libjq` yourself, or perhaps install it via your
system's package manager.
See the [jq-sys building docs][jq-sys-building] for details on how to share
hints with the [jq-sys] crate on how to link.


[jq]: https://github.com/stedolan/jq
[serde_json]: https://github.com/serde-rs/json
[json-query]: https://crates.io/crates/json-query
[jq-sys]: https://github.com/onelson/jq-sys
[jq-sys-building]: https://github.com/onelson/jq-sys#building
[jq-src]: https://github.com/onelson/jq-src
