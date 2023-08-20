use crate::models::TableStruct;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelMessage {
    OpenFile(String),
    ReadFile(TableStruct),
    ReadFileError(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    Light,
    Dark,
    Frappe,
    Latte,
    Macchiato,
    Mocha,
}
