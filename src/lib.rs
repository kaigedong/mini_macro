/// Enhanced location macro with support for custom messages
///
/// Output format: `file_path:line:column[: custom_message]`
///
/// # Features
/// - No arguments: Returns basic location info (`file:line:column`)
/// - Single argument: Location + custom message
/// - Multiple arguments: Location + formatted message (supports `format!` syntax)
///
/// # Return Type
/// - No arguments: `&'static str` (compile-time static string)
/// - With arguments: `String` (runtime-generated dynamic string)
///
/// # Examples
///
/// ## Basic usage
/// ```rust
/// println!("Location: {}", here!());
/// // Example output: src/main.rs:42:10
/// ```
///
/// ## With static message
/// ```rust
/// println!("Error: {}", here!("Something went wrong"));
/// // Example output: src/main.rs:42:10: Something went wrong
///
/// // Trailing comma supported
/// println!("Error: {}", here!("Something went wrong",));
/// ```
///
/// ## Dynamic formatted message
/// ```rust
/// let line_num = 100;
/// println!("Warning: {}", here!("Check line {}", line_num));
/// // Example output: src/main.rs:42:10: Check line 100
///
/// // Complex formatting
/// println!("Error: {}", here!("User {} not found, status: {}", "Alice", 404));
/// ```
#[macro_export]
macro_rules! here {
    // No arguments version: returns static string
    () => {
        concat!(file!(), ":", line!(), ":", column!())
    };

    // Single argument version: static string + literal message
    ($msg:literal $(,)?) => {
        concat!(file!(), ":", line!(), ":", column!(), ": ", $msg)
    };

    // Multiple arguments version: dynamic formatted message
    ($fmt:expr, $($arg:tt)*) => {
        format!(
            "{}:{}:{}: {}",
            file!(),
            line!(),
            column!(),
            format_args!($fmt, $($arg)*)
        )
    };
}
