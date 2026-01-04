Perf Trace Analysis: Functions Only in Constants Compilation

High Impact (+2%+):
+11.77% __strcmp_avx2 - String comparison (symbol resolution)
+3.27% __GI___tunables_init - Dynamic linker tuning
+3.23% bash.flatten.constprop.0 - Shell process optimization
+3.08% rustc_type_ir.flags.add_kind - Type flag computation
+3.07% rustc_query_impl.query_callbacks - Query system callbacks
+2.92% llvm::InitTargetOptionsFromCodeGenFlags - LLVM target setup
+2.90% llvm::initializeLoongArchPreRAExpandPseudoPass - LLVM pass init
+2.84% llvm::StringMapImpl::LookupBucketFor - LLVM string lookup
+2.77% malloc - Memory allocation
+2.69% rustc_codegen_llvm.configure_llvm - LLVM configuration

Low Impact (+0.8-2.5%):
+2.65% read_conf_file.isra.0 - Configuration file reading
+2.49% add_module - Module loading
+2.30% __mmap - Memory mapping
+1.50% __memmove_avx_unaligned_erms - Memory operations
+0.82% get_common_cache_info.constprop.0 - Cache info

Key Insight: Constants trigger LLVM initialization, type system activation, and query infrastructure - indicating the compiler switches from minimal parsing mode to full semantic analysis mode.
