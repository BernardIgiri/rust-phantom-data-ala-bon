# Rust Phantom Data Demo

This repository demonstrates how to use phantom data in Rust, showcasing its application in libraries like
[Bon](https://elastio.github.io/bon) to enforce compile-time checks on method calls.

## Overview

The `bon` library allows you to create builder objects that require specific fields to be populated before
calling `build()`. By leveraging phantom data, `bon` ensures that these checks occur at compile time,
preventing incomplete builders from being constructed.

For example, suppose you have a builder that requires fields `a`, `b`, and `c` to be set, while field `d`
is optional. The following code will compile successfully:

```rust
let foo = Foo::builder().a(a_value).b(b_value).c(c_value).build();
let foo2 = Foo::builder().a(a_value).b(b_value).c(c_value).d(d_value).build();
```

In contrast, the following will fail at compile time because one or more required fields are missing:

```rust
let foo = Foo::builder().b(b_value).c(c_value).build();
let foo2 = Foo::builder().a(a_value).c(c_value).build();
let foo3 = Foo::builder().a(a_value).b(b_value).build();
```

## How It Works

Each method call on the builder returns a new object, with phantom data tracking which fields have been set.
This phantom data, managed via Rust's type system and generics, ensures that the correct set of fields is
provided before the builder can be finalized with `build()`.

The use of phantom data enforces these constraints without adding runtime overhead, allowing developers to
catch errors early in the development process.
