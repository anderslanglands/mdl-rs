#include "capi-typedefs.h"
#include <mi/neuraylib/idatabase.h>

typedef mi::neuraylib::IDatabase IDatabase;
typedef mi::neuraylib::IScope IScope;
typedef mi::base::Uuid Uuid;

extern "C" {

void IDatabase_release(IDatabase* db) { db->release(); }
Uuid IDatabase_type_get_iid() { return IDatabase::IID(); }

IScope* IDatabase_get_global_scope(IDatabase* db) {
    return db->get_global_scope();
}
}
