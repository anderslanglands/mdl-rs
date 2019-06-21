#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/imaterial_instance.h>

typedef mi::neuraylib::IMaterial_instance IMaterial_instance;
typedef mi::neuraylib::ICompiled_material ICompiled_material;
typedef mi::neuraylib::IMdl_execution_context IMdl_execution_context;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::base::Uuid Uuid;

extern "C" {
void IMaterial_instance_release(IMaterial_instance* t) { t->release(); }
void IMaterial_instance_retain(IMaterial_instance* t) { t->retain(); }
bool IMaterial_instance_compare_iid(Uuid id) {
    IMaterial_instance::compare_iid(id);
}
Uuid IMaterial_instance_type_get_iid() { return IMaterial_instance::IID(); }
ICompiled_material*
IMaterial_instance_create_compiled_material(IMaterial_instance* i, u32 flags,
                                            IMdl_execution_context* ctx) {
    return i->create_compiled_material(flags, ctx);
}
}
