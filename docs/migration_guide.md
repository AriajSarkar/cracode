# Migrating from cracode 1 to 2

Cracode 2 now has an optional dependency on `serde`. You can either use `serde`, or use cracode's own `derive` feature and macros.

## From `Options` to `Configuration`

Cracode 1 had the [Options](https://docs.rs/cracode/1/cracode/config/trait.Options.html) trait. This has been replaced with the [Configuration](https://docs.rs/cracode/2/cracode/config/struct.Configuration.html) struct.

If you're using `Options`, you can change it like this:

```rust,ignore
# old
cracode_1::DefaultOptions::new().with_varint_encoding()

# new
cracode_2::config::legacy().with_variable_int_encoding()
```

If you want to be compatible with cracode 1, use the following table:

| Cracode 1                                                              | Cracode 2                                       |
| ---------------------------------------------------------------------- | ----------------------------------------------- |
| version 1.0 - 1.2 with `cracode_1::DefaultOptions::new().serialize(T)` | `config::legacy()`                              |
| version 1.3+ with `cracode_1::DefaultOptions::new().serialize(T)`      | `config::legacy().with_variable_int_encoding()` |
| No explicit `Options`, e.g. `cracode::serialize(T)`                    | `config::legacy()`                              |

If you do not care about compatibility with cracode 1, we recommend using `config::standard()`

The following changes have been made:

- `.with_limit(n)` has been changed to `.with_limit::<n>()`.
- `.with_native_endian()` has been removed. Use `.with_big_endian()` or `with_little_endian()` instead.
- `.with_varint_encoding()` has been renamed to `.with_variable_int_encoding()`.
- `.with_fixint_encoding()` has been renamed to `.with_fixed_int_encoding()`.
- `.reject_trailing_bytes()` has been removed.
- `.allow_trailing_bytes()` has been removed.
- You can no longer (de)serialize from the `Options` trait directly. Use one of the `encode_` or `decode_` methods.

Because of confusion with `Options` defaults in cracode 1, we have made `Configuration` mandatory in all calls in cracode 2.

## Migrating with `serde`

You may wish to stick with `serde` when migrating to cracode 2, for example if you are using serde-exclusive derive features such as `#[serde(deserialize_with)]`.

If so, make sure to include cracode 2 with the `serde` feature enabled, and use the `cracode::serde::*` functions instead of `cracode::*` as described below:

```toml
[dependencies]
cracode = { version = "2.0", features = ["serde"] }

# Optionally you can disable the `derive` feature:
# cracode = { version = "2.0", default-features = false, features = ["std", "serde"] }
```

Then replace the following functions: (`Configuration` is `cracode::config::legacy()` by default)

| Cracode 1                                       | Cracode 2                                                                                                                       |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `cracode::deserialize(&[u8])`                   | `cracode::serde::decode_from_slice(&[u8], Configuration)`<br />`cracode::serde::borrow_decode_from_slice(&[u8], Configuration)` |
| `cracode::deserialize_from(std::io::Read)`      | `cracode::serde::decode_from_std_read(std::io::Read, Configuration)`                                                            |
| `cracode::deserialize_from_custom(CracodeRead)` | `cracode::serde::decode_from_reader(Reader, Configuration)`                                                                     |
|                                                 |                                                                                                                                 |
| `cracode::serialize(T)`                         | `cracode::serde::encode_to_vec(T, Configuration)`<br />`cracode::serde::encode_into_slice(T, &mut [u8], Configuration)`         |
| `cracode::serialize_into(std::io::Write, T)`    | `cracode::serde::encode_into_std_write(T, std::io::Write, Configuration)`                                                       |
| `cracode::serialized_size(T)`                   | Currently not implemented                                                                                                       |

## Migrating from `serde` to `cracode-derive`

`cracode-derive` is enabled by default. If you're using `default-features = false`, make sure to add `features = ["derive"]` to your `Cargo.toml`.

```toml,ignore
[dependencies]
cracode = "2.0"

# If you need `no_std` with `alloc`:
# cracode = { version = "2.0", default-features = false, features = ["derive", "alloc"] }

# If you need `no_std` and no `alloc`:
# cracode = { version = "2.0", default-features = false, features = ["derive"] }
```

Replace or add the following attributes. You are able to use both `serde-derive` and `cracode-derive` side-by-side.

| serde-derive                    | cracode-derive               |
| ------------------------------- | ---------------------------- |
| `#[derive(serde::Serialize)]`   | `#[derive(cracode::Encode)]` |
| `#[derive(serde::Deserialize)]` | `#[derive(cracode::Decode)]` |

**note:** To implement these traits manually, see the documentation of [Encode](https://docs.rs/cracode/2/cracode/enc/trait.Encode.html) and [Decode](https://docs.rs/cracode/2/cracode/de/trait.Decode.html).

**note:** For more information on using `cracode-derive` with external libraries, see [below](#cracode-derive-and-libraries).

Then replace the following functions: (`Configuration` is `cracode::config::legacy()` by default)

| Cracode 1                                       | Cracode 2                                                                                                          |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `cracode::deserialize(&[u8])`                   | `cracode::decode_from_slice(&bytes, Configuration)`<br />`cracode::borrow_decode_from_slice(&[u8], Configuration)` |
| `cracode::deserialize_from(std::io::Read)`      | `cracode::decode_from_std_read(std::io::Read, Configuration)`                                                      |
| `cracode::deserialize_from_custom(CracodeRead)` | `cracode::decode_from_reader(Reader, Configuration)`                                                               |
|                                                 |                                                                                                                    |
| `cracode::serialize(T)`                         | `cracode::encode_to_vec(T, Configuration)`<br />`cracode::encode_into_slice(t: T, &mut [u8], Configuration)`       |
| `cracode::serialize_into(std::io::Write, T)`    | `cracode::encode_into_std_write(T, std::io::Write, Configuration)`                                                 |
| `cracode::serialized_size(T)`                   | Currently not implemented                                                                                          |

### Cracode derive and libraries

Currently not many libraries support the traits `Encode` and `Decode`. There are a couple of options if you want to use `#[derive(cracode::Encode, cracode::Decode)]`:

- Enable the `serde` feature and add a `#[cracode(with_serde)]` above each field that implements `serde::Serialize/Deserialize` but not `Encode/Decode`
- Enable the `serde` feature and wrap your field in [cracode::serde::Compat](https://docs.rs/cracode/2/cracode/serde/struct.Compat.html) or [cracode::serde::BorrowCompat](https://docs.rs/cracode/2/cracode/serde/struct.BorrowCompat.html)
- Make a pull request to the library:
  - Make sure to be respectful, most of the developers are doing this in their free time.
  - Add a dependency `cracode = { version = "2.0", default-features = false, optional = true }` to the `Cargo.toml`
  - Implement [Encode](https://docs.rs/cracode/2/cracode/enc/trait.Encode.html)
  - Implement [Decode](https://docs.rs/cracode/2/cracode/de/trait.Decode.html)
  - Make sure both of these implementations have a `#[cfg(feature = "cracode")]` attribute.
