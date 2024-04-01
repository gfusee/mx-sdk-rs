use std::{fs::File, io::Write};

use multiversx_sc::abi::{
    EndpointAbi, EnumVariantDescription, InputAbi, OutputAbi, StructFieldDescription, TypeContents,
    TypeDescription,
};

use crate::cmd::contract::meta_config::MetaConfig;

use super::proxy_naming::{extract_struct_crate, proxy_methods_type_name, proxy_type_name};

const PRELUDE: &str = "// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;";

const ZERO: &str = "0";

/// Types defined in the framework don't need to be generated again in the proxy.
const TYPES_FROM_FRAMEWORK: &[&str] = &[
    "EsdtTokenPayment",
    "EgldOrEsdtTokenPayment",
    "EsdtTokenData",
    "EgldOrEsdtTokenIdentifier",
    "EgldOrEsdtTokenPayment",
    "EgldOrMultiEsdtPayment",
    "EsdtTokenData",
    "EsdtLocalRole",
];

pub struct ProxyGenerator<'a> {
    pub meta_config: &'a MetaConfig,
    pub file: &'a mut File,
}
impl<'a> ProxyGenerator<'a> {
    pub fn new(meta_config: &'a MetaConfig, file: &'a mut File) -> Self {
        Self { meta_config, file }
    }

    pub fn write_proxy_to_file(&mut self) {
        self.write_header();
        self.write_tx_proxy_type_def();
        self.write_impl_for_tx_proxy();
        self.write_struct_tx_proxy_methods();
        self.write_content();
        self.write_types();
    }

    fn write_header(&mut self) {
        writeln!(self.file, r#"{PRELUDE}"#).unwrap();
    }

    fn write_tx_proxy_type_def(&mut self) {
        let proxy_type_name = proxy_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            self.file,
            r#"
pub struct {proxy_type_name};"#
        )
        .unwrap();
    }

    fn write_impl_for_tx_proxy(&mut self) {
        let proxy_type_name = proxy_type_name(&self.meta_config.original_contract_abi.name);
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            self.file,
            r#"
impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for {proxy_type_name}
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    type TxProxyMethods = {proxy_methods_type_name}<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {{
        {proxy_methods_type_name} {{ wrapped_tx: tx }}
    }}
}}"#
        )
        .unwrap();
    }

    fn write_struct_tx_proxy_methods(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            self.file,
            r#"
pub struct {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}}"#
        )
        .unwrap();
    }

    fn write_content(&mut self) {
        if !self
            .meta_config
            .original_contract_abi
            .constructors
            .is_empty()
        {
            self.write_constructors();
        }

        if !self
            .meta_config
            .original_contract_abi
            .upgrade_constructors
            .is_empty()
        {
            self.write_upgrades();
        }

        if !self.meta_config.original_contract_abi.endpoints.is_empty() {
            self.write_endpoints();
        }
    }

    fn write_types(&mut self) {
        for (_, type_description) in &self.meta_config.original_contract_abi.type_descriptions.0 {
            if self
                .meta_config
                .original_contract_abi
                .build_info
                .contract_crate
                .name
                != extract_struct_crate(type_description.names.rust.as_str())
            {
                continue;
            }

            let type_name = self.adjust_type_name_with_api(&type_description.names.rust);
            if TYPES_FROM_FRAMEWORK.contains(&type_name.as_str()) {
                continue;
            }

            match &type_description.contents {
                TypeContents::Enum(enum_variants) => {
                    self.write_enum(enum_variants, type_description, &type_name)
                },
                TypeContents::Struct(struct_fields) => {
                    self.write_struct(struct_fields, type_description, &type_name)
                },
                TypeContents::NotSpecified => {},
                TypeContents::ExplicitEnum(_) => {},
            }
        }
    }

    fn write_constructors(&mut self) {
        let constructors: Vec<EndpointAbi> =
            self.meta_config.original_contract_abi.constructors.clone();

        self.write_header_impl_constructor();
        for (i, constructor_abi) in constructors.into_iter().enumerate() {
            if i > 0 {
                writeln!(self.file).unwrap();
            }
            self.write_constructor_header(&constructor_abi);
            self.write_constructor_content(constructor_abi.inputs);
            self.write_end_of_function();
        }

        writeln!(self.file, "}}").unwrap();
    }

    fn write_upgrades(&mut self) {
        self.write_header_impl_upgrade();
        for (i, upgrade) in self
            .meta_config
            .original_contract_abi
            .upgrade_constructors
            .clone()
            .into_iter()
            .enumerate()
        {
            if i > 0 {
                writeln!(self.file).unwrap();
            }
            self.write_upgrade_header(&upgrade);
            self.write_upgrade_content(upgrade.inputs);
            self.write_end_of_function();
        }

        writeln!(self.file, "}}").unwrap();
    }

    fn write_endpoints(&mut self) {
        let endpoints: Vec<EndpointAbi> = self.meta_config.original_contract_abi.endpoints.clone();

        self.write_header_impl_endpoints();
        for (i, endpoint_abi) in endpoints.into_iter().enumerate() {
            if i > 0 {
                writeln!(self.file).unwrap();
            }
            self.write_endpoint_header(&endpoint_abi);
            self.write_endpoint_content(&endpoint_abi);
            self.write_end_of_function();
        }

        writeln!(self.file, "}}").unwrap();
    }

    fn write_header_impl_constructor(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            self.file,
            r#"
#[rustfmt::skip]
impl<Env, From, Gas> {proxy_methods_type_name}<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{{"#
        )
        .unwrap();
    }

    fn write_header_impl_upgrade(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            self.file,
            r#"
#[rustfmt::skip]
impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{"#
        )
        .unwrap();
    }

    fn write_header_impl_endpoints(&mut self) {
        let proxy_methods_type_name =
            proxy_methods_type_name(&self.meta_config.original_contract_abi.name);
        writeln!(
            self.file,
            r#"
#[rustfmt::skip]
impl<Env, From, To, Gas> {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{"#
        )
        .unwrap();
    }

    fn write_constructor_header(&mut self, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(constructor_abi);
        self.write_constructor_output(&constructor_abi.outputs);
    }

    fn write_upgrade_header(&mut self, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(constructor_abi);
        self.write_upgrade_output(&constructor_abi.outputs);
    }

    fn write_endpoint_header(&mut self, constructor_abi: &EndpointAbi) {
        self.write_fn_signature(constructor_abi);
        self.write_endpoint_output(&constructor_abi.outputs);
    }

    fn write_constructor_content(&mut self, inputs: Vec<InputAbi>) {
        writeln!(
            self.file,
            "        self.wrapped_tx
            .raw_deploy()"
        )
        .unwrap();
        for input in inputs.iter() {
            writeln!(self.file, "            .argument(&{})", input.arg_name).unwrap();
        }
        writeln!(self.file, "            .original_result()").unwrap();
    }

    fn write_upgrade_content(&mut self, inputs: Vec<InputAbi>) {
        writeln!(
            self.file,
            "        self.wrapped_tx
            .raw_upgrade()"
        )
        .unwrap();
        for input in inputs.iter() {
            writeln!(self.file, "            .argument(&{})", input.arg_name).unwrap();
        }
        writeln!(self.file, "            .original_result()").unwrap();
    }

    fn write_endpoint_content(&mut self, endpoint: &EndpointAbi) {
        writeln!(
            self.file,
            "        self.wrapped_tx
            .raw_call()
            .function_name(\"{}\")",
            endpoint.name
        )
        .unwrap();

        for input in endpoint.inputs.iter() {
            writeln!(self.file, "            .argument(&{})", input.arg_name).unwrap();
        }

        writeln!(self.file, "            .original_result()").unwrap();
    }

    fn write_fn_signature(&mut self, endpoint: &EndpointAbi) {
        self.write_endpoint_docs(&endpoint.docs);
        self.write_function_header_endpoint(&endpoint.rust_method_name);
        self.write_args(&endpoint.inputs);
        self.write_parameters(&endpoint.inputs);
    }

    fn write_endpoint_docs(&mut self, docs: &Vec<String>) {
        for doc in docs {
            writeln!(self.file, "    /// {doc} ").unwrap();
        }
    }

    fn write_function_header_endpoint(&mut self, rust_method_name: &String) {
        write!(self.file, "    pub fn {rust_method_name}").unwrap();
    }

    fn write_args(&mut self, inputs: &[InputAbi]) {
        if inputs.is_empty() {
            return;
        }

        writeln!(self.file, "<").unwrap();

        for (index, input) in inputs.iter().enumerate() {
            self.write_argument(index, &input.type_names.rust);
        }

        write!(self.file, "    >").unwrap();
    }

    fn write_argument(&mut self, index: usize, rust_name: &str) {
        let adjusted = self.adjust_type_name_with_env_api(rust_name);
        writeln!(self.file, "        Arg{index}: CodecInto<{adjusted}>,").unwrap();
    }

    fn write_parameters(&mut self, inputs: &[InputAbi]) {
        writeln!(self.file, "(").unwrap();
        writeln!(self.file, "        self,").unwrap();
        for (index, input) in inputs.iter().enumerate() {
            writeln!(self.file, "        {}: Arg{index},", &input.arg_name).unwrap();
        }
        write!(self.file, "    ) ").unwrap();
    }

    fn write_constructor_output(&mut self, outputs: &[OutputAbi]) {
        write!(self.file, "-> TxProxyDeploy<Env, From, Gas, ").unwrap();

        self.parse_and_write_outputs(outputs);

        writeln!(self.file, "> {{").unwrap();
    }

    fn write_upgrade_output(&mut self, outputs: &[OutputAbi]) {
        write!(self.file, "-> TxProxyUpgrade<Env, From, To, Gas, ").unwrap();

        self.parse_and_write_outputs(outputs);

        writeln!(self.file, "> {{").unwrap();
    }

    fn write_endpoint_output(&mut self, outputs: &[OutputAbi]) {
        write!(self.file, "-> TxProxyCall<Env, From, To, Gas, ").unwrap();

        self.parse_and_write_outputs(outputs);

        writeln!(self.file, "> {{").unwrap();
    }

    fn parse_and_write_outputs(&mut self, outputs: &[OutputAbi]) {
        match outputs.len() {
            0 => {
                write!(self.file, "()").unwrap();
            },
            1 => {
                let adjusted = self.adjust_type_name_with_env_api(&outputs[0].type_names.rust);
                write!(self.file, "{adjusted}").unwrap();
            },
            _ => {
                write!(self.file, "MultiValue{}<", outputs.len()).unwrap();
                for (i, output) in outputs.iter().enumerate() {
                    if i > 0 {
                        write!(self.file, ", ").unwrap();
                    }
                    let adjusted = self.adjust_type_name_with_env_api(&output.type_names.rust);
                    write!(self.file, "{adjusted}").unwrap();
                }
                write!(self.file, ">").unwrap();
            },
        }
    }

    fn write_enum(
        &mut self,
        enum_variants: &Vec<EnumVariantDescription>,
        type_description: &TypeDescription,
        name: &str,
    ) {
        self.start_write_type("enum", type_description, name);

        for variant in enum_variants {
            write!(self.file, "    {}", variant.name).unwrap();
            if variant.fields.is_empty() {
                writeln!(self.file, ",").unwrap();
                continue;
            }

            if variant.fields[0].name == ZERO {
                self.write_tuple_in_variant(&variant.fields);
            } else {
                self.write_struct_in_variant(&variant.fields);
            }
        }
        writeln!(self.file, "}}").unwrap();
    }

    fn write_struct(
        &mut self,
        struct_fields: &Vec<StructFieldDescription>,
        type_description: &TypeDescription,
        name: &str,
    ) {
        self.start_write_type("struct", type_description, name);

        for field in struct_fields {
            let adjusted_type_name = self.adjust_type_name_with_api(&field.field_type.rust);
            writeln!(self.file, "    pub {}: {adjusted_type_name},", field.name).unwrap();
        }

        writeln!(self.file, "}}").unwrap();
    }

    fn write_tuple_in_variant(&mut self, fields: &[StructFieldDescription]) {
        write!(self.file, "(").unwrap();
        let adjusted_type_name = self.adjust_type_name_with_api(&fields[0].field_type.rust);
        write!(self.file, "{adjusted_type_name}").unwrap();

        for field in &fields[1..] {
            let adjusted_type_name = self.adjust_type_name_with_api(&field.field_type.rust);
            write!(self.file, ", {adjusted_type_name}").unwrap();
        }

        writeln!(self.file, "),").unwrap();
    }

    fn write_struct_in_variant(&mut self, fields: &[StructFieldDescription]) {
        writeln!(self.file, " {{").unwrap();

        for field in fields {
            let adjusted_type_name = self.adjust_type_name_with_api(&field.field_type.rust);
            writeln!(self.file, "        {}: {adjusted_type_name},", field.name,).unwrap();
        }

        writeln!(self.file, "    }},").unwrap();
    }

    fn clean_paths(&mut self, proxy_crate: &str, rust_type: &str) -> String {
        let delimiters = "<>,()[] ";
        let words: Vec<&str> = rust_type
            .split(|c| delimiters.contains(c))
            .filter(|s| !s.is_empty())
            .collect();

        let mut words_replacer: Vec<String> = Vec::new();
        for word in &words {
            let type_rust_name = word.split("::").last().unwrap().to_string();
            if proxy_crate == extract_struct_crate(word)
                || TYPES_FROM_FRAMEWORK.contains(&type_rust_name.as_str())
            {
                words_replacer.push(type_rust_name);
            } else {
                words_replacer.push(word.to_string());
            }
        }

        let mut rust_type_with_cleaned_path: String = rust_type.to_string().clone();
        for index in 0..words.len() {
            rust_type_with_cleaned_path = rust_type_with_cleaned_path.replace(
                words.get(index).unwrap(),
                words_replacer.get(index).unwrap(),
            );
        }

        rust_type_with_cleaned_path
    }

    fn start_write_type(
        &mut self,
        type_type: &str,
        type_description: &TypeDescription,
        name: &str,
    ) {
        writeln!(self.file).unwrap();
        self.write_macro_attributes(&type_description.macro_attributes);
        write!(self.file, r#"pub {type_type} {name}"#).unwrap();

        if name.contains("<Api>") {
            writeln!(
                self.file,
                r#"
where
    Api: ManagedTypeApi,"#
            )
            .unwrap();
        } else {
            write!(self.file, " ").unwrap();
        }

        writeln!(self.file, r#"{{"#).unwrap();
    }

    pub fn write_macro_attributes(&mut self, macro_attributes: &[String]) {
        if macro_attributes.is_empty() {
            writeln!(self.file, "#[derive(TopEncode, TopDecode)]").unwrap();
        } else {
            writeln!(self.file, "#[derive({})]", macro_attributes.join(", ")).unwrap();
        }
    }

    fn adjust_type_name_with_env_api(&mut self, original_rust_name: &str) -> String {
        self.clean_paths(
            self.meta_config
                .original_contract_abi
                .build_info
                .contract_crate
                .name,
            &original_rust_name
                .replace("multiversx_sc::api::uncallable::UncallableApi", "Env::Api")
                .replace("$API", "Env::Api"),
        )
    }

    fn adjust_type_name_with_api(&mut self, original_rust_name: &str) -> String {
        self.clean_paths(
            self.meta_config
                .original_contract_abi
                .build_info
                .contract_crate
                .name,
            &original_rust_name
                .replace("multiversx_sc::api::uncallable::UncallableApi", "Api")
                .replace("$API", "Api"),
        )
    }

    fn write_end_of_function(&mut self) {
        writeln!(self.file, "    }}").unwrap();
    }
}
