use mdl_sys as sys;

use crate::base::Interface;

pub struct SceneElement {
    pub(crate) ptr: sys::ISceneElement,
}

impl Drop for SceneElement {
    fn drop(&mut self) {
        unsafe { sys::IScene_element_release(self.ptr) };
    }
}

impl Clone for SceneElement {
    fn clone(&self) -> SceneElement {
        unsafe {
            sys::IScene_element_retain(self.ptr);
        }
        SceneElement { ptr: self.ptr }
    }
}

impl SceneElement {}

impl Interface for SceneElement {
    fn from_interface_ptr(ptr: sys::IInterface) -> SceneElement {
        SceneElement {
            ptr: ptr as sys::ISceneElement,
        }
    }

    fn to_interface(&self) -> sys::IInterface {
        self.ptr as *mut sys::IInterface_api
    }

    fn type_iid() -> sys::Uuid {
        unsafe { sys::IScene_element_type_get_iid() }
    }
}