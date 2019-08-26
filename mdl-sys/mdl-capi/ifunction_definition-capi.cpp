#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/ifunction_definition.h>

typedef mi::neuraylib::IFunction_definition IFunction_definition;
typedef mi::neuraylib::IFunction_definition::Semantics
    IFunction_definition_Semantics;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::neuraylib::IType_list IType_list;
typedef mi::base::Uuid Uuid;

extern "C" {
void IFunction_definition_release(IFunction_definition* t) { t->release(); }
void IFunction_definition_retain(IFunction_definition* t) { t->retain(); }
bool IFunction_definition_compare_iid(Uuid id) {
    IFunction_definition::compare_iid(id);
}
Uuid IFunction_definition_type_get_iid() { return IFunction_definition::IID(); }

usize IFunction_definition_get_parameter_count(IFunction_definition* t) {
    return t->get_parameter_count();
}
const IType_list*
IFunction_definition_get_parameter_types(IFunction_definition* t) {
    return t->get_parameter_types();
}
const IExpression_list*
IFunction_definition_get_defaults(IFunction_definition* t) {
    return t->get_defaults();
}
const char* IFunction_definition_get_parameter_name(IFunction_definition* t,
                                                    usize index) {
    return t->get_parameter_name(index);
}
usize IFunction_definition_get_parameter_index(IFunction_definition* t,
                                               const char* name) {
    return t->get_parameter_index(name);
}
const char* IFunction_definition_get_mdl_name(IFunction_definition* t) {
    return t->get_mdl_name();
}
IFunction_definition_Semantics
IFunction_definition_get_semantic(IFunction_definition* i) {
    return i->get_semantic();
}
}
