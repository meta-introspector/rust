// The 15 Gogobglabulab Primes
pub const GOGOBGLABULAB_PRIMES: [(u64, u64, &str); 15] = [
    (2, 0x423fb00, "8.4.12.h"),
    (3, 0x425d6f0, "34.2.12.h"),
    (5, 0x4261a40, "37.2.11.k"),
    (7, 0x42317b0, "27.6.11.a"),
    (11, 0xbfa01a0, "24.6.12.x"),
    (13, 0xbf9fd30, "6.6.12.f"),
    (17, 0xbfa01c0, "2.4.12.b"),
    (19, 0x42410c0, "21.6.11.u"),
    (23, 0xbfa0280, "16.2.12.p"),
    (29, 0xbfa0290, "30.6.12.d"),
    (31, 0xbfa01b0, "1.2.11.a"),
    (37, 0xbfa02a0, "35.4.11.i"),
    (41, 0xbfa0330, "26.4.12.z"),
    (43, 0xe1b4600, "26.4.12.z"),
    (47, 0x42412f0, "31.2.11.e"),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Gogobglabulab primes initialized");
    Ok(())
}
