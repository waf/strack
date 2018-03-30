# strack

A slack client written in Rust and GTK+.

## The Stack

- Rust
- GTK+ using [relm](https://github.com/antoyo/relm).
- CSS (whaa? see The FAQ)

## The FAQ

- **Does this work?**
  - No, it currently doesn't work at all. It's not worth trying right now.
- **GTK+? That doesn't look native on all platforms!**
  - Strack solves this by looking native on _no_ platforms! It follows the familiar look-and-feel of the official Slack app.
- **How do I theme strack?**
  - Strack uses CSS for theming! Check out `style.css`, and the [GTK+ guide on CSS](https://developer.gnome.org/gtk3/stable/chap-css-overview.html).

## Help Hack

I'd love help on this, I'm new to rust and I'm sure there's many things to improve. If you want to help out:

1. Clone the repo
2. Install [rust nightly](https://doc.rust-lang.org/book/second-edition/ch01-03-how-rust-is-made-and-nightly-rust.html) and then run `cargo run`.
3. I personally use [IntelliJ Rust](https://intellij-rust.github.io/) for development, but any editor will work fine.
