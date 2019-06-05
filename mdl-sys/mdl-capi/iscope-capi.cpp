#include "capi-typedefs.h"
#include <mi/neuraylib/iscope.h>

typedef mi::neuraylib::IScope IScope;
typedef mi::neuraylib::ITransaction ITransaction;

extern "C" {
void IScope_release(IScope* s) { s->release(); }
void IScope_retain(IScope* s) { s->retain(); }
ITransaction* IScope_create_transaction(IScope* s) {
    return s->create_transaction();
}
}