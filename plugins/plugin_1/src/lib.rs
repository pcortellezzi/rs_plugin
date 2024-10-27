use plugin_api::{PluginRegistrar, PluginInstance};
use plugin_type_1::PluginType1;

struct Plugin1 {}

impl PluginType1 for Plugin1 {
    fn capability_one(&self) {
        println!("capability plugin1 type1")
    }
}

#[no_mangle]
pub fn plugin_entry(plugin_registrar: &mut PluginRegistrar) {
    // let plugin1 = Box::new(&Plugin1 {});
    let plugin1: &dyn PluginType1 = &Plugin1 {};
    let plugin_instance1 = PluginInstance {
        name: "Plugin1",
        plugin: plugin1,
    };
    plugin_registrar.register_plugin::<dyn PluginType1>(plugin_instance1);
}