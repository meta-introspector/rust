// Split-Decls Layer: Interface
// IO Signature: trait → impl
// Generated from: ./nix/vendor/rust/cargo2nix/submodules/split-decls-rs

// trait: EnvironmentOps
# [doc = " Environment operations trait - 2,470 calls (74.2% of all syscalls)"] # [doc = " Strategy: Generic bounds for performance due to high usage"] pub trait EnvironmentOps : Send + Sync { fn get_var (& self , key : & str) -> Option < String > ; fn set_var (& self , key : & str , value : & str) ; fn current_dir (& self) -> IoResult < PathBuf > ; fn args (& self) -> Vec < String > ; fn home_dir (& self) -> Option < PathBuf > ; }

// trait: IOOps
# [doc = " IO operations trait - 355 calls (10.6% of all syscalls)  "] # [doc = " Strategy: Dependency injection for testing flexibility"] pub trait IOOps : Send + Sync { fn read_to_string (& self , path : & Path) -> IoResult < String > ; fn write_all (& self , path : & Path , contents : & [u8]) -> IoResult < () > ; fn stdin_read_line (& self , buf : & mut String) -> IoResult < usize > ; fn stdout_write (& self , buf : & [u8]) -> IoResult < () > ; fn stderr_write (& self , buf : & [u8]) -> IoResult < () > ; }

// trait: FileSystemOps
# [doc = " Filesystem operations trait - 211 calls (6.3% of all syscalls)"] # [doc = " Strategy: Oracle validation for security (path traversal prevention)"] pub trait FileSystemOps : Send + Sync { fn read (& self , path : & Path) -> IoResult < Vec < u8 > > ; fn write (& self , path : & Path , contents : & [u8]) -> IoResult < () > ; fn create_dir_all (& self , path : & Path) -> IoResult < () > ; fn remove_file (& self , path : & Path) -> IoResult < () > ; fn exists (& self , path : & Path) -> bool ; fn metadata (& self , path : & Path) -> IoResult < std :: fs :: Metadata > ; }

// trait: ProcessOps
# [doc = " Process operations trait - 66 calls (1.9% of all syscalls)"] # [doc = " Strategy: DAO governance required due to security implications"] pub trait ProcessOps : Send + Sync { fn execute (& self , cmd : & str , args : & [& str]) -> IoResult < ProcessOutput > ; fn spawn (& self , cmd : & str) -> IoResult < std :: process :: Child > ; fn current_exe (& self) -> IoResult < PathBuf > ; fn exit (& self , code : i32) -> ! ; }

// trait: NetworkOps
# [doc = " Network operations trait - 10 calls (0.3% of all syscalls)"] # [doc = " Strategy: Validation required for security"] pub trait NetworkOps : Send + Sync { fn tcp_connect (& self , addr : SocketAddr) -> IoResult < TcpStream > ; fn tcp_bind (& self , addr : SocketAddr) -> IoResult < std :: net :: TcpListener > ; }

// trait: MyTrait
trait MyTrait { fn method (& self) -> String ; }

