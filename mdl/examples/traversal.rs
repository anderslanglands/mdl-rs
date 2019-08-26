use mdl::base::Interface;
use mdl::*;
type Result<T, E = mdl::Error> = std::result::Result<T, E>;

use std::collections::HashSet;

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

            for material_name in module.materials() {
                println!("Accessing material '{}'", material_name);
                let material = transaction
                    .access::<MaterialDefinition>(&material_name)
                    .unwrap();

                let definition_wrapper = DefinitionWrapper::new(
                    &transaction,
                    &material_name,
                    &factory,
                )
                .unwrap();

                let material_instance_se =
                    match definition_wrapper.create_instance(None) {
                        Ok(se) => se,
                        Err(e) => {
                            println!(
                                "Failed to create material instance of {}",
                                material_name
                            );
                            continue;
                        }
                    };

                let material_instance =
                    MaterialInstance::from_interface(material_instance_se);

                // compile the material instance
                let compiled_material = material_instance
                    .create_compiled_material(
                        CompilationOptions::CLASS_COMPILATION,
                        None,
                    );

                let compiled_material = match compiled_material {
                    Some(m) => m,
                    None => {
                        println!("Could not create compiled instance from material '{}'", material_name);
                        continue;
                    }
                };

                let mut context = Context {
                    transaction: &transaction,
                    compiler: &mdl_compiler,
                    indent: 0,
                    imports: HashSet::new(),
                    used_modules: HashSet::new(),
                    used_resources: HashSet::new(),
                };

                traverse(&compiled_material, &mut context);
            }
        }
    }
}

pub struct Context<'a> {
    pub transaction: &'a Transaction,
    pub compiler: &'a MdlCompiler,
    pub indent: usize,
    pub imports: HashSet<String>,
    pub used_modules: HashSet<String>,
    pub used_resources: HashSet<String>,
}

pub enum Element {
    Parameter(Value),
    Value(Value),
    Expression(Expression),
    Temporary(Expression),
}

pub struct TraversalElement {
    element: Element,
    sibling_count: usize,
    sibling_index: usize,
}

pub fn traverse(material: &CompiledMaterial, context: &mut Context) {
    let body = material.get_body();

    let param_count = material.get_parameter_count();
    println!("// BEGIN parameters");
    for i in 0..param_count {
        if let Some(arg) = material.get_argument(i) {
            let tel = TraversalElement {
                element: Element::Parameter(arg),
                sibling_count: 1,
                sibling_index: 0,
            };
            if let Some(name) = material.get_parameter_name(i) {
                println!("Traversing argument {}: {}", i, name);
            } else {
                println!("Traversing argument {}", i);
            }
            traverse_element(material, tel, context);
        }
    }
    println!("// END parameters");

    let temp_count = material.get_temporary_count();
    println!("// BEGIN temporarys");
    for i in 0..temp_count {
        if let Some(arg) = material.get_temporary(i) {
            let tel = TraversalElement {
                element: Element::Temporary(arg),
                sibling_count: 1,
                sibling_index: 0,
            };
            println!("Traversing temporary {}", i);
            traverse_element(material, tel, context);
        }
    }
    println!("// END temporarys");

    println!("// BEGIN body");
    traverse_element(
        material,
        TraversalElement {
            element: Element::Expression(Expression::from_interface(body)),
            sibling_count: 1,
            sibling_index: 0,
        },
        context,
    );
    println!("// END body");

}

pub fn traverse_element(
    material: &CompiledMaterial,
    element: TraversalElement,
    context: &mut Context,
) {
    context.indent += 1;

    match element.element {
        Element::Expression(expression) => match expression.get_kind() {
            ExpressionKind::Constant => {
                let expr_const = ExpressionConstant::from_interface(expression);
                let value = expr_const.get_value();
                traverse_element(
                    material,
                    TraversalElement {
                        element: Element::Value(value),
                        sibling_count: 1,
                        sibling_index: 0,
                    },
                    context,
                );
            }
            ExpressionKind::DirectCall => {
                let expr_dcall =
                    ExpressionDirectCall::from_interface(expression);
                println!(
                    "{:indent$}{}",
                    "",
                    expr_dcall.get_definition(),
                    indent = context.indent * 4
                );
                let func_def = FunctionDefinition::from_interface(
                    context
                        .transaction
                        .access::<FunctionDefinition>(
                            &expr_dcall.get_definition(),
                        )
                        .unwrap(),
                );
                let func_name = func_def.get_mdl_name();
                let semantic = func_def.get_semantic();
                println!(
                    "{:indent$}{:?}",
                    "",
                    semantic,
                    indent = context.indent * 4
                );

                let arguments = expr_dcall.get_arguments();
                for i in 0..arguments.get_size() {
                    let expr = arguments.get_expression(i).unwrap();
                    // visit child...
                    traverse_element(
                        material,
                        TraversalElement {
                            element: Element::Expression(expr),
                            sibling_count: arguments.get_size(),
                            sibling_index: i,
                        },
                        context,
                    );
                }
            }
            ExpressionKind::Parameter => {
                // print parameter name
                let expr_param =
                    ExpressionParameter::from_interface(expression);
                let index = expr_param.get_index();
                let name = material.get_parameter_name(index).unwrap();
                println!(
                    "{:indent$}{} //< param",
                    "",
                    name,
                    indent = context.indent * 4
                );
            }
            ExpressionKind::Temporary => {
                let expr_temp = ExpressionTemporary::from_interface(expression);
                let index = expr_temp.get_index();
                println!(
                    "{:indent$}temporary_{}",
                    "",
                    index,
                    indent = context.indent * 4
                );
            }
            _ => (),
        },
        Element::Value(value) => {
            match value.get_kind() {
                ValueKind::Vector
                | ValueKind::Matrix
                | ValueKind::Color
                | ValueKind::Array
                | ValueKind::Struct => {
                    let value_compound = ValueCompound::from_interface(value);
                    let size = value_compound.get_size();
                    for i in 0..size {
                        let compound_element =
                            value_compound.get_value(i).unwrap();
                        // visit child...
                        traverse_element(
                            material,
                            TraversalElement {
                                element: Element::Value(compound_element),
                                sibling_count: size,
                                sibling_index: i,
                            },
                            context,
                        );
                    }
                }
                _ => (), // nothhing to do for the others
            }
        }
        Element::Parameter(parameter) => {
            traverse_element(
                material,
                TraversalElement {
                    element: Element::Value(parameter),
                    sibling_count: 1,
                    sibling_index: 0,
                },
                context,
            );
        }
        Element::Temporary(expression) => {
            traverse_element(
                material,
                TraversalElement {
                    element: Element::Expression(expression),
                    sibling_count: 1,
                    sibling_index: 0,
                },
                context,
            );
        }
    }

    context.indent -= 1;
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
