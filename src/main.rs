extern crate libloading;
use libloading::{Symbol, Library};

extern crate shared_traits;
use shared_traits::SharedTrait;

fn load_plugin() -> libloading::Result<Box<SharedTrait>> {
    let lib = try!(Library::new("./libdynamic_plugin.so"));
    unsafe {
        let func: Symbol<unsafe extern fn() -> Box<SharedTrait>> = try!(lib.get(b"create_something\0"));
        println!("Function found.");
        Ok(func())
    }
}

fn main() {
    match load_plugin() {
        Ok(plugin) => println!("Plugin loaded, test result: {}.", plugin.test_function()),
        Err(e) => println!("Failed to load plugin: {}.", e),
    }
}
