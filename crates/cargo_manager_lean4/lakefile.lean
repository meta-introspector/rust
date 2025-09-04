import Lake

open Lake DSL

package cargo_manager_lean4 {
  -- add package configuration here
}

@[default_target]
lean_lib CargoManagerLean4 {
  -- add library configuration here
}

lean_exe cargo_manager_lean4 {
  root := `Main`
}

require mathlib from git "https://github.com/leanprover-community/mathlib4" @ "master"
require groupoid_model_in_lean4 from git "https://github.com/meta-introspector/groupoid_model_in_lean4" @ "master"

