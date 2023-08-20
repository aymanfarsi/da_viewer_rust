#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableStruct {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
