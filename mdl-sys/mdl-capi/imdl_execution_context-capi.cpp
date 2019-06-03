#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_execution_context.h>

typedef mi::neuraylib::IMdl_execution_context IMdl_execution_context;

extern "C" {
i32 IMdl_execution_context_set_option_string(IMdl_execution_context* c,
                                             const char* name,
                                             const char* value) {
    return c->set_option(name, value);
}

i32 IMdl_execution_context_set_option_float(IMdl_execution_context* c,
                                            const char* name, float value) {
    return c->set_option(name, value);
}

i32 IMdl_execution_context_set_option_bool(IMdl_execution_context* c,
                                           const char* name, bool value) {
    return c->set_option(name, value);
}
}
