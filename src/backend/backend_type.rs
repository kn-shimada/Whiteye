use strum::EnumString;

pub const BACKEND_TYPES: [&str; 2] = ["none", "llvm"];

#[derive(Debug, PartialEq, EnumString)]
pub enum BackendType {
    #[strum(ascii_case_insensitive)]
    LLVM,
    #[strum(ascii_case_insensitive)]
    None,
}
