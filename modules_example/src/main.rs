// Declare all the modules
mod module_a;
mod module_b;
mod module_c;

fn main() {
    // Call functions from various modules and submodules
    module_a::some_function_a();
    module_b::submodule_b1::some_function_b1();
    module_b::submodule_b2::some_function_b2();
}

/*

module_hierarchy/
    ├── Cargo.toml
    ├── src/
        ├── main.rs
        ├── module_a.rs
        ├── module_b/
        │   ├── mod.rs
        │   ├── submodule_b1.rs
        │   ├── submodule_b2.rs
        ├── module_c/
            ├── mod.rs
            ├── submodule_c1/
                ├── mod.rs
                ├── sub_submodule_c1_1.rs

 */