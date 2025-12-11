# gen_passphrase

![CI status](https://github.com/x4m3/gen_passphrase/actions/workflows/ci.yml/badge.svg)
[![crates.io version](https://img.shields.io/crates/v/gen_passphrase)](https://crates.io/crates/gen_passphrase)
[![docs.rs](https://img.shields.io/docsrs/gen_passphrase)](https://docs.rs/gen_passphrase)

A secure, simple yet customizable passphrase generator (in Rust). Use provided dictionaries or bring your own!

## Built-in dictionaries

Some dictionaries are built-in, to make passphrase generation easy.
They are hidden behind rust features, in order to keep the crate size small.

List of built-in dictionaries:

| Dictionary           | Rust Feature to enable |
|----------------------|------------------------|
| Eff Short Wordlist 2 | `eff_short_2`          |
| Eff Short Wordlist 1 | `eff_short_1`          |
| Eff Large Wordlist   | `eff_large`            |

### Prepare new built-in dictionary

A small program to generate dictionaries is provided in [create_dictionary_from_file](./create_dictionary_from_file).
