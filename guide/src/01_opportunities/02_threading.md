# Learning Opportunities

## Threading

Rust has a powerful threading model that allows you to write concurrent code safely. This is a great opportunity to learn how to use it. Here we want to suggest the crate `rayon` that will likely lead you down a path where you will learn about `std::sync::Arc`, `std::sync::RwLock` and `std::sync::mpsc::channel` and a likes.

rayon is a data parallelism library for Rust. It is a simple and efficient library that allows you to write parallel code that is easy to read and understand. It is a great way to get started with parallel programming in Rust.

Please note that parallelism of IO is a tricky, so you might seek help of channels to communicate between threads.

