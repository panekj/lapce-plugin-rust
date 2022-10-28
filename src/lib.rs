#![allow(unused)]

pub mod environment;
pub mod reexport;
pub mod rpc;

#[cfg(any(feature = "github"))]
pub mod github;

#[cfg(feature = "http")]
pub mod http;

#[cfg(any(feature = "zip", feature = "gzip", feature = "tar"))]
pub mod archive;

#[link(wasm_import_module = "lapce")]
extern "C" {
    fn host_handle_rpc();
    fn host_handle_stderr();
}
