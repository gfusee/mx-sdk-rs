use std::{fs::File, io::Write};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

pub(crate) fn write_imports(file: &mut File) {
    writeln!(
        file,
        r#"#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
multiversx_sc::imports!();"#
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_template(file: &mut File) {
    write!(
        file,
        "pub struct TxProxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{{
    pub address: ManagedOption<A, ManagedAddress<A>>,
}}

impl<A> TxProxy<A>
where
    A: multiversx_sc::api::VMApi + 'static,
{{"
    )
    .unwrap();
}
