use crate::ast::typed_node::TypedRegion;

use super::type_checker::TypeChecker;

impl TypeChecker {
    pub(crate) fn print(&self, region: &TypedRegion) {
        println!("{:?}", region);
    }
}
