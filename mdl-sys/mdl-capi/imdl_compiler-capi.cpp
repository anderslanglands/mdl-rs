#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_compiler.h>

typedef mi::neuraylib::IMdl_compiler IMdl_compiler;
typedef mi::neuraylib::IMdl_compiler::Mdl_backend_kind Mdl_backend_kind;
typedef mi::neuraylib::IMdl_backend IMdl_backend;

extern "C" {
i32 IMdl_compiler_add_module_path(IMdl_compiler* c, const char* path) {
    return c->add_module_path(path);
}

i32 IMdl_compiler_remove_module_path(IMdl_compiler* c, const char* path) {
    return c->remove_module_path(path);
}

void IMdl_compiler_clear_module_paths(IMdl_compiler* c) {
    return c->clear_module_paths();
}

i32 IMdl_compiler_load_plugin_library(IMdl_compiler* c, const char* path) {
    return c->load_plugin_library(path);
}

IMdl_backend* IMdl_compiler_get_backend(IMdl_compiler* c,
                                        Mdl_backend_kind kind) {
    return c->get_backend(kind);
}
}

extern "C" {
i32 IMdl_backend_set_option(IMdl_backend* be, const char* name,
                            const char* value) {
    return be->set_option(name, value);
}
}