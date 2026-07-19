#[derive(Debug)]
pub enum TypeError {}

impl TypeError {
    pub fn print(&self) {
        match self {
            _ => unreachable!(),
        }
    }
}
