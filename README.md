# Smart Pointers and Interior Mutability

This repository contains educational implementations of Rust smart pointers and interior mutability primitives, following the concepts from Jon Gjengset's **Crust of Rust** series.

## Implementations

- **`Cell<T>`**: Simple interior mutability for `Copy` types.
- **`RefCell<T>`**: Dynamic borrowing rules enforced at runtime.
- **`Rc<T>`**: Single-threaded reference counting for shared ownership.

## Learning Resources

These implementations were inspired by and learned from the following video:
- [Crust of Rust: Smart Pointers and Interior Mutability](https://www.youtube.com/watch?v=8O0Nt9qY_vo&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=7)

## Usage

This project is intended for educational purposes. To run the tests:

\`\`\`bash
cargo test
\`\`\`
