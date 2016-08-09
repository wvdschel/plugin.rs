extern crate shared_traits;
use shared_traits::SharedTrait;

struct ExampleTraitImpl {}

impl SharedTrait for ExampleTraitImpl {
    fn test_function(&self) -> i32 { 10 }
}

#[no_mangle]
pub extern fn create_something() -> Box<SharedTrait> {
    Box::new(ExampleTraitImpl {})
}
