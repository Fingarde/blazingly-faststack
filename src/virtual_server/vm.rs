use serde::{Deserialize, Serialize};
use struct_patch::Patch as SPatch;
use crate::virtual_server::IVirtualServer;

#[derive(Debug, Serialize, Deserialize, PartialEq, SPatch)]
#[patch(attribute(derive(Debug, Default, Deserialize, Serialize)))]
pub struct VirtualMachine {
    name: String,
    networks: Vec<Network>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, SPatch)]
#[patch(attribute(derive(Debug, Default, Deserialize, Serialize)))]
struct Network {
    bridge: String,
    ip: String,
    gateway: Option<String>
}

impl IVirtualServer for VirtualMachine {
    fn create(&self) {
        println!("Creating VM: {:?}", self);
    }

    fn start(&self) {
        println!("Starting VM: {:?}", self);
    }

    fn stop(&self) {
        println!("Stopping VM: {:?}", self);
    }

    fn update(&self, patch: crate::virtual_server::Patch) {
        println!("Updating VM: {:?}", self);
        println!("Patch: {:?}", patch);
    }

    fn delete(&self) {
        println!("Deleting VM: {:?}", self);
    }
}