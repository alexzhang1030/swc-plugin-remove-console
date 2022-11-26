use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{CallExpr, EmptyStmt, MemberExpr, Stmt},
        transforms::testing::test,
        visit::{as_folder, VisitMut, VisitMutWith},
    },
};

pub struct RemoveConsole;

fn is_console(expr: &MemberExpr) -> bool {
    return expr.obj.clone().ident().unwrap().sym.eq("console");
}

fn is_log(expr: &MemberExpr) -> bool {
    return expr.prop.clone().ident().unwrap().sym.eq("log");
}

fn should_remove(e: &CallExpr) -> bool {
    if e.callee.is_expr() {
        let member_expr = e.callee.as_expr().unwrap().as_member().unwrap();
        if is_console(member_expr) {
            if is_log(member_expr) {
                return true;
            }
        }
    }
    return false;
}

impl VisitMut for RemoveConsole {
    fn visit_mut_stmt(&mut self, e: &mut Stmt) {
        e.visit_mut_children_with(self);

        if let Some(expr_stmt) = e.as_expr() {
            if let Some(e_2) = expr_stmt.expr.as_call() {
                if should_remove(e_2) {
                    println!("should remove !!!");
                    *e = Stmt::Empty(EmptyStmt { span: DUMMY_SP });
                }
            }
        }
    }
}

test!(
    Default::default(),
    |_| as_folder(RemoveConsole),
    remove_1,
    r#"let a = 1;console.log("hello")"#,
    r#"let a = 1;;"#
);

test!(
    Default::default(),
    |_| as_folder(RemoveConsole),
    remove_2,
    r#";() => { console.log('hello') }"#,
    r#";() => {;}"#
);
