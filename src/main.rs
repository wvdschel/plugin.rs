extern crate libloading;
use libloading::{Symbol, Library};

extern crate shared_traits;
use shared_traits::SharedTrait;

struct Plugin {
    lib: Library,
    instance: Box<SharedTrait>,
}

fn load_plugin() -> libloading::Result<Plugin> {
    let lib = try!(Library::new("./libdynamic_plugin.so"));
    let mut instance: Option<Box<SharedTrait>> = None;
    unsafe {
        let func: Symbol<unsafe extern fn() -> Box<SharedTrait>> = lib.get(b"create_something\0").unwrap();
        instance = Some(func());
    }
    Ok(Plugin {
        lib: lib,
        instance: instance.unwrap(),
    })
}

fn main() {
    match load_plugin() {
        Ok(plugin) => println!("Plugin loaded, test result: {}.", plugin.instance.test_function()),
        Err(e) => println!("Failed to load plugin: {}.", e),
    }
}
