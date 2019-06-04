#include "capi-typedefs.h"
#include <mi/neuraylib/iscope.h>

typedef mi::neuraylib::IScope IScope;

extern "C" {
void IScope_release(IScope* s) { s->release(); }
}