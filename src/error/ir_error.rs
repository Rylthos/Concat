#[derive(Debug)]
pub enum IRError {}

impl IRError {
    pub fn print(&self) {
        match self {
            _ => unreachable!(),
        }
    }
}
