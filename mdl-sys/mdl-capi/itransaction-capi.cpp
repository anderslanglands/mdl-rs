#include "capi-typedefs.h"
#include <mi/neuraylib/ifunction_definition.h>
#include <mi/neuraylib/imaterial_definition.h>
#include <mi/neuraylib/imodule.h>
#include <mi/neuraylib/itransaction.h>

typedef mi::neuraylib::ITransaction ITransaction;
typedef mi::neuraylib::IFunction_definition IFunction_definition;
typedef mi::neuraylib::IMaterial_definition IMaterial_definition;
typedef mi::neuraylib::IModule IModule;
typedef mi::base::IInterface IInterface;

extern "C" {
void ITransaction_release(ITransaction* t) { t->release(); }
void ITransaction_retain(ITransaction* t) { t->retain(); }
const IMaterial_definition*
ITransaction_access_material_definition(ITransaction* t, const char* name) {
    return t->access<IMaterial_definition>(name);
}

const IFunction_definition*
ITransaction_access_function_definition(ITransaction* t, const char* name) {
    return t->access<IFunction_definition>(name);
}

const IInterface* ITransaction_access(ITransaction* t, const char* name) {
    return t->access(name);
}

const IModule* ITransaction_access_module(ITransaction* t, const char* name) {
    return t->access<IModule>(name);
}
}
