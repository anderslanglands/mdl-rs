#include "capi-typedefs.h"
#include <mi/neuraylib/istring.h>
#include <mi/neuraylib/ivalue.h>

typedef mi::neuraylib::IValue IValue;
typedef mi::neuraylib::IValue_list IValue_list;
typedef mi::neuraylib::IValue_factory IValue_factory;
typedef mi::base::Uuid Uuid;

extern "C" {
void IValue_release(IValue* i) { i->release(); }
void IValue_retain(IValue* i) { i->retain(); }

void IValue_list_release(IValue_list* i) { i->release(); }
void IValue_list_retain(IValue_list* i) { i->retain(); }
usize IValue_list_get_size(IValue_list* i) { return i->get_size(); }
const IValue* IValue_list_get_value(IValue_list* i, usize index) {
    return i->get_value(index);
}
usize IValue_list_get_index(IValue_list* l, const char* name) {
    return l->get_index(name);
}
const char* IValue_list_get_name(IValue_list* l, usize index) {
    return l->get_name(index);
}

void IValue_factory_release(IValue_factory* i) { i->release(); }
void IValue_factory_retain(IValue_factory* i) { i->retain(); }
const char* IValue_factory_dump(IValue_factory* i, IValue* val,
                                const char* name, usize depth) {
    return i->dump(val, name, depth)->get_c_str();
}
}
