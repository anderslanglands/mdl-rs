use crate::components::*;
use crate::scope::IScope;

extern "C" {
    pub fn IDatabase_release(db: IDatabase);
    pub fn IDatabase_get_global_scope(db: IDatabase) -> IScope;
}
