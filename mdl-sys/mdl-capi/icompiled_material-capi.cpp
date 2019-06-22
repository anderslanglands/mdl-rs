#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/icompiled_material.h>

typedef mi::neuraylib::ICompiled_material ICompiled_material;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::neuraylib::IExpression IExpression;
typedef mi::neuraylib::IExpression_direct_call IExpression_direct_call;
typedef mi::base::Uuid Uuid;
typedef mi::neuraylib::IValue IValue;

extern "C" {
void ICompiled_material_release(ICompiled_material* t) { t->release(); }
void ICompiled_material_retain(ICompiled_material* t) { t->retain(); }
bool ICompiled_material_compare_iid(Uuid id) {
    ICompiled_material::compare_iid(id);
}
Uuid ICompiled_material_type_get_iid() { return ICompiled_material::IID(); }
const IExpression_direct_call*
ICompiled_material_get_body(ICompiled_material* m) {
    return m->get_body();
}
usize ICompiled_material_get_parameter_count(ICompiled_material* m) {
    return m->get_parameter_count();
}
const char* ICompiled_material_get_parameter_name(ICompiled_material* m,
                                                  usize index) {
    return m->get_parameter_name(index);
}
const IValue* ICompiled_material_get_argument(ICompiled_material* m,
                                              usize index) {
    return m->get_argument(index);
}
usize ICompiled_material_get_temporary_count(ICompiled_material* m) {
    return m->get_temporary_count();
}
const IExpression* ICompiled_material_get_temporary(ICompiled_material* m,
                                                    usize index) {
    return m->get_temporary(index);
}
}
