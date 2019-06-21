use mdl::base::Interface;
use mdl::*;
type Result<T, E = mdl::Error> = std::result::Result<T, E>;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "m", long = "mdl-path")]
    mdl_paths: Vec<String>,
    module_name: String,
}

pub fn main() {
    let mut opt = Opt::from_args();
    println!("{:?}", opt);

    // add env paths
    let mut env_paths: Vec<String> =
        if let Ok(paths) = std::env::var("MDL_MODULE_PATH") {
            std::env::split_paths(&paths)
                .map(|p| (*p).to_str().unwrap().to_string())
                .collect()
        } else {
            Vec::new()
        };
    opt.mdl_paths.append(&mut env_paths);

    let mut api = NEURAY.lock().unwrap();

    // configure MDL by settings module searchpaths and loading plugins
    configure(&api, &opt.mdl_paths).expect("Failed to add paths to compiler");
    api.start().expect("Failed to start neuray");

    {
        let database = api.get_api_component::<Database>().unwrap();
        let mut scope = database.get_global_scope().unwrap();
        let transaction = scope.create_transaction();

        let factory = api.get_api_component::<MdlFactory>().unwrap();
        {
            let mdl_compiler = api.get_api_component::<MdlCompiler>().unwrap();
            let ctx = factory.create_execution_context();

            match mdl_compiler.load_module(&transaction, &opt.module_name, &ctx)
            {
                Ok(()) => (),
                Err(e) => {
                    println!(
                        "Could not load module '{}': {}",
                        opt.module_name, e
                    );
                    for msg in ctx.error_messages() {
                        println!("{}", msg.get_string().unwrap());
                        for note in msg.notes() {
                            println!("    {}", note.get_string().unwrap());
                        }
                    }
                    return;
                }
            }

            let module = transaction
                .access::<Module>(&format!("mdl{}", opt.module_name))
                .unwrap();

            for material in module.materials() {
                println!("Accessing material '{}'", material);
                let material = transaction
                    .access::<MaterialDefinition>(&material)
                    .unwrap();
            }
        }
    }
}

fn configure(neuray: &Neuray, paths: &Vec<String>) -> Result<()> {
    let mut mdl_compiler = neuray.get_api_component::<MdlCompiler>()?;

    for path in paths {
        mdl_compiler.add_module_path(path)?;
    }

    // load the freeimage plugin
    mdl_compiler.load_plugin_library("nv_freeimage.so")?;

    Ok(())
}
