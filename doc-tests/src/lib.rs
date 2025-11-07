//! Example of using doc-tests.

/// Added code samples are automatically turned into tests.
/// Paths need to be global, hence only `pub` features could be tested, and only in libs.
///
/// Markdown-fenced code blocks can be used:
/// ```
/// assert_eq!(doc_tests::add(3, 2), 5);
/// assert_eq!(doc_tests::add(3, -3), 0);
/// ```
///
/// Or indentation:
///
///     assert_eq!(doc_tests::add(1, 2), 3);
///     assert_eq!(doc_tests::add(-1, -2), -3);
///
pub fn add(left: i64, right: i64) -> i64 {
    left + right
}
