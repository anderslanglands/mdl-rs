#include "capi-typedefs.h"
#include <mi/neuraylib/iexpression.h>
#include <mi/neuraylib/istring.h>

typedef mi::neuraylib::IExpression IExpression;
typedef mi::neuraylib::IExpression::Kind IExpressionKind;
typedef mi::neuraylib::IType IType;
typedef mi::neuraylib::IValue IValue;
typedef mi::neuraylib::IExpression_direct_call IExpression_direct_call;
typedef mi::neuraylib::IExpression_call IExpression_call;
typedef mi::neuraylib::IExpression_constant IExpression_constant;
typedef mi::neuraylib::IExpression_parameter IExpression_parameter;
typedef mi::neuraylib::IExpression_temporary IExpression_temporary;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::neuraylib::IExpression_factory IExpression_factory;
typedef mi::base::Uuid Uuid;

extern "C" {
void IExpression_release(IExpression* i) { i->release(); }
void IExpression_retain(IExpression* i) { i->retain(); }
bool IExpression_compare_iid(Uuid id) { IExpression::compare_iid(id); }
Uuid IExpression_type_get_iid() { return IExpression::IID(); }
IExpressionKind IExpression_get_kind(IExpression* i) { return i->get_kind(); }
const IType* IExpression_get_type(IExpression* i) { return i->get_type(); }

void IExpression_list_release(IExpression_list* i) { i->release(); }
void IExpression_list_retain(IExpression_list* i) { i->retain(); }
usize IExpression_list_get_size(IExpression_list* i) { return i->get_size(); }
const IExpression* IExpression_list_get_expression(IExpression_list* i,
                                                   usize index) {
    return i->get_expression(index);
}
const IExpression* IExpression_list_get_expression_by_name(IExpression_list* i,
                                                           const char* name) {
    return i->get_expression(name);
}
usize IExpression_list_get_index(IExpression_list* l, const char* name) {
    return l->get_index(name);
}
const char* IExpression_list_get_name(IExpression_list* l, usize index) {
    return l->get_name(index);
}

void IExpression_factory_release(IExpression_factory* i) { i->release(); }
void IExpression_factory_retain(IExpression_factory* i) { i->retain(); }
const char* IExpression_factory_dump(IExpression_factory* i, IExpression* val,
                                     const char* name, usize depth) {
    return i->dump(val, name, depth)->get_c_str();
}
}

extern "C" {
void IExpression_direct_call_release(IExpression_direct_call* i) {
    i->release();
}
void IExpression_direct_call_retain(IExpression_direct_call* i) { i->retain(); }
bool IExpression_direct_call_compare_iid(Uuid id) {
    IExpression_direct_call::compare_iid(id);
}
Uuid IExpression_direct_call_type_get_iid() {
    return IExpression_direct_call::IID();
}
const char* IExpression_direct_call_get_definition(IExpression_direct_call* i) {
    return i->get_definition();
}
const IExpression_list*
IExpression_direct_call_get_arguments(IExpression_direct_call* i) {
    return i->get_arguments();
}
}

extern "C" {
void IExpression_constant_release(IExpression_constant* i) { i->release(); }
void IExpression_constant_retain(IExpression_constant* i) { i->retain(); }
bool IExpression_constant_compare_iid(Uuid id) {
    IExpression_constant::compare_iid(id);
}
Uuid IExpression_constant_type_get_iid() { return IExpression_constant::IID(); }
const IValue* IExpression_constant_get_value(IExpression_constant* i) {
    return i->get_value();
}
}

extern "C" {
void IExpression_call_release(IExpression_call* i) { i->release(); }
void IExpression_call_retain(IExpression_call* i) { i->retain(); }
bool IExpression_call_compare_iid(Uuid id) {
    IExpression_call::compare_iid(id);
}
Uuid IExpression_call_type_get_iid() { return IExpression_call::IID(); }
}

extern "C" {
void IExpression_parameter_release(IExpression_parameter* i) { i->release(); }
void IExpression_parameter_retain(IExpression_parameter* i) { i->retain(); }
bool IExpression_parameter_compare_iid(Uuid id) {
    IExpression_parameter::compare_iid(id);
}
Uuid IExpression_parameter_type_get_iid() {
    return IExpression_parameter::IID();
}
usize IExpression_parameter_get_index(IExpression_parameter* i) {
    return i->get_index();
}
void IExpression_parameter_set_index(IExpression_parameter* i, usize index) {
    i->set_index(index);
}
}

extern "C" {
void IExpression_temporary_release(IExpression_temporary* i) { i->release(); }
void IExpression_temporary_retain(IExpression_temporary* i) { i->retain(); }
bool IExpression_temporary_compare_iid(Uuid id) {
    IExpression_temporary::compare_iid(id);
}
Uuid IExpression_temporary_type_get_iid() {
    return IExpression_temporary::IID();
}
usize IExpression_temporary_get_index(IExpression_temporary* i) {
    return i->get_index();
}
void IExpression_temporary_set_index(IExpression_temporary* i, usize index) {
    i->set_index(index);
}
}
