# Stasis

Prototype implementations of concurrent data structures.

## Purpose

This repository is a tool for brainstorming and is intended to produce useful implementations that
can be used in real projects.

## Building Blocks

Currently, the basic building blocks used are concurrency-based primitives and data structures such
as:

- `Arc`: atomically reference-counted pointers
- `Mutex`: mutual exclusion locks
- `Condvar`: condition variables

## Acknowledgements

- Jon Gjengset:
  - multi-producer single-consumer channel implementation: [video](https://youtu.be/b4mS5UPHh20)

## Licensing

This product is licensed under the [MIT license](LICENSE).
