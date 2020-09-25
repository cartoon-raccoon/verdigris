# Verdigris
An interpreted programming language based on and implemented in Rust

Verdigris is a programming language based on the Rust syntax and language features, but is an interpreted, garbage-collected scripting language.
It is intended to be an easier to learn version of Rust with dynamic typing, while still retaining the features that make Rust such an expressive language to use.
It strives to be to Rust what Python is to C.

Features retained from Rust include:
- Structs and methods
- Traits
- Closures
- Attributes (maybe)

Some features from the Rust stdlib will be core parts of Verdigris. Right now, only vecs, tuples, hashmaps and hashsets are planned.

Verdigris right now is planned to be dynamically typed, but this may change in the future.

Features such as lifetimes and ownership/borrowing are not needed here, as the language is garbage collected and memory management is handled by the runtime.

Note: This language is simply a passion project, and is not intended for production usage. It doesn't have that much of a USP anyway.
To this end, a standard library is _not_ planned, and may or may not be implemented in the future.
However, core data structures and their methods will be implemented.
