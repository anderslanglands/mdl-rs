#include "capi-typedefs.h"
#include <mi/base.h>
#include <mi/neuraylib/iscene_element.h>

typedef mi::neuraylib::IScene_element IScene_element;
typedef mi::neuraylib::Element_type Element_type;
typedef mi::base::Uuid Uuid;

extern "C" {
void IScene_element_release(IScene_element* t) { t->release(); }
void IScene_element_retain(IScene_element* t) { t->retain(); }
bool IScene_element_compare_iid(Uuid id) { IScene_element::compare_iid(id); }
Uuid IScene_element_type_get_iid() { return IScene_element::IID(); }
Element_type IScene_element_get_element_type(IScene_element* t) {
    return t->get_element_type();
}
}
