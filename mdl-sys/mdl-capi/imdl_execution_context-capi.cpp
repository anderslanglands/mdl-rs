#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_execution_context.h>

typedef mi::neuraylib::IMdl_execution_context IMdl_execution_context;
typedef mi::neuraylib::IMessage IMessage;
typedef mi::base::Uuid Uuid;
typedef mi::neuraylib::IMessage::Kind MessageKind;
typedef mi::base::Message_severity Message_severity;

extern "C" {
void IMdl_execution_context_release(IMdl_execution_context* c) { c->release(); }
void IMdl_execution_context_retain(IMdl_execution_context* c) { c->retain(); }
bool IMdl_execution_context_compare_iid(Uuid id) {
    IMdl_execution_context::compare_iid(id);
}
Uuid IMdl_execution_context_type_get_iid() {
    return IMdl_execution_context::IID();
}

i32 IMdl_execution_context_set_option_string(IMdl_execution_context* c,
                                             const char* name,
                                             const char* value) {
    return c->set_option(name, value);
}

i32 IMdl_execution_context_set_option_float(IMdl_execution_context* c,
                                            const char* name, float value) {
    return c->set_option(name, value);
}

i32 IMdl_execution_context_set_option_bool(IMdl_execution_context* c,
                                           const char* name, bool value) {
    return c->set_option(name, value);
}

usize IMdl_execution_context_get_messages_count(IMdl_execution_context* c) {
    return c->get_messages_count();
}

usize IMdl_execution_context_get_error_messages_count(
    IMdl_execution_context* c) {
    return c->get_error_messages_count();
}

const IMessage* IMdl_execution_context_get_message(IMdl_execution_context* c,
                                                   usize index) {
    return c->get_message(index);
}

const IMessage*
IMdl_execution_context_get_error_message(IMdl_execution_context* c,
                                         usize index) {
    return c->get_error_message(index);
}
}

extern "C" {
void IMessage_release(IMessage* c) { c->release(); }
void IMessage_retain(IMessage* c) { c->retain(); }
bool IMessage_compare_iid(Uuid id) { IMessage::compare_iid(id); }
Uuid IMessage_type_get_iid() { return IMessage::IID(); }

MessageKind IMessage_get_kind(IMessage* m) { return m->get_kind(); }
Message_severity IMessage_get_severity(IMessage* m) {
    return m->get_severity();
}
const char* IMessage_get_string(IMessage* m) { return m->get_string(); }
i32 IMessage_get_code(IMessage* m) { return m->get_code(); }
usize IMessage_get_notes_count(IMessage* m) { return m->get_notes_count(); }
const IMessage* IMessage_get_note(IMessage* m, usize index) {
    return m->get_note(index);
}
}
