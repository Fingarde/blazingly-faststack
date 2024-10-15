mod virtual_server;

use crate::virtual_server::ct::ContainerPatch;
use serde_json::from_str;
use struct_patch::Patch as SPatch;
use virtual_server::VirtualServer;
use crate::virtual_server::{IVirtualServer, Patch};

fn main() {
    let json = r#"
    {
        "type": "ct",
        "name": "test",
        "networks": [
            {
                "name": "eth0",
                "bridge": "vmbr0"
            },
            {
                "name": "eth1",
                "bridge": "vmbr1"
            }
        ]
    }
    "#;
    
    let patch = r#"
    {
        "type": "containerpatch",
        "name": "titi",
        "networks": [
            {
                "bridge": "vmbr0",
                "name": "eth0"
            }
        ]
       
    }
    "#;
    
    let obj = from_str::<VirtualServer>(json).unwrap();
    
    let ct = match obj {
        VirtualServer::Ct(ct) => ct,
        _ => panic!("Invalid type")
    };
    
    ct.create();
    
    // PATCH /ct/:id

    // Repository.find(id)
    let mut ct = ct;

    // Extract patch from request body
    let patch: Patch = serde_json::from_str(patch).unwrap();
    

    // Repository.update(ct)
    ct.update(patch);
}
