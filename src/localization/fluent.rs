#[macro_export]
macro_rules! fprint {
    ($fd:expr, $output:expr) => {{
        let _ = $crate::wutil::unescape_bytes_and_write_to_fd($output, $fd);
    }};
}
pub use fprint;

#[macro_export]
macro_rules! localized_fprint {
    ($fd:expr, $id:expr, $args:expr) => {
        $crate::localization::fprint!($fd, &fish_fluent::localize!($id, $args))
    };
    ($fd:expr, $id:expr $(, $key:ident = $value:expr)* $(,)?) => {
        $crate::localization::fprint!($fd, &fish_fluent::localize!($id $(, $key = $value)*))
    };
}
pub use localized_fprint;

#[macro_export]
macro_rules! localized_fprintln {
    ($fd:expr, $id:expr, $args:expr) => {{
        $crate::localization::localized_fprint!($fd, $id, $args);
        $crate::wutil::write_newline_to_fd($fd);
    }};
    ($fd:expr, $id:expr $(, $key:ident = $value:expr)* $(,)?) => {{
        $crate::localization::localized_fprint!($fd, $id $(, $key = $value)*);
        $crate::wutil::write_newline_to_fd($fd);
    }};
}
pub use localized_fprintln;

#[macro_export]
macro_rules! localized_print {
    ($id:expr, $args:expr) => {
        $crate::localization::localized_fprint!(libc::STDOUT_FILENO, $id, $args)
    };
    ($id:expr $(, $key:ident = $value:expr)* $(,)?) => {
        $crate::localization::localized_fprint!(libc::STDOUT_FILENO, $id $(, $key = $value)*)
    };
}
pub use localized_print;

#[macro_export]
macro_rules! localized_println {
    ($id:expr, $args:expr) => {{
        $crate::localization::localized_fprintln!(libc::STDOUT_FILENO, $id, $args);
    }};
    ($id:expr $(,$key:ident = $value:expr)* $(,)?) => {{
        $crate::localization::localized_fprintln!(libc::STDOUT_FILENO, $id $(, $key = $value)*);
    }};
}
pub use localized_println;

#[macro_export]
macro_rules! localized_eprint {
    ($id:expr, $args:expr) => {
        $crate::localization::localized_fprint!(libc::STDERR_FILENO, $id, $args)
    };
    ($id:expr $(, $key:ident = $value:expr)* $(,)?) => {
        $crate::localization::localized_fprint!(libc::STDERR_FILENO, $id $(, $key = $value)*)
    };
}
pub use localized_eprint;

#[macro_export]
macro_rules! localized_eprintln {
    ($id:expr, $args:expr) => {{
        $crate::localization::localized_fprintln!(libc::STDERR_FILENO, $id, $args);
    }};
    ($id:expr $(, $key:ident = $value:expr)* $(,)?) => {{
        $crate::localization::localized_fprintln!(libc::STDERR_FILENO, $id $(, $key = $value)*);
    }};
}
pub use localized_eprintln;

#[macro_export]
macro_rules! localized_format {
    ($id:expr, $args:expr) => {
        fish_fluent::localize!($id, $args)
    };
    ($id:expr $(, $key:ident = $value:expr)* $(,)?) => {
        fish_fluent::localize!($id $(, $key = $value)*)
    };
}
pub use localized_format;

pub fn append_newline<S: Into<String>>(s: S) -> String {
    let mut s: String = s.into();
    s.push('\n');
    s
}

#[macro_export]
macro_rules! localized_formatln {
    ($id:expr, $args:expr) => {
        $crate::localization::append_newline(fish_fluent::localize!($id, $args))
    };
    ($id:expr $(, $key:ident = $value:expr)* $(,)?) => {
        $crate::localization::append_newline(fish_fluent::localize!($id $(, $key = $value)*))
    };
}
pub use localized_formatln;

#[cfg(test)]
mod tests {
    use fish_fluent::fluent_ids;
    use serial_test::serial;

    #[test]
    #[serial]
    fn without_args() {
        fluent_ids! {ID "test"}
        localized_print!(ID);
        localized_println!(ID);
        localized_eprint!(ID);
        localized_eprintln!(ID);
        localized_fprint!(libc::STDOUT_FILENO, ID);
        localized_fprintln!(libc::STDOUT_FILENO, ID);
        assert_eq!(localized_format!(ID), "This is a test");
        assert_eq!(localized_formatln!(ID), "This is a test\n");
    }

    #[test]
    #[serial]
    fn with_args() {
        fluent_ids! {ID "test-with-args"}
        localized_print!(ID, first = 1, second = "two");
        localized_println!(ID, first = 1, second = "two");
        localized_eprint!(ID, first = 1, second = "two");
        localized_eprintln!(ID, first = 1, second = "two");
        localized_fprint!(libc::STDOUT_FILENO, ID, first = 1, second = "two");
        localized_fprintln!(libc::STDOUT_FILENO, ID, first = 1, second = "two");
        assert_eq!(
            localized_format!(ID, first = 1, second = "two"),
            "Two arguments: 1, two"
        );
        assert_eq!(
            localized_formatln!(ID, first = 1, second = "two"),
            "Two arguments: 1, two\n"
        );

        let mut args = fluent::FluentArgs::new();
        args.set("first", 1);
        args.set("second", "two");

        localized_print!(ID, &args);
        localized_println!(ID, &args);
        localized_eprint!(ID, &args);
        localized_eprintln!(ID, &args);
        localized_fprint!(libc::STDOUT_FILENO, ID, &args);
        localized_fprintln!(libc::STDOUT_FILENO, ID, &args);
        assert_eq!(localized_format!(ID, &args), "Two arguments: 1, two");
        assert_eq!(localized_formatln!(ID, &args), "Two arguments: 1, two\n");
    }
}
