use plugin_api::{PluginRegistrar, PluginInstance};
use plugin_type_2::PluginType2;

struct Plugin2 {}

impl PluginType2 for Plugin2 {
    fn capability_two(&self) {
        println!("capability plugin2 type2")
    }
}

#[no_mangle]
pub fn plugin_entry(plugin_registrar: &mut PluginRegistrar) {
    // let plugin2 = Box::new(&Plugin2 {});
    let plugin2: &dyn PluginType2 = &Plugin2 {};
    let plugin_instance2 = PluginInstance {
        name: "Plugin2",
        plugin: plugin2,
    };
    plugin_registrar.register_plugin::<dyn PluginType2>(plugin_instance2);
}
