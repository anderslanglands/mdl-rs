pub mod neuray;
pub use neuray::*;
pub mod components;
pub use components::*;
pub mod results;
pub use results::*;
pub mod mdl_compiler;
pub use mdl_compiler::*;
pub mod database;
pub use database::*;
pub mod mdl_factory;
pub use mdl_factory::*;
pub mod mdl_execution_context;
pub use mdl_execution_context::*;
pub mod image_api;
pub use image_api::*;
pub mod scope;
pub use scope::*;
pub mod base;
pub use base::*;
pub mod version;
pub use version::*;

#[test]
fn test_load_ineuray() {
    use std::ffi::{CStr, CString};
    unsafe {
        let n = load_ineuray();
        assert!(!n.is_null(), "Could not load ineuray interface");

        println!(
            "ineuray interface version: {}",
            ineuray_get_interface_version(n),
        );

        let version = CStr::from_ptr(ineuray_get_version(n))
            .to_string_lossy()
            .to_owned();

        println!("version: {}", version);

        let mdl_compiler = ineuray_get_api_component_mdl_compiler(n);
        assert!(!mdl_compiler.is_null());

        let module_path = CString::new("/home/anders").unwrap();
        let result = IMdl_compiler_add_module_path(mdl_compiler, module_path.as_ptr());
        assert_eq!(result, AddPathResult::Success);

        // plugins must be loaded before neuray is started
        let nvmdl_freeimage = format!("{}/lib/nv_freeimage.so", std::env::var("MDL_ROOT").unwrap());
        let nvmdl_freeimage = CString::new(nvmdl_freeimage.as_str()).unwrap();
        let result = IMdl_compiler_load_plugin_library(mdl_compiler, nvmdl_freeimage.as_ptr());
        assert_eq!(result, BooleanResult::Success);

        // start neuray
        assert_eq!(ineuray_start(n), INeurayStartResult::Success);

        // get database and global scope
        let database = ineuray_get_api_component_database(n);
        assert!(!database.is_null());
        let global_scope = IDatabase_get_global_scope(database);
        assert!(!global_scope.is_null());

        // get mdl factory and create execution context
        let mdl_factory = ineuray_get_api_component_mdl_factory(n);
        assert!(!mdl_factory.is_null());

        let mdl_execution_context = IMdl_factory_create_execution_context(mdl_factory);
        assert!(!mdl_execution_context.is_null());

        // test set some options to their default values
        let coordinate_world = CString::new("coordinate_world").unwrap();
        let internal_space = CString::new("internal_space").unwrap();
        assert_eq!(
            IMdl_execution_context_set_option_string(
                mdl_execution_context,
                internal_space.as_ptr(),
                coordinate_world.as_ptr()
            ),
            SetOptionResult::Success
        );

        let bundle_resources = CString::new("bundle_resources").unwrap();
        assert_eq!(
            IMdl_execution_context_set_option_bool(
                mdl_execution_context,
                bundle_resources.as_ptr(),
                false,
            ),
            SetOptionResult::Success
        );

        // let mdl_wavelength_min = CString::new("mdl_wavelength_min").unwrap();
        // assert_eq!(
        //     IMdl_execution_context_set_option_float(
        //         mdl_execution_context,
        //         mdl_wavelength_min.as_ptr(),
        //         380.0,
        //     ),
        //     SetOptionResult::Success
        // );

        // check that setting the wrong type errors correctly
        let bundle_resources = CString::new("bundle_resources").unwrap();
        assert_eq!(
            IMdl_execution_context_set_option_float(
                mdl_execution_context,
                bundle_resources.as_ptr(),
                1.0,
            ),
            SetOptionResult::WrongType,
        );

        let cuda_be = IMdl_compiler_get_backend(mdl_compiler, MdlBackendKind::CudaPtx);
        assert!(!cuda_be.is_null());

        let num_texture_spaces = CString::new("num_texture_spaces").unwrap();
        let one = CString::new("1").unwrap();
        assert_eq!(
            IMdl_backend_set_option(cuda_be, num_texture_spaces.as_ptr(), one.as_ptr()),
            BackendSetOptionResult::Success
        );

        let num_texture_results = CString::new("num_texture_results").unwrap();
        let one = CString::new("16").unwrap();
        assert_eq!(
            IMdl_backend_set_option(cuda_be, num_texture_results.as_ptr(), one.as_ptr()),
            BackendSetOptionResult::Success
        );

        let sm_version = CString::new("sm_version").unwrap();
        let one = CString::new("30").unwrap();
        assert_eq!(
            IMdl_backend_set_option(cuda_be, sm_version.as_ptr(), one.as_ptr()),
            BackendSetOptionResult::Success
        );

        let tex_lookup_call_mode = CString::new("tex_lookup_call_mode").unwrap();
        let one = CString::new("optix_cp").unwrap();
        assert_eq!(
            IMdl_backend_set_option(cuda_be, tex_lookup_call_mode.as_ptr(), one.as_ptr()),
            BackendSetOptionResult::Success
        );

        let image_api = ineuray_get_api_component_image_api(n);
        assert!(!image_api.is_null());

        // release everything and shutdown neuray
        IImage_api_release(image_api);
        IMdl_backend_release(cuda_be);
        IMdl_execution_context_release(mdl_execution_context);
        IMdl_factory_release(mdl_factory);
        IScope_release(global_scope);
        IDatabase_release(database);
        IMdl_compiler_release(mdl_compiler);

        assert_eq!(ineuray_shutdown(n), INeurayShutdownResult::Success);
    }
}
