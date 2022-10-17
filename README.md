# Neo-RethinkDB Rust Driver

This project is actively maintained on [gitlab](https://gitlab.com/exytech/community/rethinkdb-for-rust)
## Overview

the genesis version of this project is a fork of the official [rethink-rs]("https://github.com/rethinkdb/rethinkdb-rs") driver. Initially, as we were starting with Rust, our goal was to document the official driver while trying to understand the techniques used in the genesis source code. Then we realised that some features were missing in this version, so we undertook to refactor the project by first adding the missing features and then redefining new syntaxes for some queries. Each developed and documented feature is accompanied by examples and integration tests in order to facilitate the handling of the driver. To ensure the readability and ease of writing the code while taking into account the fact that we are beginners with the language, some compromises related to performance and the size of the crates had to be made. If you need performance, it is better to use the official language driver or you can join the project and help us to improve the code base.

## Technique

- Add neor crate from gitlab into Cargo.toml
```
// Cargo.toml
...

[dependencies]
neor = { version = "0.0.9", registry = "https://gitlab.com/exytech/community/rethinkdb-for-rust/neor" }
```

- Build and open doc
```bash
$ cargo doc --open
```

## What is RethinkDB?
RethinkDB is the first open-source scalable database built for realtime applications. It exposes a new database access model -- instead of polling for changes, the developer can tell the database to continuously push updated query results to applications in realtime. RethinkDB allows developers to build scalable realtime apps in a fraction of the time with less effort.
