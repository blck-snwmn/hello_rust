/// Rust は doctest を行える
/// 以下のようにコードブロックに記載しているコードが
/// `cargo test`で実行される
///
/// doctest is excluded from bin.
/// see: https://github.com/rust-lang/rust/issues/50784
/// ```
/// assert_eq!(hello_world::retutn_len("abc"), 3);
/// ```
pub fn retutn_len(s: &str) -> usize {
    s.len()
}
