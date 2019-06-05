use mdl_sys as sys;

pub trait Interface {
    fn from_interface(ptr: sys::IInterface) -> Self;
    fn to_interface(&self) -> sys::IInterface;
    fn type_iid() -> sys::Uuid;
}
