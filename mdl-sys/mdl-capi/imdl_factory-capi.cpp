#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_factory.h>

typedef mi::neuraylib::IMdl_factory IMdl_factory;
typedef mi::neuraylib::IMdl_execution_context IMdl_execution_context;

extern "C" {
IMdl_execution_context* IMdl_factory_create_execution_context(IMdl_factory* f) {
    return f->create_execution_context();
}
}
