#include "capi-typedefs.h"
#include <mi/neuraylib/itype.h>

typedef mi::neuraylib::IType_list IType_list;
typedef mi::neuraylib::IType IType;

extern "C" {
void IType_list_release(IType_list* s) { s->release(); }
void IType_list_retain(IType_list* s) { s->retain(); }
usize IType_list_get_size(IType_list* l) { return l->get_size(); }
usize IType_list_get_index(IType_list* l, const char* name) {
    return l->get_index(name);
}
const char* IType_list_get_name(IType_list* l, usize index) {
    return l->get_name(index);
}
const IType* IType_list_get_type(IType_list* l, usize index) {
    return l->get_type(index);
}
}
