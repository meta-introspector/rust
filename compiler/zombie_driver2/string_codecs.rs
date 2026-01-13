// Auto-generated string codecs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCodecs {
    Codec_0 = 0x426fe80, // EnumToString with 10 strings
    Codec_1 = 0x4319f00, // EnumToString with 10 strings
}

impl StringCodecs {
    pub fn address(&self) -> u64 { *self as u64 }
    
    pub fn decode_string(&self, input: &str) -> Option<u32> {
        match self {
            StringCodecs::Codec_0 => {
                if input == "branch_case_418" { return Some(1); }
                if input == "branch_case_362" { return Some(0); }
                if input == "branch_case_449" { return Some(2); }
                if input == "branch_case_553" { return Some(3); }
                None
            },
            StringCodecs::Codec_1 => {
                if input == "cmp_case_248" { return Some(0); }
                if input == "cmp_case_248" { return Some(2); }
                if input == "cmp_case_192" { return Some(1); }
                None
            },
        }
    }
    
    pub fn encode_enum(&self, value: u32) -> Option<&'static str> {
        match self {
            StringCodecs::Codec_0 => {
                match value {
                    1 => Some("branch_case_418"),
                    0 => Some("branch_case_362"),
                    2 => Some("branch_case_449"),
                    3 => Some("branch_case_553"),
                    _ => None,
                }
            },
            StringCodecs::Codec_1 => {
                match value {
                    0 => Some("cmp_case_248"),
                    2 => Some("cmp_case_248"),
                    1 => Some("cmp_case_192"),
                    _ => None,
                }
            },
        }
    }
}
