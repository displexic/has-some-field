# has-some-field

Check whether a struct has any field that is_some().

## Usage

Use the `HasSomeField` derive macro on a struct with optional fields.

- `has_some_field()` - Returns whether any of the fields is_some().
- `some_field_count()` - Returns the number of optional fields that is_some().

## Example

```rust
#[derive(HasSomeField)]
struct TestStruct {
    a: Option<u8>,
    b: Option<u8>,
}
```

```rust
let test_struct = TestStruct {
    a: Some(5),
    b: Some(5),
};
assert_eq!(test_struct.some_field_count(), 2);
```

```rust
let test_struct = TestStruct { a: None, b: None };
assert!(!test_struct.has_some_field());
```

```rust
let test_struct = TestStruct {
    a: Some(5),
    b: None,
};
assert!(test_struct.has_some_field());
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
