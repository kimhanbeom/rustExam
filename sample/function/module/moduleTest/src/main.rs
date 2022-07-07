use moduleTest::module1::module1_function;
use moduleTest::module2::module2_function;
use moduleTest::module2::submodule::submodule_function;

pub fn main() {
    println!("module test");
    module1_function();
    module2_function();
    submodule_function();
}
