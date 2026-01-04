// Generated Feature Flags

#[cfg(control_flow)]
mod control_flow {
    pub fn parse_if() { /* implementation */ }
    pub fn parse_match() { /* implementation */ }
    pub fn compile_branch() { /* implementation */ }
}

#[cfg(expressions)]
mod expr {
    pub fn parse_expr() { /* implementation */ }
    pub fn compile_binop() { /* implementation */ }
}

#[cfg(functions)]
mod functions {
    pub fn parse_fn() { /* implementation */ }
    pub fn compile_call() { /* implementation */ }
    pub fn type_check() { /* implementation */ }
}

#[cfg(core)]
mod core {
    pub fn parse_const() { /* implementation */ }
    pub fn compile_literal() { /* implementation */ }
}

#[cfg(bool_support)]
mod bool {
    pub fn parse_bool() { /* implementation */ }
    pub fn compile_bool() { /* implementation */ }
}

