#include "capi-typedefs.h"
#include <mi/neuraylib/ifunction_definition.h>

typedef mi::neuraylib::IFunction_definition IFunction_definition;

extern "C" {
void IFunction_definition_release(IFunction_definition* f) { f->release(); }
void IFunction_definition_retain(IFunction_definition* f) { f->retain(); }
}
