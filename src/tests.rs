use crate::RemoveConsole;
use swc_core::ecma::{transforms::testing::test, visit::as_folder};

// console
test!(
    Default::default(),
    |_| as_folder(RemoveConsole),
    console,
    r#"let a = 1;
        console.log("hello");
    let b = 2;"#,
    r#"let a = 1;
        ;
    let b = 2;"#
);

// function console
test!(
    Default::default(),
    |_| as_folder(RemoveConsole),
    in_function,
    r#";() => { 
        console.log('hello') 
    }"#,
    r#";() => {;}"#
);
