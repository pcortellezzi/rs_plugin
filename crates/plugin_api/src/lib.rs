use std::any::{Any, TypeId};
use std::collections::HashMap;
use libloading::Library;
use regex::Regex;

#[derive(Default)]
pub struct PluginRegistrar {
    plugin_instances: HashMap<TypeId, Box<dyn Any>>,
}

impl<'a> PluginRegistrar {
    pub fn register_plugin<PluginType: ?Sized>(&mut self, plugin_instance: PluginInstance<PluginType>) {
        let type_id = TypeId::of::<PluginType>();
        // let plugin: Box<dyn Any> = Box::new(plugin);
        match self.plugin_instances.get_mut(&type_id) {
            Some(plugin_instances) => {
                let plugin_instances = plugin_instances.downcast_mut::<Vec<PluginInstance<PluginType>>>().unwrap();
                plugin_instances.push(plugin_instance);
            }
            None => {
                self.plugin_instances
                    .insert(type_id, Box::new(vec![plugin_instance]));
            }
        };
    }

    pub fn get_plugins<PluginType: ?Sized + 'static>(&self) -> Option<&Vec<PluginInstance<PluginType>>> {
        // self.plugin_instances.get(&TypeId::of::<PluginType>()).unwrap()
        let type_id = TypeId::of::<PluginType>();
        match self.plugin_instances.get(&type_id) {
            Some(plugin_instances) => {
                plugin_instances.downcast_ref::<Vec<PluginInstance<PluginType>>>()
            }
            None => None
        }
    }

    pub fn load_plugins(&mut self, path: std::path::PathBuf) {
        let pattern = r"libplugin.*.so$";
        let expression = Regex::new(&pattern).unwrap();
        for entry in std::fs::read_dir(path).unwrap() {
            // NOTE: You need to do something to ensure you're only loading "safe" code. Out of scope
            // for this code.
            let path = entry.unwrap().path();
            let path_str = path.to_str().unwrap();
            //let filename = entry.unwrap().file_name().to_str().unwrap();
            if expression.is_match(path_str) {
                unsafe {
                    let lib = Box::leak(Box::new(Library::new(path.to_str().unwrap()).unwrap()));
                    // In this code, we never close the shared library - if you need to be able to unload the
                    // library, that will require more work.
                    let func: libloading::Symbol<unsafe extern "C" fn(&mut PluginRegistrar) -> ()> =
                        lib.get(b"plugin_entry").unwrap();
                    func(self);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PluginInstance<PluginType: ?Sized + 'static> {
    pub name: &'static str,
    pub plugin: &'static PluginType,
}

pub trait PluginGenericType:  {
    fn use_generic_plugins(&self, plugin_registrar: &PluginRegistrar);
}