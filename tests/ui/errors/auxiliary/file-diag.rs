//@ compile-flags: --remap-path-prefix={{src-base}}=remapped
//@ compile-flags: -Zremap-path-scope=diagnostics

#[macro_export]
macro_rules! my_file {
    () => { file!() }
}

pub fn file() -> &'static str {
    file!()
}
