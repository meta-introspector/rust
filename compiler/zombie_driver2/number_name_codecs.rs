// Auto-generated number-to-name codecs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberNameCodecs {
    Codec_0 = 7, // Modulo with 10 mappings
    Codec_1 = 5, // Modulo with 10 mappings
    Codec_2 = 5, // Modulo with 10 mappings
    Codec_3 = 5, // Modulo with 10 mappings
    Codec_4 = 5, // Modulo with 10 mappings
    Codec_5 = 5, // Modulo with 10 mappings
    Codec_6 = 7, // Modulo with 10 mappings
    Codec_7 = 5, // Modulo with 10 mappings
    Codec_8 = 5, // Modulo with 10 mappings
    Codec_9 = 5, // Modulo with 10 mappings
    Codec_10 = 5, // Modulo with 10 mappings
}

impl NumberNameCodecs {
    pub fn decode_number(&self, input: u32) -> Option<&'static str> {
        match self {
            NumberNameCodecs::Codec_0 => {
                let index = (input % 7) as usize;
                match index {
                    0 => Some("$P"),
                    1 => Some("|$pH"),
                    2 => Some("t$`H"),
                    3 => Some("L$PH"),
                    4 => Some("|$hH"),
                    5 => Some("$X"),
                    6 => Some("$G"),
                    7 => Some("$H"),
                    8 => Some("T$XH"),
                    9 => Some("D$hH"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_1 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("$P"),
                    1 => Some("|$ H"),
                    2 => Some("$H"),
                    3 => Some("D$(H"),
                    4 => Some("|$("),
                    5 => Some("t$ "),
                    6 => Some("$X"),
                    7 => Some("|$ "),
                    8 => Some("D$7"),
                    9 => Some("$H"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_2 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("$P"),
                    1 => Some("$`"),
                    2 => Some("$0"),
                    3 => Some("$("),
                    4 => Some("$?"),
                    5 => Some("$@"),
                    6 => Some("$H"),
                    7 => Some("$@"),
                    8 => Some("$o"),
                    9 => Some("$H"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_3 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("$ "),
                    1 => Some("u,"),
                    2 => Some("$0"),
                    3 => Some("$("),
                    4 => Some("$0"),
                    5 => Some("$("),
                    6 => Some("$("),
                    7 => Some(",H"),
                    8 => Some(")H"),
                    9 => Some("$("),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_4 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("4H"),
                    1 => Some("H9"),
                    2 => Some("$ "),
                    3 => Some("$ "),
                    4 => Some("$("),
                    5 => Some("tKH"),
                    6 => Some("$0"),
                    7 => Some("$@"),
                    8 => Some("$H"),
                    9 => Some("$8"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_5 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("T$."),
                    1 => Some("T$"),
                    2 => Some("$("),
                    3 => Some("D$ "),
                    4 => Some(")H"),
                    5 => Some("D$-@"),
                    6 => Some("$ "),
                    7 => Some("|$0"),
                    8 => Some("L$/$"),
                    9 => Some("|$0"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_6 => {
                let index = (input % 7) as usize;
                match index {
                    0 => Some("$p"),
                    1 => Some("|$hH"),
                    2 => Some("|$xH"),
                    3 => Some("$h"),
                    4 => Some(",H"),
                    5 => Some("t$pH"),
                    6 => Some("|$x"),
                    7 => Some("T$XH"),
                    8 => Some("$x"),
                    9 => Some("D$`"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_7 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("$g"),
                    1 => Some("$h"),
                    2 => Some("$g"),
                    3 => Some("$p"),
                    4 => Some("$x"),
                    5 => Some(",-t"),
                    6 => Some("$v"),
                    7 => Some("58"),
                    8 => Some("4H"),
                    9 => Some(",+t#"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_8 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("|$"),
                    1 => Some("|$`H"),
                    2 => Some("|$"),
                    3 => Some("|$"),
                    4 => Some("|$@H"),
                    5 => Some("t$`"),
                    6 => Some("t$"),
                    7 => Some("D$@"),
                    8 => Some("T$ H"),
                    9 => Some("<K"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_9 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("L$XH"),
                    1 => Some("D$hH"),
                    2 => Some("L$`H"),
                    3 => Some("L$@H"),
                    4 => Some("D$HH"),
                    5 => Some("T$PH"),
                    6 => Some("T$h"),
                    7 => Some("t$@H"),
                    8 => Some("|$PH"),
                    9 => Some("T$pH"),
                    _ => None,
                }
            },
            NumberNameCodecs::Codec_10 => {
                let index = (input % 5) as usize;
                match index {
                    0 => Some("t$X"),
                    1 => Some("|$X"),
                    2 => Some("|$H"),
                    3 => Some("|$X"),
                    4 => Some("|$@H"),
                    5 => Some(")H"),
                    6 => Some("|$PH"),
                    7 => Some("t$HH"),
                    8 => Some("|$8H"),
                    9 => Some("D$P"),
                    _ => None,
                }
            },
        }
    }
}
