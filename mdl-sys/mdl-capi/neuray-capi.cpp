#include "capi-typedefs.h"
#include "neuray_loader.h"

typedef mi::neuraylib::INeuray INeuray;

// API components
typedef mi::neuraylib::IDatabase IDatabase;
typedef mi::neuraylib::IDebug_configuration IDebug_configuration;
typedef mi::neuraylib::IFactory IFactory;
typedef mi::neuraylib::IImage_api IImage_api;
typedef mi::neuraylib::IMdl_archive_api IMdl_archive_api;
typedef mi::neuraylib::IMdl_compiler IMdl_compiler;
typedef mi::neuraylib::IMdl_discovery_api IMdl_discovery_api;
typedef mi::neuraylib::IMdl_distiller_api IMdl_distiller_api;
typedef mi::neuraylib::IMdl_evaluator_api IMdl_evaluator_api;
typedef mi::neuraylib::IMdl_factory IMdl_factory;
typedef mi::neuraylib::IMdle_api IMdle_api;
typedef mi::neuraylib::IVersion IVersion;
typedef mi::base::IInterface IInterface;
typedef mi::base::Uuid Uuid;

extern "C" {
INeuray* load_ineuray() { return load_and_get_ineuray(); }

u32 ineuray_get_interface_version(INeuray* n) {
    return n->get_interface_version();
}

const char* ineuray_get_version(INeuray* n) { return n->get_version(); }

i32 ineuray_start(INeuray* n) { return n->start(); }
i32 ineuray_shutdown(INeuray* n) { return n->shutdown(); }

IInterface* ineuray_get_api_component(INeuray* n, Uuid iid) {
    return n->get_api_component(iid);
}

}
