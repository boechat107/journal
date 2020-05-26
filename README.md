# Journal

![Action Status](https://github.com/boechat107/journal/workflows/Rust/badge.svg)

A simplistic personal journal in the command-line.

## Installation

There are no released binaries for now. You need to have
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) in
order to compile the code.

## Usage

```bash
> journal
journal 0.1.0

USAGE:
    journal <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list
    new

```

Add a new journal page:

```bash
> journal new -t "This is my first page"
New page id: 0
1 Jornal pages:
Collection {
    pages: {
        0: Page {
            text: "This is my first page",
            date: 2020-05-09-03:00,
            created_at: 2020-05-09T18:45:50.963010304-03:00,
            updated_at: 2020-05-09T18:45:50.963010304-03:00,
            tags: [],
        },
    },
    id_cnt: 1,
```

## Roadmap

- [x] persist data
- [ ] command to update pages
- [ ] support filter options for the `list` sub-command
- [ ] support for tags
- [ ] improve the string format used to print data on STDOUT
