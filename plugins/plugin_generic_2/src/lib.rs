use plugin_api::{PluginRegistrar, PluginInstance, PluginGenericType};
use plugin_type_2::PluginType2;

struct GenericPlugin2 {}

impl PluginGenericType for GenericPlugin2 {
    fn use_generic_plugins(&self, plugin_registrar: &PluginRegistrar) {
        match plugin_registrar.get_plugins::<dyn PluginType2>() {
            Some(plugins) => {
                for plugin_instance in plugins {
                    println!("{}", plugin_instance.name);
                    plugin_instance.plugin.capability_two();
                }
            }
            None => {}
        }
    }
}

#[no_mangle]
pub fn plugin_entry(plugin_registrar: &mut PluginRegistrar) {
    // let generic_plugin1 = Box::new(&GenericPlugin1 {});
    let generic_plugin2: &dyn PluginGenericType = &GenericPlugin2 {};
    let generic_plugin_instance2 = PluginInstance {
        name: "GenericPlugin2",
        plugin: generic_plugin2,
    };
    plugin_registrar.register_plugin::<dyn PluginGenericType>(generic_plugin_instance2);
}