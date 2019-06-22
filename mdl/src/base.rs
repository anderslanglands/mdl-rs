use mdl_sys as sys;

pub trait Interface: Sized {
    fn from_interface<I: Interface>(i: I) -> Self {
        let ptr = unsafe {
            sys::IInterface_get_interface(i.to_interface(), Self::type_iid())
        };
        if ptr.is_null() {
            panic!("Tried to convert from null interface");
        }

        // We rlease the original pointer
        // unsafe { sys::IInterface_release(i) };

        Self::from_interface_ptr(ptr)
    }

    fn to_interface(&self) -> sys::IInterface;
    fn from_interface_ptr(ptr: sys::IInterface) -> Self;
    fn type_iid() -> sys::Uuid;
}
