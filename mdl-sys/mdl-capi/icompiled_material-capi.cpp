#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/icompiled_material.h>

typedef mi::neuraylib::ICompiled_material ICompiled_material;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::base::Uuid Uuid;

extern "C" {
void ICompiled_material_release(ICompiled_material* t) { t->release(); }
void ICompiled_material_retain(ICompiled_material* t) { t->retain(); }
bool ICompiled_material_compare_iid(Uuid id) {
    ICompiled_material::compare_iid(id);
}
Uuid ICompiled_material_type_get_iid() { return ICompiled_material::IID(); }
}
