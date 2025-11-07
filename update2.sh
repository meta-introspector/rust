#`cat cargos.txt`;
for x in   "compiler/rustc" "src/build_helper" "src/etc/test-float-parse" "src/rustc-std-workspace/rustc-std-workspace-core" "src/rustc-std-workspace/rustc-std-workspace-alloc" "src/rustc-std-workspace/rustc-std-workspace-std" "src/rustdoc-json-types" "src/tools/cargotest" "src/tools/clippy" "src/tools/clippy/clippy_dev" "src/tools/compiletest" "src/tools/run-make-support" "src/tools/error_index_generator" "src/tools/linkchecker" "src/tools/lint-docs" "src/tools/miropt-test-tools" "src/tools/unstable-book-gen" "src/tools/tidy" "src/tools/tier-check" "src/tools/build-manifest" "src/tools/remote-test-client" "src/tools/remote-test-server" "src/tools/rust-installer" "src/tools/rustdoc" "src/tools/rls" "src/tools/rustfmt" "src/tools/miri" "src/tools/miri/cargo-miri" "src/tools/rustdoc-themes" "src/tools/unicode-table-generator" "src/tools/jsondocck" "src/tools/jsondoclint" "src/tools/llvm-bitcode-linker" "src/tools/html-checker" "src/tools/bump-stage0" "src/tools/replace-version-placeholder" "src/tools/lld-wrapper" "src/tools/collect-license-metadata" "src/tools/generate-copyright" "src/tools/suggest-tests" "src/tools/generate-windows-sys" "src/tools/rustdoc-gui-test" "src/tools/opt-dist" "src/tools/coverage-dump" "src/tools/rustc-perf-wrapper" "src/tools/wasm-component-ld"; 
do echo $x;
   pushd  `dirname $x`;
   if ls Cargo.lock ;
   then
       if ! ls Cargo.nix ;
       then
	   /data/data/com.termux.nix/files/home/nix/vendor/nix/cargo2nix/target/debug/cargo2nix ;
       else
	   git add Cargo.nix
	   #git commit -m 'nix'
	   #git checkout -b feature/CRQ-016-nixify
	   #git status
	   
       fi
   fi
   popd
       
done
