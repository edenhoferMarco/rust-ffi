# rust-ffi

## Abstract
This is a sample project to showcase two scenarios:
1) How a C-ABI compatible library (rust-c-lib)[rust-c-lib]
   can be built
2) How to call a C-ABI compatible library from rust
   (rust-client)[rust-client]

## How to build
A simple `cargo build --release` is enough. This will build
both projects including generating the .h files from
the lib (see (rust-c-lib/build.rs)[rust-c-lib/build.rs])
and also generate the rust bindings out of the generated 
.h file see (rust-client/build.rs)[rust-client/build.rs])

## What's next?
Use the client to play around with it.

Have fun :)