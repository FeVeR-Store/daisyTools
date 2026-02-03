mod data_structure;
mod device;
mod transport;
use proc_macro::TokenStream;

use crate::{
    data_structure::{pipeline::pipeline_impl, tlv::tlv_impl},
    device::{device_impl, expose::expose_impl, handle::handle_impl, listen::listen_impl},
    transport::transport_impl,
};

#[proc_macro_attribute]
pub fn tlv(attr: TokenStream, input: TokenStream) -> TokenStream {
    tlv_impl(attr, input)
}

#[proc_macro]
pub fn pipeline(input: TokenStream) -> TokenStream {
    pipeline_impl(input)
}

#[proc_macro]
pub fn device(input: TokenStream) -> TokenStream {
    device_impl(input)
}

#[proc_macro_attribute]
pub fn handle(attr: TokenStream, input: TokenStream) -> TokenStream {
    handle_impl(attr, input)
}

#[proc_macro_attribute]
pub fn listen(attr: TokenStream, input: TokenStream) -> TokenStream {
    listen_impl(attr, input)
}

#[proc_macro_attribute]
pub fn expose(attr: TokenStream, input: TokenStream) -> TokenStream {
    expose_impl(attr, input)
}

#[proc_macro]
pub fn transport(input: TokenStream) -> TokenStream {
    transport_impl(input)
}
