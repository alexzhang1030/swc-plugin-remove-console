use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{CallExpr, Ident, MemberExpr, Stmt},
        visit::{VisitMut, VisitMutWith},
    },
};

#[cfg(test)]
mod tests;

pub struct RemoveConsole;

impl RemoveConsole {
    fn eq(&self, ident: Option<&Ident>, eq_name: &str) -> bool {
        if let Some(ident) = ident {
            return ident.sym.eq(eq_name);
        }
        false
    }

    fn is_console(&self, expr: &MemberExpr) -> bool {
        let obj = &expr.obj;
        self.eq(obj.as_ident(), "console")
    }

    fn is_log(&self, expr: &MemberExpr) -> bool {
        let prop = &expr.prop;
        self.eq(prop.as_ident(), "log")
    }

    fn should_remove(&self, e: &CallExpr) -> bool {
        if e.callee.is_expr() {
            if let Some(expr) = e.callee.as_expr() {
                if let Some(expr) = expr.as_member() {
                    return self.is_console(expr) && self.is_log(expr);
                }
            }
        }
        false
    }
}

impl VisitMut for RemoveConsole {
    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_mut_children_with(self);

        if let Stmt::Expr(expr) = stmt {
            if let Some(call_expr) = expr.expr.as_call() {
                if self.should_remove(call_expr) {
                    stmt.take();
                }
            }
        }
    }
}
