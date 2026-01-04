// Generated Macro Labels

mkrust!(
    feature!(control_flow);
    flags = ["cfg(control_flow)"];
    functions = ["parse_if", "parse_match", "compile_branch"];
    deps = ["expr"];
);

mkrust!(
    feature!(expr);
    flags = ["cfg(expressions)"];
    functions = ["parse_expr", "compile_binop"];
    deps = ["bool"];
);

mkrust!(
    feature!(functions);
    flags = ["cfg(functions)"];
    functions = ["parse_fn", "compile_call", "type_check"];
    deps = ["control_flow"];
);

mkrust!(
    feature!(core);
    flags = ["cfg(core)"];
    functions = ["parse_const", "compile_literal"];
    deps = [];
);

mkrust!(
    feature!(bool);
    flags = ["cfg(bool_support)"];
    functions = ["parse_bool", "compile_bool"];
    deps = ["core"];
);

