pub mod grpc_message {
    tonic::include_proto!("org.apache.seata.protocol.protobuf.grcp_message");
}

use std::collections::HashMap;
use tonic::transport::Channel;
use lazy_static::lazy_static;
use std::sync::Mutex;
use grpc_message::{seata_service_client::SeataServiceClient, GrpcMessageProto};
use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);
pub struct SessionManager;

lazy_static! {
    pub static ref SESSION_MANAGER: Mutex<HashMap<String, SeataServiceClient<Channel>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

impl SessionManager {
    pub async fn init(addresses: &Vec<String>) {
        let result = INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
        if (result.is_ok()) {
            let mut map = SESSION_MANAGER.lock().unwrap();
            for address in addresses {
                let client = SeataServiceClient::connect(address.clone()).await.unwrap();
                map.insert(address.clone(), client);
            }
            println!("SessionManager initialized");
        } else {
            println!("SessionManager already initialized");
        }
    }

    pub fn get(address: &str) -> Option<SeataServiceClient<Channel>> {
        let map = SESSION_MANAGER.lock().unwrap();
        map.get(address).cloned()
    }
}