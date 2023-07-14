use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use multiversx_sc::{abi::EndpointAbi, external_view_contract::EXTERNAL_VIEW_CONSTRUCTOR_FLAG};

use super::OutputContract;

const PREFIX_AUTO_GENERATED: &str =
    "// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

";

const NUM_INIT: usize = 1;
const NUM_ASYNC_CB: usize = 1;

const PREFIX_NO_STD: &str = "
#![no_std]
#![feature(lang_items)]

";

impl OutputContract {
    /// Makes sure that all the necessary wasm crate directories exist.
    pub fn create_wasm_crate_dir(&self) {
        fs::create_dir_all(PathBuf::from(&self.wasm_crate_path()).join("src")).unwrap();
    }

    fn allocator_macro_invocation(&self) -> String {
        format!(
            "multiversx_sc_wasm_adapter::allocator!({});",
            self.settings.allocator.to_allocator_macro_selector()
        )
    }

    fn panic_handler_macro_invocation(&self) -> &'static str {
        if self.settings.panic_message {
            "multiversx_sc_wasm_adapter::panic_handler_with_message!();"
        } else {
            "multiversx_sc_wasm_adapter::panic_handler!();"
        }
    }

    fn endpoint_macro_name(&self) -> &'static str {
        if self.settings.external_view {
            "multiversx_sc_wasm_adapter::external_view_endpoints!"
        } else {
            "multiversx_sc_wasm_adapter::endpoints!"
        }
    }

    /// Generates the wasm crate lib.rs source, st the given path.
    pub fn generate_wasm_src_lib_file(&self) {
        let lib_path = format!("{}/src/lib.rs", &self.wasm_crate_path());
        let mut wasm_lib_file = File::create(lib_path).unwrap();
        self.write_wasm_src_lib_contents(&mut wasm_lib_file);
    }

    fn write_wasm_src_lib_contents(&self, wasm_lib_file: &mut File) {
        wasm_lib_file
            .write_all(PREFIX_AUTO_GENERATED.as_bytes())
            .unwrap();
        self.write_stat_comments(wasm_lib_file);
        wasm_lib_file.write_all(PREFIX_NO_STD.as_bytes()).unwrap();

        writeln!(wasm_lib_file, "{}", self.allocator_macro_invocation()).unwrap();
        writeln!(wasm_lib_file, "{}", self.panic_handler_macro_invocation()).unwrap();

        if self.settings.external_view {
            write_external_view_init(wasm_lib_file);
        }

        let contract_module_name = self.abi.get_crate_name_for_code();
        write_endpoints_macro(
            self.endpoint_macro_name(),
            wasm_lib_file,
            &contract_module_name,
            self.abi.iter_all_exports(),
        );

        write_async_callback_macro(wasm_lib_file, self.abi.has_callback, &contract_module_name);
    }
}

fn write_stat_comment(wasm_lib_file: &mut File, label: &str, number: usize) {
    writeln!(wasm_lib_file, "// {label:<35} {number:3}").unwrap();
}

impl OutputContract {
    /// Writing some nicely formatted comments breaking down all exported functions.
    fn write_stat_comments(&self, wasm_lib_file: &mut File) {
        write_stat_comment(wasm_lib_file, "Init:", NUM_INIT);
        write_stat_comment(wasm_lib_file, "Endpoints:", self.abi.endpoints.len());
        if self.abi.has_callback {
            write_stat_comment(wasm_lib_file, "Async Callback:", NUM_ASYNC_CB);
        } else {
            write_stat_comment(wasm_lib_file, "Async Callback (empty):", NUM_ASYNC_CB);
        }
        if !self.abi.promise_callbacks.is_empty() {
            write_stat_comment(
                wasm_lib_file,
                "Promise callbacks:",
                self.abi.promise_callbacks.len(),
            );
        }
        let total =
            self.abi.endpoints.len() + NUM_INIT + NUM_ASYNC_CB + self.abi.promise_callbacks.len();

        write_stat_comment(wasm_lib_file, "Total number of exported functions:", total);
    }
}

fn write_endpoints_macro<'a, I>(
    full_macro_name: &str,
    wasm_lib_file: &mut File,
    contract_module_name: &str,
    endpoint_iter: I,
) where
    I: Iterator<Item = &'a EndpointAbi>,
{
    writeln!(wasm_lib_file).unwrap();
    writeln!(wasm_lib_file, "{full_macro_name} {{").unwrap();
    writeln!(wasm_lib_file, "    {contract_module_name}").unwrap();
    writeln!(wasm_lib_file, "    (").unwrap();
    for endpoint in endpoint_iter {
        if endpoint.rust_method_name == EXTERNAL_VIEW_CONSTRUCTOR_FLAG {
            continue;
        }
        writeln!(
            wasm_lib_file,
            "        {} => {}",
            endpoint.name, endpoint.rust_method_name
        )
        .unwrap();
    }
    writeln!(wasm_lib_file, "    )").unwrap();
    writeln!(wasm_lib_file, "}}").unwrap();
}

fn write_async_callback_macro(
    wasm_lib_file: &mut File,
    has_callback: bool,
    contract_module_name: &str,
) {
    writeln!(wasm_lib_file).unwrap();
    if has_callback {
        writeln!(
            wasm_lib_file,
            "multiversx_sc_wasm_adapter::async_callback! {{ {contract_module_name} }}"
        )
        .unwrap();
    } else {
        writeln!(
            wasm_lib_file,
            "multiversx_sc_wasm_adapter::async_callback_empty! {{}}",
        )
        .unwrap();
    }
}

fn write_external_view_init(wasm_lib_file: &mut File) {
    writeln!(wasm_lib_file).unwrap();
    writeln!(
        wasm_lib_file,
        "multiversx_sc_wasm_adapter::external_view_init! {{}}",
    )
    .unwrap();
}
