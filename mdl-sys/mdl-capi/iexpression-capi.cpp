#include "capi-typedefs.h"
#include <mi/neuraylib/iexpression.h>
#include <mi/neuraylib/istring.h>

typedef mi::neuraylib::IExpression IExpression;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::neuraylib::IExpression_factory IExpression_factory;
typedef mi::base::Uuid Uuid;

extern "C" {
void IExpression_release(IExpression* i) { i->release(); }
void IExpression_retain(IExpression* i) { i->retain(); }

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
