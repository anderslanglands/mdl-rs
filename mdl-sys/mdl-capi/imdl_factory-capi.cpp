#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_factory.h>
#include <mi/neuraylib/ivalue.h>

typedef mi::neuraylib::IMdl_factory IMdl_factory;
typedef mi::neuraylib::ITransaction ITransaction;
typedef mi::neuraylib::IType IType;
typedef mi::neuraylib::IMdl_execution_context IMdl_execution_context;
typedef mi::neuraylib::IType_factory IType_factory;
typedef mi::neuraylib::IValue_factory IValue_factory;
typedef mi::neuraylib::IExpression_factory IExpression_factory;

extern "C" {
void IMdl_factory_release(IMdl_factory* f) { f->release(); }
void IMdl_factory_retain(IMdl_factory* f) { f->retain(); }
IMdl_execution_context* IMdl_factory_create_execution_context(IMdl_factory* f) {
    return f->create_execution_context();
}
IType_factory* IMdl_factory_create_type_factory(IMdl_factory* f,
                                                ITransaction* t) {
    return f->create_type_factory(t);
}

IValue_factory* IMdl_factory_create_value_factory(IMdl_factory* f,
                                                  ITransaction* t) {
    return f->create_value_factory(t);
}

IExpression_factory* IMdl_factory_create_expression_factory(IMdl_factory* f,
                                                            ITransaction* t) {
    return f->create_expression_factory(t);
}
}
