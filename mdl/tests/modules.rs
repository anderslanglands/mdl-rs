use mdl::*;

type Result<T, E = mdl::Error> = std::result::Result<T, E>;

use std::path::Path;

#[test]
fn modules() -> Result<()> {
    let mut neuray = NEURAY.lock().unwrap();

    // configure MDL by settings module searchpaths and loading plugins
    configure(&neuray)?;

    // start the SDK
    neuray.start()?;

    // load a module and dump its results
    load_module(&mut neuray)?;

    Ok(())
}

fn configure(neuray: &Neuray) -> Result<()> {
    let mut mdl_compiler = neuray.get_api_component_mdl_compiler()?;

    // set the module and texture search paths
    let mdl_samples_root = Path::new(&std::env::var("MDL_ROOT").unwrap())
        .join("examples")
        .join("mdl");
    mdl_compiler.add_module_path(mdl_samples_root)?;

    // load the freeimage plugin
    mdl_compiler.load_plugin_library("nv_freeimage.so")?;

    Ok(())
}

fn load_module(neuray: &mut Neuray) -> Result<()> {
    let database = neuray.get_api_component_database()?;
    let mut scope = database.get_global_scope()?;
    let transaction = scope.create_transaction();
    {
        let mdl_compiler = neuray.get_api_component_mdl_compiler()?;
        let mdl_factory = neuray.get_api_component_mdl_factory()?;

        let execution_context = mdl_factory.create_execution_context();

        // load the module 'tutorials'
        mdl_compiler.load_module(
            &transaction,
            "::nvidia::sdk_examples::tutorials",
            &execution_context,
        )?;

        // Access the module by its name. The name to be used here is the MDL
        // name of the module ("example") plus the "mdl::" prefix.
        let module = transaction
            .access::<Module>("mdl::nvidia::sdk_examples::tutorials")
            .expect("Could not access examples module");

        // Dump imported modules
        println!("Loaded file '{}'", module.get_filename());
        println!("Found module '{}'", module.get_mdl_name());
        println!("## The module imports the following modules:");
        for import in module.imports() {
            println!("    {}", import);
        }

        // Dump exported types
        let type_factory = mdl_factory.create_type_factory(&transaction);
        let type_list = module.get_types();
        println!("## The module contains the following types:");
        for t in type_list.iter() {
            println!("    {}", type_factory.dump(&t, 1).unwrap());
        }

        // Dump exported constants
        let value_factory = mdl_factory.create_value_factory(&transaction);
        let constants = module.get_constants();
        println!("## The module contains the following constants:");
        for c in constants.iter() {
            println!("    {}", value_factory.dump(&c, None, 1).unwrap());
        }

        // Dump function definitions
        println!("## The module contains the following function definitions:");
        for f in module.functions() {
            println!("    {}", f);
        }

        // Dump material definitions
        println!("## The module contains the following material definitions:");
        for f in module.materials() {
            println!("    {}", f);
        }

        // Dump a function definition from the module
        let function_name = module.get_function(0);
        println!("## Dumping function definition '{}'", function_name);

        let function_definition = transaction.access::<FunctionDefinition>(&function_name)?;
        dump_definition(&transaction, &mdl_factory, &function_definition, 1)?;

        // Dump a material definition from the module
        let material_name = module.get_material(0);
        println!("## Dumping material definition '{}'", material_name);

        let material_definition = transaction.access::<MaterialDefinition>(&material_name)?;
        dump_definition(&transaction, &mdl_factory, &material_definition, 1)?;

        // dump resources
        println!("## Dumping resources of this module:");
        for res in module.resources() {
            if let Some(db_name) = res.name() {
                println!("    DB name      : '{}'", db_name);
                println!("    MDL file path: '{}'", res.mdl_file_path().unwrap());

                let res_type = res.get_type().unwrap();
                let kind = res_type.get_kind();
                println!("    kind: {:?}", kind);

            } else {
                println!("    The module contains a resource that could not be resolved");
                println!("    MDL file path: '{}'", res.mdl_file_path().unwrap());
            }
        }
    }

    // all transactions need to be committed
    transaction.commit()?;
    Ok(())
}

fn dump_definition<D: Definition>(
    transaction: &Transaction,
    mdl_factory: &MdlFactory,
    definition: &D,
    depth: usize,
) -> Result<()> {
    let type_factory = mdl_factory.create_type_factory(&transaction);
    let expression_factory = mdl_factory.create_expression_factory(&transaction);

    let count = definition.get_parameter_count();
    let type_list = definition.get_parameter_types();
    let defaults = definition.get_defaults();

    for i in 0..count {
        let typ = type_list.get_type(i).unwrap();
        let type_text = type_factory.dump(&typ, depth + 1).unwrap();
        let name = definition.get_parameter_name(i).unwrap();

        if let Some(expr_default) = defaults.get_expression_by_name(&name) {
            let default_text = expression_factory
                .dump(&expr_default, None, depth + 1)
                .unwrap();
            println!("    parameter {} {} = {}", type_text, name, default_text);
        } else {
            println!("    parameter {} {} = <no default>", type_text, name);
        }
    }

    Ok(())
}
