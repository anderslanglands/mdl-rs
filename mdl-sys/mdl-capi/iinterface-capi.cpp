#include "capi-typedefs.h"
#include <mi/base/iinterface.h>

typedef mi::base::IInterface IInterface;
typedef mi::base::Uuid Uuid;

extern "C" {
void IInterface_release(IInterface* t) { t->release(); }
void IInterface_retain(IInterface* t) { t->retain(); }
Uuid IInterface_get_iid(IInterface* i) { return i->get_iid(); }
IInterface* IInterface_get_interface(IInterface* i, Uuid id) {
    return i->get_interface(id);
}
}
