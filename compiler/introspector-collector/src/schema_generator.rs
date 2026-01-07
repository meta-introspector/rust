/// Macro to generate JSON schema from Rust structures
#[macro_export]
macro_rules! generate_schema {
    ($rust_type:ty) => {
        {
            let schema = json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "title": stringify!($rust_type),
                "type": "object",
                "properties": schema_properties!($rust_type),
                "required": required_fields!($rust_type),
                "additionalProperties": false
            });
            schema
        }
    };
}

/// Generate schema properties for any Rust type
#[macro_export]
macro_rules! schema_properties {
    (rustc_hir::Item) => {
        json!({
            "owner_id": { "type": "string", "description": "DefId of the item owner" },
            "ident": { "type": "string", "description": "Item identifier" },
            "kind": { "$ref": "#/definitions/ItemKind" },
            "vis_span": { "$ref": "#/definitions/Span" },
            "span": { "$ref": "#/definitions/Span" },
            "canonical_hash": { "type": "string", "pattern": "^[a-f0-9]{16}$" }
        })
    };
    
    (rustc_hir::ItemKind) => {
        json!({
            "variant": { 
                "type": "string",
                "enum": ["Const", "Static", "Fn", "Macro", "Use", "ExternCrate", 
                        "Mod", "ForeignMod", "GlobalAsm", "TyAlias", "OpaqueTy",
                        "Enum", "Struct", "Union", "Trait", "TraitAlias", "Impl"]
            },
            "data": { "type": "object", "description": "Variant-specific data" }
        })
    };
    
    (rustc_hir::Expr) => {
        json!({
            "hir_id": { "type": "string" },
            "kind": { "$ref": "#/definitions/ExprKind" },
            "span": { "$ref": "#/definitions/Span" }
        })
    };
    
    (rustc_hir::def::DefKind) => {
        json!({
            "variant": {
                "type": "string", 
                "enum": ["Mod", "Struct", "Union", "Enum", "Variant", "Trait",
                        "TyAlias", "ForeignTy", "TraitAlias", "AssocTy", "AssocFn",
                        "AssocConst", "Fn", "Const", "ConstParam", "Static"]
            }
        })
    };
}

/// Generate required fields list
#[macro_export]
macro_rules! required_fields {
    (rustc_hir::Item) => {
        vec!["owner_id", "ident", "kind", "span"]
    };
    
    (rustc_hir::ItemKind) => {
        vec!["variant"]
    };
    
    ($type:ty) => {
        vec![]
    };
}

/// Generate complete schema with all definitions
#[macro_export]
macro_rules! generate_complete_schema {
    () => {
        {
            json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "title": "Rust Compiler Introspection Schema",
                "description": "Auto-generated schema matching Rust's internal structures",
                "type": "object",
                "properties": {
                    "crate_name": { "type": "string" },
                    "items": {
                        "type": "array",
                        "items": { "$ref": "#/definitions/Item" }
                    },
                    "expressions": {
                        "type": "array", 
                        "items": { "$ref": "#/definitions/Expr" }
                    },
                    "def_kinds": {
                        "type": "array",
                        "items": { "$ref": "#/definitions/DefKind" }
                    }
                },
                "definitions": {
                    "Item": generate_schema!(rustc_hir::Item),
                    "ItemKind": generate_schema!(rustc_hir::ItemKind),
                    "Expr": generate_schema!(rustc_hir::Expr),
                    "ExprKind": generate_schema!(rustc_hir::ExprKind),
                    "DefKind": generate_schema!(rustc_hir::def::DefKind),
                    "Span": {
                        "type": "object",
                        "properties": {
                            "lo": { "type": "integer" },
                            "hi": { "type": "integer" }
                        }
                    },
                    "StructuredData": {
                        "type": "object",
                        "properties": {
                            "category": { "type": "string" },
                            "type_name": { "type": "string" },
                            "canonical_form": { "type": "string" },
                            "content_hash": { "type": "string", "pattern": "^[a-f0-9]{16}$" },
                            "fields": {
                                "type": "array",
                                "items": { "$ref": "#/definitions/FieldMetadata" }
                            },
                            "variants": {
                                "type": "array", 
                                "items": { "type": "string" }
                            }
                        },
                        "required": ["category", "type_name", "canonical_form", "content_hash"]
                    },
                    "FieldMetadata": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "type_name": { "type": "string" },
                            "size": { "type": "integer", "minimum": 0 },
                            "alignment": { "type": "integer", "minimum": 1 }
                        },
                        "required": ["name", "type_name", "size", "alignment"]
                    }
                }
            })
        }
    };
}

/// Generate schema and save to file
#[macro_export]
macro_rules! save_schema {
    ($path:expr) => {
        {
            let schema = generate_complete_schema!();
            let schema_json = serde_json::to_string_pretty(&schema).unwrap();
            std::fs::write($path, schema_json).unwrap();
            println!("Generated JSON schema: {}", $path);
        }
    };
}
