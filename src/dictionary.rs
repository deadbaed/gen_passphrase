/*!

Built-in dictionaries, ready to be used

By default, this module will be empty. Each built-in dictionary is hidden behind a feature, usually named after the dictionary.
Dictionaries are hidden behind features in order keep the crate lightweight to compile.

# Example

When the feature `eff_short_2` is enabled, you can use the dictionary directly:

```
# #[cfg(feature = "eff_short_2")]
use gen_passphrase::dictionary::EFF_SHORT_2;

# #[cfg(feature = "eff_short_2")]
let passphrase = gen_passphrase::generate(&[EFF_SHORT_2], 1, None);
```

# Compiler errors

If you do not have the feature enabled for your dictionary, you might get a compiler error like this:

```text
error[E0432]: unresolved import `gen_passphrase::dictionary::EFF_SHORT_2`
  |
  | use gen_passphrase::dictionary::EFF_SHORT_2;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------
  |     |                           |
  |     |                           help: a similar name exists in the module: `eff_short_2`
  |     no `EFF_SHORT_2` in `dictionary`
```

Enable the feature associated with the dictionary to use it.

*/

mod bip39_english;
mod bip39_french;
mod eff_large;
mod eff_short_1;
mod eff_short_2;

#[cfg(feature = "eff_short_2")]
pub use eff_short_2::EFF_SHORT_2;

#[cfg(feature = "eff_short_1")]
pub use eff_short_1::EFF_SHORT_1;

#[cfg(feature = "eff_large")]
pub use eff_large::EFF_LARGE;

#[cfg(feature = "bip39_english")]
pub use bip39_english::BIP39_ENGLISH;

#[cfg(feature = "bip39_french")]
pub use bip39_french::BIP39_FRENCH;
