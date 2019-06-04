#include "capi-typedefs.h"
#include <mi/neuraylib/iversion.h>

typedef mi::neuraylib::IVersion IVersion;
typedef mi::base::Uuid Uuid;

extern "C" {
void IVersion_release(IVersion* i) { i->release(); }
const char* IVersion_get_product_name(IVersion* i) {
    return i->get_product_name();
}
const char* IVersion_get_product_version(IVersion* i) {
    return i->get_product_version();
}
const char* IVersion_get_build_number(IVersion* i) {
    return i->get_build_number();
}
const char* IVersion_get_build_date(IVersion* i) { return i->get_build_date(); }
const char* IVersion_get_build_platform(IVersion* i) {
    return i->get_build_platform();
}
const char* IVersion_get_string(IVersion* i) { return i->get_string(); }
Uuid IVersion_get_neuray_iid(IVersion* i) { return i->get_neuray_iid(); }
}
