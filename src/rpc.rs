use std::{
    io,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use jsonrpc_lite::{Id, JsonRpc};
use once_cell::sync::Lazy;
use psp_types::{
    lsp_types::{
        notification::{LogMessage, ShowMessage},
        DocumentSelector, LogMessageParams, MessageType, ShowMessageParams, Url,
    },
    Notification, StartLspServer, StartLspServerParams,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub static PLUGIN_RPC: Lazy<PluginServerRpcHandler> = Lazy::new(PluginServerRpcHandler::new);

#[allow(unused_variables)]
pub trait LapcePlugin {
    fn handle_request(&mut self, id: u64, method: String, params: Value) {}
    fn handle_notification(&mut self, method: String, params: Value) {}
}

pub enum PluginServerRpc {
    Request {
        id: u64,
        method: String,
        params: Value,
    },
    Notification {
        method: String,
        params: Value,
    },
}

#[allow(unused)]
pub struct PluginServerRpcHandler {
    rx: Receiver<PluginServerRpc>,
    tx: Sender<PluginServerRpc>,
    id: Arc<AtomicU64>,
}

#[macro_export]
macro_rules! register_plugin {
    ($t:ty) => {
        use std::cell::RefCell;

        thread_local! {
            static STATE: RefCell<$t> = RefCell::new(Default::default());
        }

        fn main() {}

        #[no_mangle]
        pub fn handle_rpc() {
            if let Ok(rpc) = $crate::rpc::parse_stdin() {
                match rpc {
                    $crate::rpc::PluginServerRpc::Request { id, method, params } => {
                        STATE.with(|state| {
                            state.borrow_mut().handle_request(id, method, params);
                        });
                    }
                    $crate::rpc::PluginServerRpc::Notification { method, params } => {
                        STATE.with(|state| {
                            state.borrow_mut().handle_notification(method, params);
                        });
                    }
                }
            }
        }
    };
}

impl PluginServerRpcHandler {
    fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self {
            rx,
            tx,
            id: Arc::new(AtomicU64::new(0)),
        }
    }

    #[allow(unused)]

    pub fn mainloop<H>(&self, handler: &mut H)
    where
        H: LapcePlugin,
    {
        use PluginServerRpc::*;
        for rpc in &self.rx {
            // match rpc {}
        }
    }

    // fn handle_server_request(&self) {
    //     if let Ok(value) = object_from_stdin::<Value>() {
    //         let _ = self.tx.send(PluginServerRpc::Request(value));
    //     }
    // }

    // fn handle_server_notification(&self) {
    //     if let Ok(value) = object_from_stdin::<Value>() {
    //         let _ = self.tx.send(PluginServerRpc::Notification(value));
    //     }
    // }

    // fn handle_rpc(&self) {
    //     if let Ok(value) = object_from_stdin::<Value>() {
    //         let _ = self.tx.send(PluginServerRpc::Notification(value));
    //     }
    // }

    pub fn stderr(&self, msg: &str) {
        eprintln!("{}", msg);
        unsafe { crate::host_handle_stderr() };
    }

    pub fn window_log_message(&self, kind: MessageType, message: String) {
        self.host_notification(LogMessage::METHOD, LogMessageParams { typ: kind, message });
    }

    pub fn window_show_message(&self, kind: MessageType, message: String) {
        self.host_notification(
            ShowMessage::METHOD,
            ShowMessageParams { typ: kind, message },
        );
    }

    pub fn start_lsp(
        &self,
        server_uri: Url,
        server_args: Vec<String>,
        document_selector: DocumentSelector,
        options: Option<Value>,
    ) {
        self.host_notification(
            StartLspServer::METHOD,
            StartLspServerParams {
                server_uri,
                server_args,
                document_selector,
                options,
            },
        );
    }

    #[allow(unused)]

    fn host_request<P: Serialize>(&self, method: &str, params: P) {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let params = serde_json::to_value(params).unwrap();
        send_host_request(id, method, &params);
    }

    fn host_notification<P: Serialize>(&self, method: &str, params: P) {
        let params = serde_json::to_value(params).unwrap();
        send_host_notification(method, &params);
    }
}

// pub fn handle_server_request() {
//     PLUGIN_RPC.handle_server_request();
// }

// pub fn handle_server_notification() {
//     PLUGIN_RPC.handle_server_notification();
// }

// pub fn handle_rpc() {
//     PLUGIN_RPC.handle_rpc();
// }

fn number_from_id(id: &Id) -> u64 {
    match *id {
        Id::Num(n) => n as u64,
        Id::Str(ref s) => s
            .parse::<u64>()
            .expect("failed to convert string id to u64"),
        _ => panic!("unexpected value for id: None"),
    }
}

pub fn parse_stdin() -> Result<PluginServerRpc, serde_json::Error> {
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).unwrap();
    let rpc = match JsonRpc::parse(&msg) {
        Ok(value @ JsonRpc::Request(_)) => {
            let id = number_from_id(&value.get_id().unwrap());
            PluginServerRpc::Request {
                id,
                method: value.get_method().unwrap().to_string(),
                params: serde_json::to_value(value.get_params().unwrap()).unwrap(),
            }
        }
        Ok(value @ JsonRpc::Notification(_)) => PluginServerRpc::Notification {
            method: value.get_method().unwrap().to_string(),
            params: serde_json::to_value(value.get_params().unwrap()).unwrap(),
        },
        Ok(_value @ JsonRpc::Success(_)) => {
            todo!()
        }
        Ok(_value @ JsonRpc::Error(_)) => {
            todo!()
        }
        Err(_err) => {
            todo!()
        }
    };
    Ok(rpc)
}

pub fn object_from_stdin<T: DeserializeOwned>() -> Result<T, serde_json::Error> {
    let mut json = String::new();
    io::stdin().read_line(&mut json).unwrap();
    serde_json::from_str(&json)
}

pub fn object_to_stdout(object: &impl Serialize) {
    println!("{}", serde_json::to_string(object).unwrap());
}

fn send_host_notification(method: &str, params: &Value) {
    object_to_stdout(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
    }));
    unsafe { crate::host_handle_rpc() };
}

fn send_host_request(id: u64, method: &str, params: &Value) {
    object_to_stdout(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    }));
    unsafe { crate::host_handle_rpc() };
}
