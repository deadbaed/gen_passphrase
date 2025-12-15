# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Unreleased

## 0.2.0 - 2025-12-15

### Added

- New dictionaries: BIP 39 [English](https://raw.githubusercontent.com/bitcoin/bips/870c7629aee3dbd0fea1932e498f588dc9421497/bip-0039/english.txt) and [French](https://raw.githubusercontent.com/bitcoin/bips/870c7629aee3dbd0fea1932e498f588dc9421497/bip-0039/french.txt), available as features `bip39_english` and `bip39_french`.
- Expose function to choose a random word from a dictionary, `gen_passphrase::choose_random_word()`

### Changed

- Update dependency `nanorand` to version 0.8.
- Function `gen_passphrase::generate()` returns a `Option<String>` instead of `String`, to handle the case where provided dictionaries are empty.

## 0.1.0 - 2023-04-10

Initial release
