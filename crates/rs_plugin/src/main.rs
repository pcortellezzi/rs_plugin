use plugin_api::{PluginRegistrar, PluginGenericType};


fn plugin_process_generic_plugins(plugin_registrar: &PluginRegistrar) {
    let plugins = plugin_registrar.get_plugins::<dyn PluginGenericType>().unwrap();
    for plugin_instance in plugins {
        println!("{}", plugin_instance.name);
        plugin_instance.plugin.use_generic_plugins(plugin_registrar);
    }
}

fn main() {
    let mut plugin_registrar = PluginRegistrar::default();
    let mut exe_dir = std::env::current_exe().unwrap();
    exe_dir.pop();
    plugin_registrar.load_plugins(exe_dir);

    plugin_process_generic_plugins(&plugin_registrar);

    println!("End")
}
