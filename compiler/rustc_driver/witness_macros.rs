#[macro_export]
macro_rules! witness {
    (symbol: $sym:expr, from: $from:expr) => {
        println!("WITNESS_SYMBOL: {{\
            \"symbol\":\"{}\",\
            \"from\":\"{}\",\
            \"file\":\"{}\",\
            \"line\":{}\
        }}", $sym, $from, file!(), line!());
    };
    
    (resolve: $from:expr => $to:expr) => {
        println!("WITNESS_RESOLVE: {{\
            \"from\":\"{}\",\
            \"to\":\"{}\",\
            \"file\":\"{}\",\
            \"line\":{}\
        }}", $from, $to, file!(), line!());
    };
    
    (crate: $name:expr, version: $ver:expr) => {
        println!("WITNESS_CRATE: {{\
            \"name\":\"{}\",\
            \"version\":\"{}\",\
            \"file\":\"{}\",\
            \"line\":{}\
        }}", $name, $ver, file!(), line!());
    };
}
