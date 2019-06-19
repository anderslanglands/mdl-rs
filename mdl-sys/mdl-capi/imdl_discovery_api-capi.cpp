#include "capi-typedefs.h"
#include <mi/neuraylib/imdl_discovery_api.h>
#include <mi/neuraylib/istring.h>

typedef mi::neuraylib::IMdl_info IMdl_info;
typedef mi::neuraylib::IMdl_info::Kind Kind;
typedef mi::neuraylib::IMdl_module_info IMdl_module_info;
typedef mi::neuraylib::IMdl_xliff_info IMdl_xliff_info;
typedef mi::neuraylib::IMdl_discovery_api IMdl_discovery_api;
typedef mi::neuraylib::IMdl_discovery_result IMdl_discovery_result;
typedef mi::neuraylib::IMdl_package_info IMdl_package_info;
typedef mi::neuraylib::IMdl_resource_info IMdl_resource_info;
typedef mi::neuraylib::IMdl_texture_info IMdl_texture_info;
typedef mi::neuraylib::IMdl_lightprofile_info IMdl_lightprofile_info;
typedef mi::neuraylib::IMdl_measured_bsdf_info IMdl_measured_bsdf_info;
typedef mi::base::Uuid Uuid;

extern "C" {
Kind IMdl_info_get_kind(IMdl_info* info) { return info->get_kind(); }

const char* IMdl_info_get_qualified_name(IMdl_info* info) {
    return info->get_qualified_name();
}

const char* IMdl_info_get_simple_name(IMdl_info* info) {
    return info->get_simple_name();
}
Uuid IMdl_info_type_get_iid() { return IMdl_info::IID(); }

// IMdl_module_info
void IMdl_module_info_release(IMdl_module_info* info) { info->release(); }

void IMdl_module_info_retain(IMdl_module_info* info) { info->retain(); }

/// Returns the index of the search path where this module has been found.
usize IMdl_module_info_get_search_path_index(IMdl_module_info* info) {
    return info->get_search_path_index();
}
/// Returns the search path in the local file system that contains this module.
const char* IMdl_module_info_get_search_path(IMdl_module_info* info) {
    return info->get_search_path();
}
/// Returns an absolute path to the module file in the local file system.
const char* IMdl_module_info_get_resolved_path(IMdl_module_info* info) {
    return info->get_resolved_path()->get_c_str();
}
/// Returns true if the module has been discovered inside of an archive, false
/// if not.
bool IMdl_module_info_in_archive(IMdl_module_info* info) {
    return info->in_archive();
}

/// Returns the number of shadows of this module.
usize IMdl_module_info_get_shadows_count(IMdl_module_info* info) {
    return info->get_shadows_count();
}
/// Returns one of the shadows this module has.
/// \param index     Index in the shadow list of this module.
const IMdl_module_info* IMdl_module_info_get_shadow(IMdl_module_info* info,
                                                    usize index) {
    return info->get_shadow(index);
}

Uuid IMdl_module_info_type_get_iid() { return IMdl_module_info::IID(); }

/// Returns an absolute path to the xliff file in the local file system.
const char* IMdl_xliff_info_get_resolved_path(IMdl_xliff_info* info) {
    return info->get_resolved_path();
}

/// Returns the index of the search path where the xliff file has been found.
usize IMdl_xliff_info_get_search_path_index(IMdl_xliff_info* info) {
    return info->get_search_path_index();
}

/// Returns the search path in the local file system that contains this xliff
/// file.
const char* IMdl_xliff_info_get_search_path(IMdl_xliff_info* info) {
    return info->get_search_path();
}

/// Returns true if the xliff file has been discovered inside of an archive,
/// false if not.
bool IMdl_xliff_info_in_archive(IMdl_xliff_info* info) {
    return info->in_archive();
}

Uuid IMdl_xliff_info_type_get_iid() { return IMdl_xliff_info::IID(); }

Uuid IMdl_discovery_api_type_get_iid() { return IMdl_discovery_api::IID(); }
const IMdl_discovery_result* IMdl_discovery_api_discover(IMdl_discovery_api* d,
                                                         Kind filter) {
    return d->discover(filter);
}

const IMdl_package_info*
IMdl_discovery_result_get_graph(IMdl_discovery_result* r) {
    return r->get_graph();
}
usize IMdl_discovery_result_get_search_paths_count(IMdl_discovery_result* r) {
    return r->get_search_paths_count();
}
const char* IMdl_discovery_result_get_search_path(IMdl_discovery_result* r,
                                                  usize index) {
    return r->get_search_path(index);
}

usize IMdl_package_info_get_child_count(IMdl_package_info* i) {
    return i->get_child_count();
}

const IMdl_info* IMdl_package_info_get_child(IMdl_package_info* i,
                                             usize index) {
    return i->get_child(index);
}

usize IMdl_package_info_get_search_path_index_count(IMdl_package_info* i) {
    return i->get_search_path_index_count();
}

usize IMdl_package_info_get_search_path_index(IMdl_package_info* i,
                                              usize index) {
    return i->get_search_path_index(index);
}

const char* IMdl_package_info_get_search_path(IMdl_package_info* i,
                                              usize index) {
    return i->get_search_path(index);
}

const char* IMdl_package_info_get_resolved_path(IMdl_package_info* i,
                                                usize index) {
    return i->get_resolved_path(index)->get_c_str();
}

bool IMdl_package_info_in_archive(IMdl_package_info* i, usize index) {
    return i->in_archive(index);
}

Uuid IMdl_package_info_type_get_iid() { return IMdl_package_info::IID(); }

/// Returns the index of the search path where this resource has been found.
usize IMdl_resource_info_get_search_path_index(IMdl_resource_info* info) {
    return info->get_search_path_index();
}

/// Returns the search path in the local file system that contains this
/// resource.
const char* IMdl_resource_info_get_search_path(IMdl_resource_info* info) {
    return info->get_search_path();
}
/// Returns an absolute path to the resource file in the local file system.
const char* IMdl_resource_info_get_resolved_path(IMdl_resource_info* info) {
    return info->get_resolved_path();
}
/// Returns true if the resource has been discovered inside of an archive, false
/// if not.
bool IMdl_resource_info_in_archive(IMdl_resource_info* info) {
    return info->in_archive();
}

/// Returns the number of shadows of this resource.
usize IMdl_resource_info_get_shadows_count(IMdl_resource_info* info) {
    return info->get_shadows_count();
}
/// Returns one of the shadows this resource has.
/// \param index     Index in the shadow list of this resource.
const IMdl_resource_info*
IMdl_resource_info_get_shadow(IMdl_resource_info* info, usize index) {
    return info->get_shadow(index);
}

/// Returns the extension of the resource file
const char* IMdl_resource_get_extension(IMdl_resource_info* info) {
    return info->get_extension();
}
Uuid IMdl_resource_info_type_get_iid() { return IMdl_resource_info::IID(); }

/// Returns the index of the search path where this texture has been found.
usize IMdl_texture_info_get_search_path_index(IMdl_texture_info* info) {
    return info->get_search_path_index();
}
/// Returns the search path in the local file system that contains this
/// texture.
const char* IMdl_texture_info_get_search_path(IMdl_texture_info* info) {
    return info->get_search_path();
}
/// Returns an absolute path to the texture file in the local file system.
const char* IMdl_texture_info_get_resolved_path(IMdl_texture_info* info) {
    return info->get_resolved_path();
}
/// Returns true if the texture has been discovered inside of an archive, false
/// if not.
bool IMdl_texture_info_in_archive(IMdl_texture_info* info) {
    return info->in_archive();
}

/// Returns the number of shadows of this texture.
usize IMdl_texture_info_get_shadows_count(IMdl_texture_info* info) {
    return info->get_shadows_count();
}
/// Returns one of the shadows this texture has.
/// \param index     Index in the shadow list of this texture.
const IMdl_resource_info* IMdl_texture_info_get_shadow(IMdl_texture_info* info,
                                                       usize index) {
    return info->get_shadow(index);
}

/// Returns the extension of the texture file
const char* IMdl_texture_get_extension(IMdl_texture_info* info) {
    return info->get_extension();
}
Uuid IMdl_texture_info_type_get_iid() { return IMdl_texture_info::IID(); }

/// Returns the index of the search path where this lightprofile has been found.
usize IMdl_lightprofile_info_get_search_path_index(
    IMdl_lightprofile_info* info) {
    return info->get_search_path_index();
}
/// Returns the search path in the local file system that contains this
/// lightprofile.
const char*
IMdl_lightprofile_info_get_search_path(IMdl_lightprofile_info* info) {
    return info->get_search_path();
}
/// Returns an absolute path to the lightprofile file in the local file system.
const char*
IMdl_lightprofile_info_get_resolved_path(IMdl_lightprofile_info* info) {
    return info->get_resolved_path();
}
/// Returns true if the lightprofile has been discovered inside of an archive,
/// false if not.
bool IMdl_lightprofile_info_in_archive(IMdl_lightprofile_info* info) {
    return info->in_archive();
}

/// Returns the number of shadows of this lightprofile.
usize IMdl_lightprofile_info_get_shadows_count(IMdl_lightprofile_info* info) {
    return info->get_shadows_count();
}
/// Returns one of the shadows this lightprofile has.
/// \param index     Index in the shadow list of this lightprofile.
const IMdl_resource_info*
IMdl_lightprofile_info_get_shadow(IMdl_lightprofile_info* info, usize index) {
    return info->get_shadow(index);
}

/// Returns the extension of the lightprofile file
const char* IMdl_lightprofile_get_extension(IMdl_lightprofile_info* info) {
    return info->get_extension();
}
Uuid IMdl_lightprofile_info_type_get_iid() {
    return IMdl_lightprofile_info::IID();
}

/// Returns the index of the search path where this measured_bsdf has been
/// found.
usize IMdl_measured_bsdf_info_get_search_path_index(
    IMdl_measured_bsdf_info* info) {
    return info->get_search_path_index();
}
/// Returns the search path in the local file system that contains this
/// measured_bsdf.
const char*
IMdl_measured_bsdf_info_get_search_path(IMdl_measured_bsdf_info* info) {
    return info->get_search_path();
}
/// Returns an absolute path to the measured_bsdf file in the local file system.
const char*
IMdl_measured_bsdf_info_get_resolved_path(IMdl_measured_bsdf_info* info) {
    return info->get_resolved_path();
}
/// Returns true if the measured_bsdf has been discovered inside of an archive,
/// false if not.
bool IMdl_measured_bsdf_info_in_archive(IMdl_measured_bsdf_info* info) {
    return info->in_archive();
}

/// Returns the number of shadows of this measured_bsdf.
usize IMdl_measured_bsdf_info_get_shadows_count(IMdl_measured_bsdf_info* info) {
    return info->get_shadows_count();
}
/// Returns one of the shadows this measured_bsdf has.
/// \param index     Index in the shadow list of this measured_bsdf.
const IMdl_resource_info*
IMdl_measured_bsdf_info_get_shadow(IMdl_measured_bsdf_info* info, usize index) {
    return info->get_shadow(index);
}

/// Returns the extension of the measured_bsdf file
const char* IMdl_measured_bsdf_get_extension(IMdl_measured_bsdf_info* info) {
    return info->get_extension();
}
Uuid IMdl_measured_bsdf_info_type_get_iid() {
    return IMdl_measured_bsdf_info::IID();
}
}
