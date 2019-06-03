#include "capi-typedefs.h"
#include <mi/neuraylib/idatabase.h>

typedef mi::neuraylib::IDatabase IDatabase;
typedef mi::neuraylib::IScope IScope;

extern "C" {

IScope* IDatabase_get_global_scope(IDatabase* db) {
    return db->get_global_scope();
}

}