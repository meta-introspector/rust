// Split-Decls Layer: Data
// IO Signature: () → struct
// Generated from: ./nix/vendor/rust/cargo2nix/submodules/split-decls-rs

// struct: ProductionEnvironmentOps
# [derive (Debug , Default)] pub struct ProductionEnvironmentOps ;

// struct: ProductionFileSystemOps
# [derive (Debug , Default)] pub struct ProductionFileSystemOps ;

// struct: MockEnvironmentOps
# [derive (Debug , Default)] pub struct MockEnvironmentOps { pub vars : std :: collections :: HashMap < String , String > , }

// struct: FileSystemOracle
# [derive (Debug)] pub struct FileSystemOracle ;

// struct: DaoPolicy
# [derive (Debug)] pub struct DaoPolicy { pub allowed_commands : Vec < String > , pub approval_threshold : f64 , }

// struct: CrateInfo
struct CrateInfo { name : String , }

// struct: TestStruct
struct TestStruct ;

// struct: FunctionVisitor
struct FunctionVisitor { call_count : usize , }

