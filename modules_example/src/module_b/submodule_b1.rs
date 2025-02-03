// Importing function from sub_submodule_c1_1 (different parenting tree)
use crate::module_c::submodule_c1;

pub fn some_function_b1() {
    println!("Function in submodule_b1");
    // Call function from sub_submodule_c1_1
    submodule_c1::sub_submodule_c1_1::some_function_c1_1();
}
