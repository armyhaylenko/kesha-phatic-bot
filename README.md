# Kesha phatic dialogue bot

### How to run

Install Rust & Cargo via [rustup](https://rustup.rs).
Then run like `cargo +nightly run` from the project root.

### How to extend

#### Position markers

`*` are word boundaries in the templates.

#### Position markers: example

`*hi` would match `very hi`,
just like `hi*` would match `hi girls`.
`*hi*` will match any string that contains `hi`.

#### Word markers

`<w>` is a *word* marker. If you want to get a reply that would
contain some word from user input, mark that word as `<w>`.

#### Word markers: example

`*hi <w>*` would match strings like `very hi natasha :D` and capture
`natasha` as `<w>`. We could reply to it like `<w>?! I'm not <w>, I'm Kesha!`.
In our case, the actual reply would be `natasha? I'm not natasha, I'm Kesha!`.

#### Template/Answer format

The templates and answers are located in the `responses.txt` file.
All templates/answers must obey the following format: `<template>-><answer>`,
with template/answer only containing ASCII characters and position/word markers.

Have fun!


