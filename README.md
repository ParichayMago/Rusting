here is my everyday report of learning rust

Learning basic programming Syntax of the Rust.

Learning goal is to make stack web-application using Rust

using lyb like actix, actix-web, Dioxus

## what we know about Rust

Rust use borow and owning

evey varable has its lifetime inside a {}


imp:
  str:
    str is an immutable string slice, also known as a string literal or string reference.
    It's a view into a sequence of UTF-8 bytes stored elsewhere, such as in the data segment of the program's binary or on the heap.
    It cannot be mutated directly because it's immutable.
    str is commonly used for string literals and string slices.
    Example: let s: &str = "hello";

  String:
    String is a growable, heap-allocated string.
    It's mutable and can be modified or extended after creation.
    String owns its data, which is stored on the heap, and it has the ability to grow or shrink dynamically as needed.
    String is commonly used when you need to manipulate or modify strings at runtime.
    Example: let s: String = String::from("hello");