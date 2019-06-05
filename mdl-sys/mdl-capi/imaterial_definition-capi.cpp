#include "capi-typedefs.h"
#include <mi/neuraylib/imaterial_definition.h>

typedef mi::neuraylib::IMaterial_definition IMaterial_definition;

extern "C" {
void IMaterial_definition_release(IMaterial_definition* m) { m->release(); }
void IMaterial_definition_retain(IMaterial_definition* m) { m->retain(); }
}
