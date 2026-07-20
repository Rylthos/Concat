use crate::ast::reduced_node::ReducedRegion;

use super::reducer::Reducer;

impl Reducer {
    pub(crate) fn print(&self, region: &ReducedRegion) {
        println!("{:?}", region);
    }
}
