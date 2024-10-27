use plugin_api::{PluginRegistrar, PluginInstance, PluginGenericType};
use plugin_type_1::PluginType1;

struct GenericPlugin1 {}

impl PluginGenericType for GenericPlugin1 {
    fn use_generic_plugins(&self, plugin_registrar: &PluginRegistrar) {
        match plugin_registrar.get_plugins::<dyn PluginType1>() {
            Some(plugins) => {
                for plugin_instance in plugins {
                    println!("{}", plugin_instance.name);
                    plugin_instance.plugin.capability_one();
                }
            }
            None => {}
        }
    }
}

#[no_mangle]
pub fn plugin_entry(plugin_registrar: &mut PluginRegistrar) {
    // let generic_plugin1 = Box::new(&GenericPlugin1 {});
    let generic_plugin1: &dyn PluginGenericType = &GenericPlugin1 {};
    let generic_plugin_instance1 = PluginInstance {
        name: "GenericPlugin1",
        plugin: generic_plugin1,
    };
    plugin_registrar.register_plugin::<dyn PluginGenericType>(generic_plugin_instance1);
}