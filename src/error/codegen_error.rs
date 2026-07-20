#[derive(Debug)]
pub enum CodeGenError {}

impl CodeGenError {
    pub fn print(&self) {
        match self {
            _ => unreachable!(),
        }
    }
}
