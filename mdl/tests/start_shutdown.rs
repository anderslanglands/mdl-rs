use mdl::*;

type Result<T, E = mdl::Error> = std::result::Result<T, E>;

#[test]
fn start_shutdown() -> Result<()> {
    let mut neuray = NEURAY.lock().unwrap();

    // No need for is_valid_interface() here as Neuray can never contain a 
    // null ptr
    neuray.start()?;

    let version = neuray.get_api_component_version().expect("Could not get version component");

    println!("MDL product name:    {}", version.get_product_name());
    println!("MDL product version: {}", version.get_product_version());
    println!("MDL build number:    {}", version.get_build_number());
    println!("MDL build date:      {}", version.get_build_date());
    println!("MDL build platform:  {}", version.get_build_platform());
    println!("MDL version string:  {}", version.get_string());
    println!("MDL neuray UUID:     {}", version.get_neuray_iid());

    Ok(())
}
