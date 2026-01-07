/// Generate enum-to-string macros from rustc source
use introspector_collector::enum_string_generator::EnumStringGenerator;
use std::fs;

fn main() {
    println!("🔧 Generating enum-to-string macros...");
    
    let mut generator = EnumStringGenerator::new();
    
    // Example: Extract from rustc_hir DefKind enum
    let defkind_code = r#"
        pub enum DefKind {
            Fn,
            AssocFn,
            Struct,
            Variant,
            Enum,
            Const,
            Static,
            Trait,
            Impl,
            Mod,
            Use,
            Macro(MacroKind),
            Ctor(CtorOf, CtorKind),
        }
    "#;
    
    if let Ok(file) = syn::parse_str::<syn::File>(defkind_code) {
        generator.extract_enums(&file);
        generator.find_enum_string_functions();
        
        let macros = generator.generate_all_macros();
        
        // Write to generated file
        let output = format!("// Auto-generated enum-to-string macros\n{}", macros);
        fs::write("src/generated/enum_string_macros.rs", output)
            .expect("Failed to write macro file");
        
        println!("✅ Generated macros for {} enums", generator.enums.len());
        
        // Show example usage
        for enum_name in generator.enums.keys() {
            let macro_name = format!("mk_enum_to_string_{}", enum_name.to_lowercase());
            println!("📝 Use: {}!(value) -> &str", macro_name);
        }
    }
}
