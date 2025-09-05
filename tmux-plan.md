# context
 
HOME : /data/data/com.termux/files/home
GITHUB ROOT : /data/data/com.termux/files/home/storage/github/
all files in here are allowed to be read.
PWD : /data/data/com.termux/files/home/storage/github/rustc/sessions/tmux
origin :https://github.com/meta-introspector/rust
 
# Plan for Unifying Tmux-Related Files

pwd:

2.  **Read File List:** Read the content of to get all relative to ~/storage/github/rustc/ file paths.

## DOCS
./crates/introspector/vendor/libminizinc/change_request_launch_gemini_tmux.md
./crates/introspector/vendor/libminizinc/change_request_tmux_task_execution.md
./crates/introspector/vendor/libminizinc/crates/dump_tmux_status/
./crates/introspector/vendor/libminizinc/crates/launchpad/src/stages/tmux_controller_cmd_stage.rs
./crates/introspector/vendor/libminizinc/crates/launchpad/src/stages/tmux_stage.rs
./crates/introspector/vendor/libminizinc/crates/solfunmeme-core/src/stages/tmux_controller_cmd_stage.rs
./crates/introspector/vendor/libminizinc/crates/solfunmeme-core/src/stages/tmux_stage.rs
./crates/introspector/vendor/libminizinc/crates/solfunmeme-core/src/tmux_controller_commands/capture_session_output.rs
./crates/introspector/vendor/libminizinc/crates/tmux_controller/Cargo.toml
./crates/introspector/vendor/libminizinc/crates/tmux_controller/src/main.rs
./crates/introspector/vendor/libminizinc/crq_tmux_view_tool.md
./crates/introspector/vendor/libminizinc/crq_tmux_watch_workflow.md
./crates/introspector/vendor/libminizinc/docs/architecture/c4_model_launchpad_tmux.md
./crates/introspector/vendor/libminizinc/docs/cheatsheets/tmux_noob_cheatsheet.md
./crates/introspector/vendor/libminizinc/docs/cli_arguments/tmux_controller_cli_arguments.md
./crates/introspector/vendor/libminizinc/docs/commits/018333aec1ad92675a664ff64bd615ff301eb086_feat_Implement_new_three-pane_tmux_layout_in_tmux_controller.md
./crates/introspector/vendor/libminizinc/docs/commits/0cbb28d534ed2f7b056adaaeff81cbd0e82d87f6_feat_Enhance_tmux_controller_with_comprehensive_session_management_introduce_operational_workflow_CRQ.md
./crates/introspector/vendor/libminizinc/docs/commits/1f0549476cd5f50c4824605d5e508286eed79fa4_refactor_launchpad_stage_system_and_tmux_controller_layout_creation.md
./crates/introspector/vendor/libminizinc/docs/commits/23104bac1cf99fa82e998471ac;1f929724700122_feat_Enhance_launchpad_and_tmux_controller_CLI_with_new_arguments_and_documentation.md
./crates/introspector/vendor/libminizinc/docs/commits/4c692c2827e9ca0bf9b54f56cc1d370002124060_refactor_launchpad_stage_system_and_tmux_controller_layout_creation.md
./crates/introspector/vendor/libminizinc/docs/commits/6825742237cf46e25273a1ab6418298857ac40f5_docs_Add_n00b_onboarding_guide_for_tmux_controller_--task.md
./crates/introspector/vendor/libminizinc/docs/commits/883eb8c29b2d6f26fa7ea8959714c25aaea12d50_feat_Introduce_tmux_controller_and_credential_manager_core_components.md
./crates/introspector/vendor/libminizinc/docs/commits/9a63d56f6e1ff35eb9dd93cec914c12c8ac22dd5_refactor_tmux_interface_Document_prelude_and_cfg_refactoring_debug_gemini_eprintln.md
./crates/introspector/vendor/libminizinc/docs/commits/ac86ef2eda7bd9de1a4ed252273b284b8d682d16_docs_Add_Tmux_Workflow_and_Debugging_SOP_Refactor_tmux_controller_and_dump_tmux_status.md
./crates/introspector/vendor/libminizinc/docs/commits/c2b540178526f79e341c1be203bff3e7d4397d82_feat_Implement_--task_argument_for_tmux_controller_create-layout.md
./crates/introspector/vendor/libminizinc/docs/commits/d361e4cb6d58496c067a93ed323203b4455a25dc_docs_Add_CRQ_for_implementing_--task_argument_in_tmux_controller_create-layout.md
./crates/introspector/vendor/libminizinc/docs/commits/eb340ee4905937cc3d3a0e99c8658a40265e3267_docs_Update_GEMINI.md_and_README.md_with_tmux_integration_info.md
./crates/introspector/vendor/libminizinc/docs/git_history_tmux_review.md
./crates/introspector/vendor/libminizinc/docs/onboarding/n00b_tmux_onboarding.md
./crates/introspector/vendor/libminizinc/docs/qa/tmux_controller_cli_qa.md
./crates/introspector/vendor/libminizinc/docs/sops/gemini_tmux_crq_sop.md
./crates/introspector/vendor/libminizinc/docs/sops/tmux_workflow_and_debugging_sop.md
./crates/introspector/vendor/libminizinc/docs/tmux_interface_refactoring.md


## REVIEW AND RENAME:
./crates/introspector/vendor/libminizinc/temp/dump_tmux_status.rs

## CRATES:

crates/introspector/vendor/libminizinc/vendor/crates/tmux_interface
crates/introspector/vendor/libminizinc/vendor/tmux_interface


## SCRIPTS USING TMUX

./crates/introspector/vendor/libminizinc/run_build_in_tmux.sh
./vendor/clangir/clang/utils/analyzer/projects/tmux/cleanup_run_static_analyzer.sh
./vendor/clangir/clang/utils/analyzer/projects/tmux/run_static_analyzer.cmd
./vendor/llvm-project/clang/utils/analyzer/projects/tmux/cleanup_run_static_analyzer.sh
./vendor/llvm-project/clang/utils/analyzer/projects/tmux/run_static_analyzer.cmd

## Packages
./vendor/bootstrapping/guix/gnu/packages/tmux.scm
./vendor/termux/termux-packages/packages/fzf/bin-fzf-tmux.patch
./vendor/termux/termux-packages/packages/tmate/tmux.h.patch
./vendor/termux/termux-packages/packages/tmux/build.sh
./vendor/termux/termux-packages/packages/tmux/compat-imsg.c.patch
./vendor/termux/termux-packages/packages/tmux/compat-setproctitle.c.patch
./vendor/termux/termux-packages/packages/tmux/configure.ac.patch
./vendor/termux/termux-packages/packages/tmux/tmux.conf
./vendor/termux/termux-packages/packages/tmux/tmux.h.patch


5.  **Iterate and Process Files:** For each relative file above
    *   **Construct Absolute Path:** Convert the relative path to an absolute path by prepending `/data/data/com.termux/files/home/storage/github/rustc/`.
    *   **Read and Append Content (Non-Excluded Files):** If the file is not excluded:
        *   Attempt to read its content using the `read_file` tool.
        *   If `read_file` is successful, study it.

for each chunk in the input data
1. extract an emoji vector
1.1. for each key vibe:
1.1.1. assign a unique prime number
1.1.2. assign a unique emoji string
1.1.3. create an model in rust using enum, function, structure
1.1.4. create a clifford multivector with relationships to the other objects.
1.1.4. create a point on a 8d manfold
1.1.4. create a point on the lattice
