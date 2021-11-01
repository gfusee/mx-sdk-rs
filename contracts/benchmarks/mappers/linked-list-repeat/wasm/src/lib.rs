////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    linked_list_repeat::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn add() {
    linked_list_repeat::endpoints::add(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn count() {
    linked_list_repeat::endpoints::count(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn remove() {
    linked_list_repeat::endpoints::remove(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getBenchmark() {
    linked_list_repeat::endpoints::getBenchmark(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    linked_list_repeat::endpoints::callBack(elrond_wasm_node::arwen_api());
}