![ci](https://github.com/emef/prost-arrow/actions/workflows/ci.yml/badge.svg)
[![Documentation](https://docs.rs/prost-arrow/badge.svg)](https://docs.rs/prost-arrow/)
[![Crate](https://img.shields.io/crates/v/prost-arrow.svg)](https://crates.io/crates/prost-arrow)

# PROST! Apache Arrow Support

`prost-arrow` provides a `derive` trait that can be used to generate `arrow`
array builders for any protobuf types generated using
[prost](https://github.com/tokio-rs/prost/tree/master).

## Usage

This crate provides the `ToArrow` trait and a proc-macro to derive it. It must
be derived on _all_ messages, so we add it as a `type_attribute` with the
catch-all path `"."`. The generated `impls` depend on both the `prost-arrow`
crate as well as a few `arrow` crates.

You will need to add the following dependencies to your Cargo.toml:

```rust
arrow-array
arrow-buffer
arrow-schema
prost-arrow
```

In your build script:

```rust
// prost
prost_build::Config::new()
    .type_attribute(".", "#[derive(::prost_arrow::ToArrow)]")
    .compile_protos(&["proto/routeguide/route_guide.proto"], &["proto/"])
    .unwrap();

// tonic
tonic_build::configure()
    .type_attribute(".", "#[derive(::prost_arrow::ToArrow)]")
    .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
    .unwrap();
```

Finally, to access the array builder for a generated prost type, we use
`prost_arrow::new_builder<T>` for some prost-generated type `T` that has the
`ToArrow` type derived. The builder returned will implement the base
`arrow_builder::Builder` trait, but will also have an `append` and
`append_option` method that accepts our prost type `T`.

```rust
// Rectangle is a prost-generated struct that has ToArrow derived.
let mut builder = prost_arrow::new_builder::<Rectangle>();

builder.append(Some(Rectangle {
    lo: Some(pt_1),
    hi: None,
    messages: vec!["one".to_string(), "two".to_string()],
    extra_points: vec![
        Point {
            latitude: 1,
            longitude: 2,
        },
        Point {
            latitude: 3,
            longitude: 4,
        },
    ],
    binary: vec![0, 1, 2, 3],
    repeated_binary: vec![vec![10, 100]],
}));
```

The builder can be used just like any other arrow builder implementation type,
so the `finish` or `finish_cloned` methods can be used to finalize the arrow
array (in our case, a struct array).

```rust

// finish the array builder to get an ArrayRef
let arr = builder.finish();

// downcast the array into StructArray
let struct_arr = arr.as_any().downcast_ref::<StructArray>().unwrap();

// convert to RecordBatch if desired
let record_batch: RecordBatch = struct_arr.into();
```

## Completeness

| feature                             | supported |
| ----------------------------------- | --------- |
| primitive types                     | ✅        |
| repeated fields                     | ✅        |
| optional fields (via `optional`)    | ✅        |
| optional fields (via wrapper types) | 🚧        |
| well-known types (e.g. timestamp)   | 🚧        |
| oneof fields                        | 🚧        |
| map fields                          | 🚧        |
| nested messages                     | ✅        |
| recursive/cyclic messages           | ❌        |
