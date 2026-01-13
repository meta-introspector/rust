// Split-Decls Layer: Data
// IO Signature: () → struct
// Generated from: .

// enum: TestType
# [derive (Debug , Clone)] enum TestType { OpcodeDecoding (u8 , String) , PatternHunting (String) , DecoderVerification (String) , SelfReferenceCheck (String) , }

// struct: TestFeature
# [derive (Debug , Clone)] struct TestFeature { name : String , test_type : TestType , auto_generated : bool , }

// struct: AutoExperiment
struct AutoExperiment { features : Vec < TestFeature > , results : HashMap < String , bool > , }

// struct: ExperimentResults
# [derive (Debug)] struct ExperimentResults { total_features : usize , passed : usize , failed : usize , results : HashMap < String , bool > , }

// enum: NumberNameCodecs
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum NumberNameCodecs { Codec_0 = 7 , Codec_1 = 5 , Codec_2 = 5 , Codec_3 = 5 , Codec_4 = 5 , Codec_5 = 5 , Codec_6 = 7 , Codec_7 = 5 , Codec_8 = 5 , Codec_9 = 5 , Codec_10 = 5 , }

// struct: RustElement
# [derive (Debug , Clone)] struct RustElement { atomic_number : usize , symbol : String , name : String , element_type : ElementType , modular_key : String , weight : u32 , level : u32 , period : usize , group : usize , hotness : f64 , }

// enum: ElementType
# [derive (Debug , Clone)] enum ElementType { Function , Enum , Struct , Trait , }

// struct: LMFDBIndex
# [derive (Debug , Clone)] struct LMFDBIndex { label : String , weight : u32 , level : u32 , character : String , dimension : u32 , }

// struct: SymbolLMFDBMapping
# [derive (Debug)] struct SymbolLMFDBMapping { symbol_name : String , address : u64 , lmfdb_index : Option < LMFDBIndex > , modular_signature : u64 , }

// struct: UserRegs
# [repr (C)] struct UserRegs { r15 : u64 , r14 : u64 , r13 : u64 , r12 : u64 , rbp : u64 , rbx : u64 , r11 : u64 , r10 : u64 , r9 : u64 , r8 : u64 , rax : u64 , rcx : u64 , rdx : u64 , rsi : u64 , rdi : u64 , orig_rax : u64 , rip : u64 , cs : u64 , eflags : u64 , rsp : u64 , ss : u64 , }

// enum: RustLanguageComponents
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum RustLanguageComponents { ComplexFunctions = 6 , TrivialFunctions = 41 , ModerateFunctions = 7 , SimpleFunctions = 4 , BranchBlocks = 10 , SequentialBlocks = 39 , ReturnBlocks = 9 , CallBlocks = 1 , IoFunctions = 19194 , SyscallPatterns = 20071 , PureBlocks = 58 , ImpureBlocks = 1 , }

// struct: SymbolMonsterData
# [derive (Debug)] struct SymbolMonsterData { name : String , address : u64 , size : u64 , bytes : Vec < u8 > , monster_signature : Vec < u64 > , morse_critical_points : Vec < f64 > , symmetry_group : u64 , }

// struct: CallGraph
# [derive (Debug)] struct CallGraph { edges : Vec < (String , String) > , monster_flows : HashMap < u64 , Vec < (String , String) > > , morse_landscape : Vec < (f64 , f64) > , }

// struct: EnumModularForm
# [derive (Debug , Clone)] struct EnumModularForm { enum_name : String , modular_key : String , level : u32 , weight : u32 , nesting_depth : usize , usage_count : usize , module_hotness : f64 , variant_signatures : Vec < String > , }

// struct: FunctionTrace
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct FunctionTrace { pub name : String , pub timestamp : u64 , pub registers : RegisterState , pub memory_accesses : Vec < MemoryAccess > , pub call_depth : u32 , pub return_value : Option < u64 > , }

// struct: RegisterState
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct RegisterState { pub rdi : u64 , pub rsi : u64 , pub rdx : u64 , pub rcx : u64 , pub r8 : u64 , pub r9 : u64 , pub rax : u64 , pub rsp : u64 , }

// struct: MemoryAccess
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct MemoryAccess { pub address : u64 , pub size : usize , pub access_type : AccessType , pub data_preview : Vec < u8 > , }

// enum: AccessType
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum AccessType { Read , Write , Execute , }

// struct: FunctionSignature
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct FunctionSignature { pub name : String , pub args : Vec < ArgumentType > , pub return_type : ReturnType , pub calling_convention : CallingConvention , pub confidence : f64 , }

// enum: ArgumentType
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum ArgumentType { Pointer { target_type : Box < ArgumentType > } , Integer { size : usize , signed : bool } , Float { size : usize } , Struct { size : usize , fields : Vec < ArgumentType > } , String { encoding : StringEncoding } , Unknown { size : usize } , }

// enum: StringEncoding
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum StringEncoding { Utf8 , Ascii , CString , }

// enum: ReturnType
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum ReturnType { Void , Value (ArgumentType) , Pointer (Box < ArgumentType >) , }

// enum: CallingConvention
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum CallingConvention { SystemV , Windows , Rust , C , }

// struct: ABIExtractor
pub struct ABIExtractor { traces : Vec < FunctionTrace > , memory_snapshots : BTreeMap < u64 , Vec < u8 > > , function_patterns : HashMap < String , Vec < FunctionTrace > > , }

// struct: MonsterTopologyNode
# [derive (Debug , Serialize , Deserialize)] struct MonsterTopologyNode { coordinates : (f64 , f64 , f64) , symbol_name : String , lattice_label : String , ngrams : Vec < NgramCoordinate > , syn_ast_data : Option < SynAstData > , monster_signature : Vec < u64 > , phi_score : f64 , topology_connections : Vec < String > , }

// struct: NgramCoordinate
# [derive (Debug , Serialize , Deserialize)] struct NgramCoordinate { ngram : String , ngram_coordinates : (f64 , f64 , f64) , monster_correlation : f64 , frequency : u32 , source : String , }

// struct: SynAstData
# [derive (Debug , Serialize , Deserialize)] struct SynAstData { ast_type : String , ast_structure : Value , syn_ngrams : Vec < String > , ast_monster_correlation : f64 , }

// struct: MonsterTopology
# [derive (Debug , Serialize , Deserialize)] struct MonsterTopology { nodes : Vec < MonsterTopologyNode > , ngram_coordinate_map : HashMap < String , Vec < (f64 , f64 , f64) > > , topology_metrics : TopologyMetrics , }

// struct: TopologyMetrics
# [derive (Debug , Serialize , Deserialize)] struct TopologyMetrics { total_nodes : usize , total_ngrams : usize , syn_coverage : f64 , monster_density : f64 , topology_completeness : f64 , }

// enum: P2PVerb
# [derive (Debug , Clone , Serialize , Deserialize)] pub enum P2PVerb { LoadSo (String , String) , RegisterEvent (String , u32) , AttachData (String , Vec < u8 >) , RunWithFiles (String , Vec < String >) , CaptureResult (String) , CompileSource (String , String) , CompileFile (String , String) , InvokeFunction (String , String , u32) , StartTrace (String , String) , StopTrace (String) , PerfRecord (String , String) , StraceRecord (String , String) , CompareTraces (String , String) , Intercept (String , String) , Extract (String , String) , Exfiltrate (String , String) , Munge (String , String) , CompileAndLoad (String , String) , LoadMultipleSo (Vec < (String , String) >) , UnloadSo (String) , ListLoadedSo , CallSoFunction (String , String , Vec < String >) , GetSoSymbols (String) , ReloadSo (String) , CompileToBinary (String , String , String) , RegisterPeer (PeerInfo) , SeedDataset (DatasetSeed) , QueryCapabilities (String) , RequestAnalysis (String , String) , ShareResults (String , Vec < u8 >) , SyncGitRepo (String , String) , PublishToHF (String , String) , CompileSelf , RebootSelf , UpdateSelf (String) , GetSelfStatus , LoadCargo (String) , CallCargoMain (Vec < String >) , CargoBuild (String , Vec < String >) , CargoRun (String , Vec < String >) , CargoTest (String , Vec < String >) , LoadRustcDriver (String) , CallRustcMain (Vec < String >) , CompileViaRustc (String , String , Vec < String >) , GetRustcVersion , CalculateLattice (String , Vec < u64 >) , CompareEnergy (String , String) , ClusterFunctions (Vec < String >) , GenerateParquet (String , String) , }

// struct: PeerInfo
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct PeerInfo { pub peer_id : String , pub git_endpoint : String , pub huggingface_repo : String , pub nix_store_path : String , pub mathematical_capabilities : Vec < String > , pub dataset_contributions : Vec < String > , pub last_seen : String , pub lattice_support : bool , pub parquet_generation : bool , }

// struct: DatasetSeed
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct DatasetSeed { pub dataset_name : String , pub version : String , pub mathematical_framework : String , pub peer_seeds : Vec < PeerInfo > , pub git_refs : Vec < String > , pub hf_commits : Vec < String > , pub lattice_dimensions : u32 , pub function_count : u64 , pub total_energy : u64 , }

// struct: LoadedSo
# [derive (Debug)] pub struct LoadedSo { pub name : String , pub path : String , pub library : Library , pub symbols : Vec < String > , pub load_time : String , pub size : u64 , }

// struct: CompilationResult
# [derive (Debug , Clone)] pub struct CompilationResult { pub name : String , pub source_path : String , pub output_path : String , pub compilation_time : f64 , pub success : bool , pub errors : Vec < String > , pub warnings : Vec < String > , }

// struct: FunctionAnalysis
# [derive (Debug , Clone)] pub struct FunctionAnalysis { pub name : String , pub coordinates : Vec < u64 > , pub energy : u64 , pub classification : String , pub size : u64 , }

// struct: UnifiedP2PServer
pub struct UnifiedP2PServer { plugin_driver : PluginDriver , event_registry : HashMap < String , Vec < u32 > > , stored_data : HashMap < String , Vec < u8 > > , results : HashMap < String , Vec < u8 > > , loaded_sos : HashMap < String , LoadedSo > , compilation_results : HashMap < String , CompilationResult > , temp_dir : String , rustc_driver_loaded : bool , rustc_driver_path : Option < String > , cargo_loaded : bool , cargo_path : Option < String > , peers : HashMap < String , PeerInfo > , datasets : HashMap < String , DatasetSeed > , function_analyses : HashMap < String , FunctionAnalysis > , lattice_cache : HashMap < String , Vec < u64 > > , git_manager : GitManager , hf_manager : HuggingFaceManager , nix_manager : NixManager , }

// struct: PluginDriver
struct PluginDriver ;

// struct: GitManager
struct GitManager ;

// struct: HuggingFaceManager
struct HuggingFaceManager ;

// struct: NixManager
struct NixManager ;

// struct: EnhancedTracer
pub struct EnhancedTracer { output_dir : String , }

// struct: Symbol
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct Symbol { pub name : String , pub address : u64 , pub size : u64 , pub demangled : String , }

// struct: FunctionStats
# [derive (Debug , Serialize , Deserialize)] pub struct FunctionStats { pub count : u64 , pub samples : u64 , pub total_time : f64 , }

// struct: PerfRecorder
pub struct PerfRecorder { rustc_path : PathBuf , output_dir : PathBuf , }

// struct: PerfAnalyzer
pub struct PerfAnalyzer { functions : HashMap < String , FunctionStats > , }

