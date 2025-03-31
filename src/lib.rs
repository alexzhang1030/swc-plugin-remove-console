use serde::Deserialize;
use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{CallExpr, MemberExpr, Program, Stmt},
        visit::{visit_mut_pass, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[cfg(test)]
mod tests;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Options {
    exclude: Vec<String>,
}

const SPECIFY_SUBCOMMAND: [&str; 4] = ["log", "warn", "error", "info"];

pub struct RemoveConsole {
    options: Options,
}

impl RemoveConsole {
    fn is_console(&self, expr: &MemberExpr) -> bool {
        let obj = &expr.obj;
        obj.as_ident().map_or(false, |ident| ident.sym == "console")
    }

    fn is_specify_subcommand(&self, expr: &MemberExpr) -> bool {
        let prop = &expr.prop;
        return SPECIFY_SUBCOMMAND.iter().any(|command| {
            if self.options.exclude.contains(&command.to_string()) {
                return false;
            }
            prop.as_ident().map_or(false, |ident| ident.sym == *command)
        });
    }

    fn should_remove(&self, e: &CallExpr) -> bool {
        if e.callee.is_expr() {
            if let Some(expr) = e.callee.as_expr() {
                if let Some(expr) = expr.as_member() {
                    return self.is_console(expr) && self.is_specify_subcommand(expr);
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

#[plugin_transform]
pub fn remove_console(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let options = metadata
        .get_transform_plugin_config()
        .map(|json| {
            serde_json::from_str::<Options>(&json)
                .expect("failed to deserialize options for remove-console plugin")
        })
        .unwrap_or_default();
    program.apply(&mut visit_mut_pass(RemoveConsole { options }))
}
