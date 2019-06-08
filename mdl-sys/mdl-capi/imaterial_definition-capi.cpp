#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/imaterial_definition.h>

typedef mi::neuraylib::IMaterial_definition IMaterial_definition;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::neuraylib::IType_list IType_list;
typedef mi::base::Uuid Uuid;

extern "C" {
void IMaterial_definition_release(IMaterial_definition* t) { t->release(); }
void IMaterial_definition_retain(IMaterial_definition* t) { t->retain(); }
bool IMaterial_definition_compare_iid(Uuid id) {
    IMaterial_definition::compare_iid(id);
}
Uuid IMaterial_definition_type_get_iid() { return IMaterial_definition::IID(); }

usize IMaterial_definition_get_parameter_count(IMaterial_definition* t) {
    return t->get_parameter_count();
}
const IType_list*
IMaterial_definition_get_parameter_types(IMaterial_definition* t) {
    return t->get_parameter_types();
}
const IExpression_list*
IMaterial_definition_get_defaults(IMaterial_definition* t) {
    return t->get_defaults();
}
const char* IMaterial_definition_get_parameter_name(IMaterial_definition* t,
                                                    usize index) {
    return t->get_parameter_name(index);
}
usize IMaterial_definition_get_parameter_index(IMaterial_definition* t,
                                               const char* name) {
    return t->get_parameter_index(name);
}
}
