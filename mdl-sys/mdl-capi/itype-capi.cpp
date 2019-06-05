#include "capi-typedefs.h"
#include <mi/neuraylib/istring.h>
#include <mi/neuraylib/itype.h>

typedef mi::neuraylib::IType IType;
typedef mi::neuraylib::IType_factory IType_factory;

extern "C" {
void IType_release(IType* f) { f->release(); }
void IType_retain(IType* f) { f->retain(); }

void IType_factory_release(IType_factory* f) { f->release(); }
void IType_factory_retain(IType_factory* f) { f->retain(); }
const char* IType_factory_dump(IType_factory* f, IType* type, usize depth) {
    return f->dump(type, depth)->get_c_str();
}
}
