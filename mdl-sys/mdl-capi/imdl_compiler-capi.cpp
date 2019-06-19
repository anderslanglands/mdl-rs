#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_compiler.h>

typedef mi::neuraylib::IMdl_compiler IMdl_compiler;
typedef mi::neuraylib::IMdl_compiler::Mdl_backend_kind Mdl_backend_kind;
typedef mi::neuraylib::IMdl_backend IMdl_backend;
typedef mi::neuraylib::IMdl_execution_context IMdl_execution_context;
typedef mi::neuraylib::ITransaction ITransaction;
typedef mi::base::Uuid Uuid;

extern "C" {

void IMdl_compiler_release(IMdl_compiler* c) { c->release(); }
void IMdl_compiler_retain(IMdl_compiler* c) { c->retain(); }
Uuid IMdl_compiler_type_get_iid() { return IMdl_compiler::IID(); }

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

i32 IMdl_compiler_load_module(IMdl_compiler* c, ITransaction* transaction,
                              const char* name, IMdl_execution_context* ctx) {
    return c->load_module(transaction, name, ctx);
}
}

extern "C" {
void IMdl_backend_release(IMdl_backend* be) { be->release(); }
i32 IMdl_backend_set_option(IMdl_backend* be, const char* name,
                            const char* value) {
    return be->set_option(name, value);
}
}
