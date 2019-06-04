#include "capi-typedefs.h"
#include <mi/neuraylib/iimage_api.h>

typedef mi::neuraylib::IImage_api IImage_api;

extern "C" {
void IImage_api_release(IImage_api* i) { i->release(); }
}
