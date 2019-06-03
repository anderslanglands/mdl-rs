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

extern "C" {
INeuray* load_ineuray() { return load_and_get_ineuray(); }

u32 ineuray_get_interface_version(INeuray* n) {
    return n->get_interface_version();
}

const char* ineuray_get_version(INeuray* n) { return n->get_version(); }

i32 ineuray_start(INeuray* n) { return n->start(); }
i32 ineuray_shutdown(INeuray* n) { return n->shutdown(); }

IDatabase* ineuray_get_api_component_database(INeuray* n) {
    return n->get_api_component<IDatabase>();
}

IDebug_configuration*
ineuray_get_api_component_debug_configuration(INeuray* n) {
    return n->get_api_component<IDebug_configuration>();
}

IFactory* ineuray_get_api_component_factory(INeuray* n) {
    return n->get_api_component<IFactory>();
}

IImage_api* ineuray_get_api_component_image_api(INeuray* n) {
    return n->get_api_component<IImage_api>();
}

IMdl_archive_api* ineuray_get_api_component_mdl_archive_api(INeuray* n) {
    return n->get_api_component<IMdl_archive_api>();
}

IMdl_compiler* ineuray_get_api_component_mdl_compiler(INeuray* n) {
    return n->get_api_component<IMdl_compiler>();
}

IMdl_discovery_api* ineuray_get_api_component_discovery_api(INeuray* n) {
    return n->get_api_component<IMdl_discovery_api>();
}

IMdl_distiller_api* ineuray_get_api_component_distiller_api(INeuray* n) {
    return n->get_api_component<IMdl_distiller_api>();
}

IMdl_evaluator_api* ineuray_get_api_component_evaluator_api(INeuray* n) {
    return n->get_api_component<IMdl_evaluator_api>();
}

IMdl_factory* ineuray_get_api_component_mdl_factory(INeuray* n) {
    return n->get_api_component<IMdl_factory>();
}

IMdle_api* ineuray_get_api_component_mdle_api(INeuray* n) {
    return n->get_api_component<IMdle_api>();
}

IVersion* ineuray_get_api_component_version(INeuray* n) {
    return n->get_api_component<IVersion>();
}
}
