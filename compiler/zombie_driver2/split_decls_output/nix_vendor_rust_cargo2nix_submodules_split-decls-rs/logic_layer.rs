// Split-Decls Layer: Logic
// IO Signature: data → result
// Generated from: ./nix/vendor/rust/cargo2nix/submodules/split-decls-rs

// impl: impl_ProductionEnvironmentOps
impl EnvironmentOps for ProductionEnvironmentOps { fn get_var (& self , key : & str) -> Option < String > { std :: env :: var (key) . ok () } fn set_var (& self , key : & str , value : & str) { std :: env :: set_var (key , value) } fn current_dir (& self) -> IoResult < PathBuf > { std :: env :: current_dir () } fn args (& self) -> Vec < String > { std :: env :: args () . collect () } fn home_dir (& self) -> Option < PathBuf > { std :: env :: var ("HOME") . ok () . map (PathBuf :: from) } }

// impl: impl_ProductionFileSystemOps
impl FileSystemOps for ProductionFileSystemOps { fn read (& self , path : & Path) -> IoResult < Vec < u8 > > { std :: fs :: read (path) } fn write (& self , path : & Path , contents : & [u8]) -> IoResult < () > { std :: fs :: write (path , contents) } fn create_dir_all (& self , path : & Path) -> IoResult < () > { std :: fs :: create_dir_all (path) } fn remove_file (& self , path : & Path) -> IoResult < () > { std :: fs :: remove_file (path) } fn exists (& self , path : & Path) -> bool { path . exists () } fn metadata (& self , path : & Path) -> IoResult < std :: fs :: Metadata > { std :: fs :: metadata (path) } }

// impl: impl_MockEnvironmentOps
impl EnvironmentOps for MockEnvironmentOps { fn get_var (& self , key : & str) -> Option < String > { self . vars . get (key) . cloned () } fn set_var (& self , key : & str , value : & str) { } fn current_dir (& self) -> IoResult < PathBuf > { Ok (PathBuf :: from ("/mock/current/dir")) } fn args (& self) -> Vec < String > { vec ! ["mock_program" . to_string ()] } fn home_dir (& self) -> Option < PathBuf > { Some (PathBuf :: from ("/mock/home")) } }

// impl: impl_FileSystemOracle
impl FileSystemOracle { pub fn new () -> Self { Self } pub fn validate_path (& self , path : & Path) -> IoResult < () > { let path_str = path . to_string_lossy () ; if path_str . contains ("..") { return Err (IoError :: new (std :: io :: ErrorKind :: PermissionDenied , "Path traversal detected")) ; } let forbidden_paths = ["/etc/passwd" , "/etc/shadow" , "/root"] ; for forbidden in & forbidden_paths { if path_str . starts_with (forbidden) { return Err (IoError :: new (std :: io :: ErrorKind :: PermissionDenied , "Access to sensitive path denied")) ; } } Ok (()) } }

// impl: impl_DaoPolicy
impl DaoPolicy { pub fn new () -> Self { Self { allowed_commands : vec ! ["ls" . to_string () , "cat" . to_string () , "echo" . to_string () ,] , approval_threshold : 0.66 , } } pub fn approve_command (& self , cmd : & str) -> bool { if self . allowed_commands . contains (& cmd . to_string ()) { return true ; } let dangerous_commands = ["rm" , "dd" , "format" , "del" , "sudo"] ; for dangerous in & dangerous_commands { if cmd . contains (dangerous) { return false ; } } false } }

// function: main
fn main () { let output = Command :: new ("ls") . arg ("-la") . output () . unwrap () ; println ! ("Files: {}" , String :: from_utf8_lossy (& output . stdout)) ; Command :: new ("echo") . arg ("hello") . status () . unwrap () ; }

// function: main
fn main () -> Result < () > { let project_root = Path :: new ("../../") ; let (deps , patches) = generate_workspace_deps_from_project_root (& project_root) ? ; println ! ("Found {} workspace deps and {} patches" , deps . len () , patches . len ()) ; for (i , dep) in deps . iter () . enumerate () . take (10) { println ! ("Dep {}: {}" , i + 1 , dep) ; } Ok (()) }

// function: generate_workspace_deps_from_project_root
pub fn generate_workspace_deps_from_project_root (project_root : & Path) -> Result < (Vec < String > , Vec < String >) > { let mut workspace_deps = Vec :: new () ; let mut patch_entries = Vec :: new () ; let cargo_tomls = find_cargo_tomls (project_root) ? ; for cargo_path in cargo_tomls { if let Some (crate_info) = extract_crate_info (& cargo_path) ? { let relative_path = cargo_path . parent () . unwrap () . strip_prefix (project_root) ? . to_string_lossy () ; let dep_entry = format ! ("{} = {{ path = \"{}\" }}" , crate_info . name , relative_path) ; workspace_deps . push (dep_entry . clone ()) ; patch_entries . push (dep_entry) ; } } Ok ((workspace_deps , patch_entries)) }

// function: find_cargo_tomls
fn find_cargo_tomls (dir : & Path) -> Result < Vec < PathBuf > > { let mut cargo_tomls = Vec :: new () ; if dir . is_dir () { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . file_name () == Some ("Cargo.toml" . as_ref ()) { cargo_tomls . push (path) ; } else if path . is_dir () && ! should_skip_dir (& path) { cargo_tomls . extend (find_cargo_tomls (& path) ?) ; } } } Ok (cargo_tomls) }

// function: should_skip_dir
fn should_skip_dir (path : & Path) -> bool { let name = path . file_name () . unwrap () . to_string_lossy () ; matches ! (name . as_ref () , "target" | ".git" | "node_modules") }

// function: extract_crate_info
fn extract_crate_info (cargo_path : & Path) -> Result < Option < CrateInfo > > { let content = fs :: read_to_string (cargo_path) ? ; let toml : Value = toml :: from_str (& content) ? ; if let Some (package) = toml . get ("package") { if let Some (name) = package . get ("name") . and_then (| n | n . as_str ()) { return Ok (Some (CrateInfo { name : name . to_string () , })) ; } } Ok (None) }

// function: test_fn
fn test_fn () { }

// function: main
fn main () { println ! ("🧪 Testing macro patch warnings...\n") ; let output = audit_execute ! (Command :: new ("echo") . arg ("Hello World") . output ()) . unwrap () ; println ! ("Result: {}" , String :: from_utf8_lossy (& output . stdout)) ; audit_execute ! (Command :: new ("date") . status ()) . unwrap () ; }

// function: main
fn main () { let output = Command :: new ("ls") . arg ("-la") . output () . expect ("Failed to execute command") ; println ! ("Command output: {}" , String :: from_utf8_lossy (& output . stdout)) ; let result = std :: process :: Command :: new ("echo") . arg ("hello") . execute () ; }

// function: process_function
fn process_function (input : TokenStream) -> TokenStream { let parsed : syn :: ItemFn = syn :: parse2 (input) . unwrap () ; let fn_name = & parsed . sig . ident ; let inputs = & parsed . sig . inputs ; let output = & parsed . sig . output ; let block = & parsed . block ; let mut visitor = FunctionVisitor :: new () ; visitor . visit_item_fn (& parsed) ; quote ! { pub fn # fn_name (# inputs) # output { println ! ("Wrapped function: {}" , stringify ! (# fn_name)) ; # block } } }

// impl: impl_FunctionVisitor
impl FunctionVisitor { fn new () -> Self { Self { call_count : 0 } } }

// impl: impl_FunctionVisitor
impl < 'ast > Visit < 'ast > for FunctionVisitor { fn visit_expr_call (& mut self , node : & 'ast ExprCall) { self . call_count += 1 ; syn :: visit :: visit_expr_call (self , node) ; } }

// function: transform_struct
fn transform_struct (input : TokenStream) -> TokenStream { let mut parsed : syn :: ItemStruct = syn :: parse2 (input) . unwrap () ; if let syn :: Fields :: Named (ref mut fields) = parsed . fields { for field in & mut fields . named { field . attrs . push (parse_quote ! (# [debug])) ; } } quote ! (# parsed) }

// function: main
fn main () { println ! ("cargo:rerun-if-changed=recursive_dependencies.json") ; println ! ("cargo:rerun-if-changed=import_macros.rs") ; println ! ("cargo:rerun-if-changed=name_index.json") ; let json_content = fs :: read_to_string ("recursive_dependencies.json") . unwrap () ; let deps : Value = serde_json :: from_str (& json_content) . unwrap () ; let timestamp = SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) . unwrap () . as_secs () ; let mut macro_content = format ! ("// GENERATED FILE - DO NOT EDIT MANUALLY\n\
         //\n\
         // Generator: build.rs (split-decls-rs)\n\
         // Generated: Unix timestamp {}\n\
         // Source files read:\n\
         //   - recursive_dependencies.json ({} resolved terms)\n\
         //   - import_macros.rs (macro definitions)\n\
         //   - name_index.json (if exists)\n\
         //\n\
         // This file is regenerated when:\n\
         //   - recursive_dependencies.json changes\n\
         //   - import_macros.rs changes  \n\
         //   - name_index.json changes\n\
         //   - build.rs is modified\n\
         //\n\
         // To manually regenerate: cargo build\n\
         \n" , timestamp , deps ["resolved_terms"] . as_array () . map_or (0 , | arr | arr . len ())) ; macro_content . push_str ("include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/import_macros.rs\"));\n\nmacro_rules! mkbin {\n    () => {\n") ; if let Some (resolved) = deps ["resolved_terms"] . as_array () { macro_content . push_str (& format ! ("        // Processing {} resolved dependencies\n" , resolved . len ())) ; macro_content . push_str ("        // Note: import_* macros are currently disabled\n") ; macro_content . push_str ("        // Enable by regenerating import_macros.rs with proper content\n") ; for term in resolved . iter () . take (10) { if let Some (name) = term . as_str () { macro_content . push_str (& format ! ("        // mod {} {{\n" , name)) ; macro_content . push_str (& format ! ("        //     println!(\"Module {} loaded\");\n" , name)) ; macro_content . push_str (& format ! ("        //     import_{}!();\n" , name)) ; macro_content . push_str ("        // }\n") ; } } } macro_content . push_str ("        println!(\"mkbin executed with modules\");\n") ; macro_content . push_str ("    };\n}\n") ; fs :: write ("src/generated_mkbin.rs" , macro_content) . unwrap () ; }

