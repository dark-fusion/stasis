<div align="center">
 <h1>Stasis</h1>
  <h3>Fast and concurrent key-value store</h3>
  <p>Blue skies research project for studying concurrency and parallelism</p>
  <div style="width:fit-content">
The official documentation is available on <a href="https://docs.rs/stasis-rs">docs.rs</a>

  </div>  
</div>

## Purpose

Stasis initially served two main purposes:

1. Provide a knowledge base and environment for experimentation
   with [concurrency and parallelism][concurrency] in Rust.
2. Use the knowledge gained to build something practical and useful.

### Results

<!-- TODO: Link to blog post -->
After some experimentation (including various side-projects), it became apparent that the Rust
programming language possesses unique properties that make it an excellent choice for certain
projects.

The language design and features of Rust make it particularly suitable for projects that benefit
from the abstractions traditionally provided by "high-level" languages and the fine-grained control
that exists within "low-level" languages.

An in-memory database (backed by a key-value store) was chosen as it meets the criteria briefly
outlined above.

## Project Description

Stasis is an in-memory database backed by a key-value store. It borrows many ideas from Redis and
other existing solutions.

### Project Roadmap

The following features will be included in the MVP:

- [ ] Protocol and Codec
    - [ ] Deserialization
        - [ ] Parse _untrusted_ client requests
    - [ ] Serialization
        - [ ] Send _appropriate_ responses to clients
- [ ] Telemetry
    - [ ] Console logger
    - [ ] File logger
- [ ] Storage engine
    - [ ] Ephemeral, in-memory key-value store
    - [ ] LRU-based cache eviction policy for cleanup
    - [ ] Thread-safe data structure ([Mutex][std::sync::Mutex], [Arc][std::sync::Arc], etc.)
- [ ] HTTP / TCP **server**
- [ ] HTTP / TCP **client**

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in `stasis` by you, shall be licensed as MIT, without any additional terms or conditions.

Please review the [contribution guidelines][contributing].

## Licensing

This project is licensed under the [MIT license][license].

<!-- Links -->

[contributing]: /CONTRIBUTING.md

[documentation]: https://docs.rs/stasis

[license]: /LICENSE

[issue]: https://github.com/dark-fusion/stasis/issues/new

[concurrency]: https://doc.rust-lang.org/book/ch16-00-concurrency.html

[std::sync::Mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html

[std::sync::Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
