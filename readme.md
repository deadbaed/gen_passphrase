# gen_passphrase

![CI status](https://github.com/x4m3/gen_passphrase/actions/workflows/ci.yml/badge.svg)
[![crates.io version](https://img.shields.io/crates/v/gen_passphrase)](https://crates.io/crates/gen_passphrase)
[![docs.rs](https://img.shields.io/docsrs/gen_passphrase)](https://docs.rs/gen_passphrase)

A secure, simple yet customizable passphrase generator (in Rust). Use provided dictionaries or bring your own!

<!-- cargo-rdme start -->


## Generate passphrase

This crate allows to [generate] passphrases easily and securely.

Choose the source of words to use in passphrases: either use [built-in dictionaries](https://docs.rs/gen_passphrase/latest/gen_passphrase/dictionary/) or provide your own dictionary!

By default, the crate does not come with any built-in dictionaries in order to keep the crate small.
Built-in dictionaries can be added through the usage of features in the crate.

### Custom dictionary

```rust
let dictionary = &["hello", "bonjour", "hola", "ciao"];

// Example of generated passphrase: "bonjour-hello"
let passphrase = gen_passphrase::generate(&[dictionary], 2, Some("-"));
```

### Built-in dictionary

To enable [built-in dictionaries](https://docs.rs/gen_passphrase/latest/gen_passphrase/dictionary/), you need to enable the feature corresponding to the requested dictionary.

| Dictionary | Feature to enable |
|------------|-------------------|
| Eff Short Wordlist 2 | `eff_short_2` |
| Eff Short Wordlist 1 | `eff_short_1` |
| Eff Large Wordlist | `eff_large` |
| BIP39 English | `bip39_english` |
| BIP39 French | `bip39_french` |

Then, use the dictionary like you would use a custom dictionary:

```rust
use gen_passphrase::dictionary::EFF_SHORT_2;

let passphrase = gen_passphrase::generate(&[EFF_SHORT_2], 1, None);
```

<!-- cargo-rdme end -->

## Prepare new built-in dictionary

A small program to generate dictionaries is provided in [create_dictionary_from_file](./create_dictionary_from_file).

## Development

Use [cargo-rdme](https://github.com/orium/cargo-rdme) to generate part of the readme from [lib.rs](src/lib.rs). For nix users, a nix shell file is included in this repository.
