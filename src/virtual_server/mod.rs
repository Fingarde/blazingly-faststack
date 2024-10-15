pub mod vm;
pub mod ct;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use vm::VirtualMachine;
use ct::Container;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
#[enum_dispatch(IVirtualServer)]
pub enum VirtualServer {
    Vm(VirtualMachine),
    Ct(Container)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Patch {
    VirtualMachinePatch(vm::VirtualMachinePatch),
    ContainerPatch(ct::ContainerPatch)
}



#[enum_dispatch]
pub trait IVirtualServer {
    fn create(&self);
    fn start(&self);
    fn stop(&self);
    fn update(&self, patch: Patch);
    fn delete(&self);
}