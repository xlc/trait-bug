Crate `foo` failed to compile with this error;

```
$ cargo check
    Checking foo v0.1.0 (/Users/xiliangchen/projects/acala/trait-bug/foo)
error[E0119]: conflicting implementations of trait `From<Foo>` for type `Foo`
  --> foo/src/lib.rs:11:1
   |
11 | impl From<<baz::Baz as ::baz::BazTrait>::BazType> for Foo {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `core`:
           - impl<T> From<T> for T;

For more information about this error, try `rustc --explain E0119`.
error: could not compile `foo` (lib) due to previous error
```

And it works if `Baz` is defined in a same crate with `Foo`.
