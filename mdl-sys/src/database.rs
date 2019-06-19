use crate::components::*;
use crate::scope::IScope;
use crate::base::Uuid;

extern "C" {
    pub fn IDatabase_release(db: IDatabase);
    pub fn IDatabase_retain(db: IDatabase);
    pub fn IDatabase_get_global_scope(db: IDatabase) -> IScope;
    pub fn IDatabase_type_get_iid() -> Uuid;
}
