use mdl::base::Interface;
use mdl::*;
type Result<T, E = mdl::Error> = std::result::Result<T, E>;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "m", long = "mdl-path")]
    mdl_paths: Vec<String>,
    #[structopt(short = "f", long = "filter")]
    filters: Vec<String>,
}
pub fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let filter = MdlInfoKind::All;

    let mut neuray = NEURAY.lock().unwrap();

    // configure MDL by settings module searchpaths and loading plugins
    configure(&neuray, &opt.mdl_paths)
        .expect("Failed to add paths to compiler");
    neuray.start().expect("Failed to start neuray");

    {
        let discovery_api = neuray
            .get_api_component::<MdlDiscoveryApi>()
            .expect("Could not get Discovery api");

        let disc_result = discovery_api
            .discover(filter)
            .expect("Could not get discorvery result");

        let root = disc_result.get_graph().expect("Could not get package info");

        let num = disc_result.get_search_paths_count();
        if num > 1 {
            println!("s: ");
        } else {
            println!(": ");
        }

        for i in 0..num {
            println!("{}", disc_result.get_search_path(i).unwrap());
        }

        println!("---------------- MDL GRAPH ----------------");
        log_api_package(&root, 0);
        println!("----------------\\ MDL GRAPH ---------------");
    }
}

fn configure(neuray: &Neuray, paths: &Vec<String>) -> Result<()> {
    let mut mdl_compiler = neuray.get_api_component::<MdlCompiler>()?;

    for path in paths {
        mdl_compiler.add_module_path(path)?;
    }

    Ok(())
}

fn log_api_package<I: MdlInfo>(info: &I, level: usize) {
    println!(
        "{:indent$}simple name   : {}",
        "",
        info.get_simple_name().unwrap(),
        indent = level * 4,
    );
    println!(
        "{:indent$}qualified name: {}",
        "",
        info.get_qualified_name().unwrap(),
        indent = level * 4,
    );
    println!(
        "{:indent$}kind          : {:?}",
        "",
        info.get_kind(),
        indent = level * 4,
    );

    match info.get_kind() {
        MdlInfoKind::Xliff => {
            let info = MdlXliffInfo::from_interface(info);
            log_default_attributes(
                level,
                info.get_search_path_index(),
                &info.get_search_path().unwrap(),
                &info.get_resolved_path().unwrap(),
                info.in_archive(),
            );
        }
        MdlInfoKind::Texture => {
            let info = MdlTextureInfo::from_interface(info);
            log_default_attributes(
                level,
                info.get_search_path_index(),
                &info.get_search_path().unwrap(),
                &info.get_resolved_path().unwrap(),
                info.in_archive(),
            );
        }
        MdlInfoKind::LightProfile => {
            let info = MdlLightProfileInfo::from_interface(info);
            log_default_attributes(
                level,
                info.get_search_path_index(),
                &info.get_search_path().unwrap(),
                &info.get_resolved_path().unwrap(),
                info.in_archive(),
            );
        }
        MdlInfoKind::MeasuredBsdf => {
            let info = MdlMeasuredBsdfInfo::from_interface(info);
            log_default_attributes(
                level,
                info.get_search_path_index(),
                &info.get_search_path().unwrap(),
                &info.get_resolved_path().unwrap(),
                info.in_archive(),
            );
        }
        MdlInfoKind::Module => {
            let info = MdlModuleInfo::from_interface(info);
            log_default_attributes(
                level,
                info.get_search_path_index(),
                &info.get_search_path().unwrap(),
                &info.get_resolved_path().unwrap(),
                info.in_archive(),
            );
        }
        MdlInfoKind::Package | MdlInfoKind::Directory => {
            let info = MdlPackageInfo::from_interface(info);
            let spi_count = info.get_search_path_index_count();
            if spi_count > 0 {
                println!(
                    "{:indent$}discovered in {} search paths:",
                    "",
                    spi_count,
                    indent = level * 4,
                );
            }
            for i in 0..spi_count {
                log_default_attributes(
                    level,
                    info.get_search_path_index(i),
                    &info.get_search_path(i).unwrap(),
                    &info.get_resolved_path(i).unwrap(),
                    info.in_archive(i),
                );
            }

            // recursively iterate over all subpackages or modules
            let child_count = info.get_child_count();
            println!(
                "{:indent$}number of children: {}",
                "",
                child_count,
                indent = level * 4,
            );

            for c in 0..child_count {
                let child =
                    MdlInfoBase::from_interface(info.get_child(c).unwrap());
                log_api_package(&child, level + 1);
            }
        }
        _ => (),
    }
}

fn log_default_attributes(
    level: usize,
    search_path_index: usize,
    search_path: &str,
    resolved_path: &str,
    found_in_archive: bool,
) {
    println!(
        "{:indent$}search path index: {}",
        "",
        search_path_index,
        indent = level * 4,
    );
    println!(
        "{:indent$}search path      : {}",
        "",
        search_path,
        indent = level * 4,
    );
    println!(
        "{:indent$}resolved path    : {}",
        "",
        resolved_path,
        indent = level * 4,
    );
    println!(
        "{:indent$}found in archive : {}",
        "",
        found_in_archive,
        indent = level * 4,
    );

}