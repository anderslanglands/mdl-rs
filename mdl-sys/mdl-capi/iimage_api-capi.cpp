#include "capi-typedefs.h"
#include <mi/neuraylib/iimage_api.h>

typedef mi::neuraylib::IImage_api IImage_api;
typedef mi::base::Uuid Uuid;

extern "C" {
void IImage_api_release(IImage_api* i) { i->release(); }
void IImage_api_retain(IImage_api* i) { i->retain(); }
Uuid IImage_api_type_get_iid() { return IImage_api::IID(); }
}
