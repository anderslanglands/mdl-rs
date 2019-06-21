#include "capi-typedefs.h"
#include <mi/neuraylib/definition_wrapper.h>

typedef mi::neuraylib::Definition_wrapper Definition_wrapper;
typedef mi::neuraylib::ITransaction ITransaction;
typedef mi::neuraylib::IMdl_factory IMdl_factory;
typedef mi::neuraylib::Element_type Element_type;
typedef mi::neuraylib::IType_list IType_list;
typedef mi::neuraylib::IType IType;
typedef mi::neuraylib::IExpression_list IExpression_list;
typedef mi::neuraylib::IAnnotation_block IAnnotation_block;
typedef mi::neuraylib::IAnnotation_list IAnnotation_list;
typedef mi::neuraylib::IScene_element IScene_element;

extern "C" {
Definition_wrapper* Definition_wrapper_new(ITransaction* transaction,
                                           const char* name,
                                           IMdl_factory* factory) {
    return new Definition_wrapper(transaction, name, factory);
}

void Definition_wrapper_delete(Definition_wrapper* df) { delete df; }

bool Definition_wrapper_is_valid(Definition_wrapper* df) {
    return df->is_valid();
}

Element_type Definition_wrapper_get_type(Definition_wrapper* dw) {
    return dw->get_type();
}

const char* Definition_wrapper_get_mdl_definition(Definition_wrapper* df) {
    return df->get_mdl_definition();
}

const char* Definition_wrapper_get_module(Definition_wrapper* df) {
    return df->get_module();
}

bool Definition_wrapper_is_exported(Definition_wrapper* df) {
    return df->is_exported();
}

usize Definition_wrapper_get_parameter_count(Definition_wrapper* df) {
    return df->get_parameter_count();
}

const char* Definition_wrapper_get_parameter_name(Definition_wrapper* df,
                                                  usize index) {
    return df->get_parameter_name(index);
}

usize Definition_wrapper_get_parameter_index(Definition_wrapper* df,
                                             const char* name) {
    return df->get_parameter_index(name);
}

const IType_list*
Definition_wrapper_get_parameter_types(Definition_wrapper* df) {
    return df->get_parameter_types();
}

const IType* Definition_wrapper_get_return_type(Definition_wrapper* df) {
    return df->get_return_type();
}

const char* Definition_wrapper_get_thumbnail(Definition_wrapper* df) {
    return df->get_thumbnail();
}

const IExpression_list*
Definition_wrapper_get_defaults(Definition_wrapper* df) {
    return df->get_defaults();
}

i32 Definition_wrapper_get_default_bool(Definition_wrapper* dw, usize index,
                                        bool* value) {
    return dw->get_default(index, *value);
}

i32 Definition_wrapper_get_default_int(Definition_wrapper* dw, usize index,
                                       int* value) {
    return dw->get_default(index, *value);
}

i32 Definition_wrapper_get_default_float(Definition_wrapper* dw, usize index,
                                         float* value) {
    return dw->get_default(index, *value);
}

const IAnnotation_block*
Definition_wrapper_get_annotations(Definition_wrapper* df) {
    return df->get_annotations();
}

const IAnnotation_block*
Definition_wrapper_get_return_annotations(Definition_wrapper* df) {
    return df->get_return_annotations();
}

const IAnnotation_list*
Definition_wrapper_get_parameter_annotations(Definition_wrapper* df) {
    return df->get_parameter_annotations();
}

const IExpression_list*
Definition_wrapper_get_enable_if_conditions(Definition_wrapper* df) {
    return df->get_enable_if_conditions();
}

usize Definition_wrapper_get_enable_if_users(Definition_wrapper* df,
                                             usize index) {
    return df->get_enable_if_users(index);
}

usize Definition_wrapper_get_enable_if_user(Definition_wrapper* df, usize index,
                                            usize u_index) {
    return df->get_enable_if_user(index, u_index);
}

IScene_element* Definition_wrapper_create_instance(Definition_wrapper* dw,
                                                   IExpression_list* args,
                                                   i32* errors) {
    return dw->create_instance(args, errors);
}

ITransaction* Definition_wrapper_get_transaction(Definition_wrapper* dw) {
    return dw->get_transaction();
}

IMdl_factory* Definition_wrapper_get_mdl_factory(Definition_wrapper* dw) {
    return dw->get_mdl_factory();
}

const IScene_element*
Definition_wrapper_get_scene_element(Definition_wrapper* dw) {
    return dw->get_scene_element();
}

Element_type Definition_wrapper_get_element_type(Definition_wrapper* dw) {
    return dw->get_element_type();
}

const char* Definition_wrapper_get_name(Definition_wrapper* dw) {
    return dw->get_name().c_str();
}
}
