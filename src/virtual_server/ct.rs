use std::fmt::Debug;
use std::ops::{Add, Deref};
use serde::{Deserialize, Serialize};
use struct_patch::Patch as SPatch;
use crate::virtual_server::{IVirtualServer, Patch};

#[derive(Debug, Clone, Serialize, Deserialize, SPatch)]
#[patch(attribute(derive(Debug, Default, Deserialize, Serialize)))]
pub struct Container {
    name: String,
    networks: Vec<Network>
}

#[derive(Debug, Clone, Serialize, Deserialize, SPatch, PartialEq)]
#[patch(attribute(derive(Debug, Default, Deserialize, Serialize)))]

struct Network {
    name: String,
    bridge: String,
    ip: Option<String>,
    gateway: Option<String>
}

impl IVirtualServer for Container {
    fn create(&self) {
        println!("Creating container: {:?}", self);
    }

    fn start(&self) {
        println!("Starting container: {:?}", self);
    }

    fn stop(&self) {
        println!("Stopping container: {:?}", self);
    }

    fn update(&self, patch: Patch) {
        println!("Updating container: {:?}", self);
        let patch = match patch {
            Patch::ContainerPatch(patch) => patch,
            _ => panic!("Invalid patch")
        };

        let mut e  = self.clone();

        e.apply(patch);
        
        
        println!("Patched container: {:?}", e);
    }

    fn delete(&self) {
        println!("Deleting container: {:?}", self);
    }
}
