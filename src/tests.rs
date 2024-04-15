use crate::{Options, RemoveConsole};
use swc_core::ecma::{transforms::testing::test_inline, visit::as_folder};

// console
test_inline!(
    Default::default(),
    |_| as_folder(RemoveConsole {
        options: Options {
            ..Default::default()
        }
    }),
    console,
    r#"let a = 1;
        console.log("hello");
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    let b = 2;"#,
    r#"let a = 1;
        ;;;;
        console.table("table");
    let b = 2;"#
);

// function console
test_inline!(
    Default::default(),
    |_| as_folder(RemoveConsole {
        options: Options {
            ..Default::default()
        }
    }),
    in_function,
    r#";() => {
        console.log('hello');
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    }"#,
    r#";() => {;;;;console.table("table")}"#
);

// pass exclude option
test_inline!(
    Default::default(),
    |_| as_folder(RemoveConsole {
        options: Options {
            exclude: vec!["log".to_string()]
        }
    }),
    pass_option_1,
    r#";() => {
        console.log('hello');
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    }"#,
    r#";() => {console.log('hello');;;;console.table("table")}"#
);

test_inline!(
    Default::default(),
    |_| as_folder(RemoveConsole {
        options: Options {
            exclude: vec!["warn".to_string()]
        }
    }),
    pass_option_2,
    r#";() => {
        console.log('hello');
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    }"#,
    r#";() => {;console.warn("warn");;;console.table("table")}"#
);

test_inline!(
    Default::default(),
    |_| as_folder(RemoveConsole {
        options: Options {
            exclude: vec![
                "log".to_string(),
                "warn".to_string(),
                "error".to_string(),
                "info".to_string()
            ]
        }
    }),
    pass_option_3,
    r#";() => {
        console.log('hello');
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    }"#,
    r#";() => {
        console.log('hello');
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    }"#
);

test_inline!(
    Default::default(),
    |_| as_folder(RemoveConsole {
        options: Options {
            exclude: vec!["foo".to_string(),]
        }
    }),
    pass_option_4,
    r#";() => {
        console.log('hello');
        console.warn("warn");
        console.error("error");
        console.info("info");
        console.table("table");
    }"#,
    r#";() => {
        ;;;;
        console.table("table");
    }"#
);
