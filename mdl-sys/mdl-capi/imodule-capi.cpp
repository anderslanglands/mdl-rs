#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/imodule.h>
#include <mi/neuraylib/itype.h>
#include <mi/neuraylib/ivalue.h>

typedef mi::neuraylib::IModule IModule;
typedef mi::neuraylib::IType_list IType_list;
typedef mi::neuraylib::IType_resource IType_resource;
typedef mi::neuraylib::IValue_list IValue_list;
typedef mi::base::Uuid Uuid;

extern "C" {
void IModule_release(IModule* t) { t->release(); }
void IModule_retain(IModule* t) { t->retain(); }
bool IModule_compare_iid(Uuid id) { IModule::compare_iid(id); }
Uuid IModule_type_get_iid() { return IModule::IID(); }

const char* IModule_get_filename(IModule* m) { return m->get_filename(); }
const char* IModule_get_mdl_name(IModule* m) { return m->get_mdl_name(); }
usize IModule_get_import_count(IModule* m) { return m->get_import_count(); }
const char* IModule_get_import(IModule* m, usize index) {
    return m->get_import(index);
}
const IType_list* IModule_get_types(IModule* m) { return m->get_types(); }
const IValue_list* IModule_get_constants(IModule* m) {
    return m->get_constants();
}
const char* IModule_get_function(IModule* m, usize index) {
    return m->get_function(index);
}
usize IModule_get_function_count(IModule* m) { return m->get_function_count(); }
const char* IModule_get_material(IModule* m, usize index) {
    return m->get_material(index);
}
usize IModule_get_material_count(IModule* m) { return m->get_material_count(); }

usize IModule_get_resources_count(IModule* m) {
    return m->get_resources_count();
}
const char* IModule_get_resource_name(IModule* m, usize index) {
    return m->get_resource_name(index);
}
const char* IModule_get_resource_mdl_file_path(IModule* m, usize index) {
    return m->get_resource_mdl_file_path(index);
}
const IType_resource* IModule_get_resource_type(IModule* m, usize index) {
    return m->get_resource_type(index);
}
}
