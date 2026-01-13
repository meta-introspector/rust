// Auto-generated Rust language construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustLanguageComponents {
    // Function Complexity Levels
    ComplexFunctions = 6,
    TrivialFunctions = 41,
    ModerateFunctions = 7,
    SimpleFunctions = 4,
    // Basic Block Types
    BranchBlocks = 10,
    SequentialBlocks = 39,
    ReturnBlocks = 9,
    CallBlocks = 1,
    // I/O and Purity
    IoFunctions = 19194,
    SyscallPatterns = 20071,
    PureBlocks = 58,
    ImpureBlocks = 1,
}

impl RustLanguageComponents {
    pub fn count(&self) -> usize { *self as usize }
    
    pub fn component_type(&self) -> &'static str {
        match self {
            RustLanguageComponents::TrivialFunctions | 
            RustLanguageComponents::SimpleFunctions | 
            RustLanguageComponents::ModerateFunctions | 
            RustLanguageComponents::ComplexFunctions | 
            RustLanguageComponents::CriticalFunctions => "Function",
            _ => "Block",
        }
    }
}
