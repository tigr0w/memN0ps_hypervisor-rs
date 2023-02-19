extern crate alloc;
use core::{cell::OnceCell};

use alloc::boxed::Box;


use crate::{error::HypervisorError, support, vcpu_data::VcpuData};

pub struct Vcpu {
    /// The index of the processor.
    index: u32,
    
    data: OnceCell<Box<VcpuData>>,
}

impl Vcpu {
    pub fn new(index: u32) -> Result<Self, HypervisorError> {
        log::trace!("Creating processor {}", index);

        Ok (Self {
            index,
            data: OnceCell::new(),
        })
    }

    /// Virtualize the CPU by capturing the context, enabling VMX operation, adjusting control registers, calling VMXON, VMPTRLD and VMLAUNCH
    pub fn virtualize_cpu(&self) -> Result<(), HypervisorError> {
        //log::info!("[+] Capturing context");
        //let context = Context::capture();

        //Check if already virtualized or not, then do it otherwise don't.

        //
        // 2) Intel Manual: 24.7 Enable and Enter VMX Operation
        //
        log::info!("[+] Enabling Virtual Machine Extensions (VMX)");
        support::enable_vmx_operation()?;

        log::info!("[+] Adjusting Control Registers");
        support::adjust_control_registers();

        log::info!("[+] Initializing VcpuData");        
 
        let _vcpu_data = &self.data.get_or_try_init(|| VcpuData::new())?;

        log::info!("[+] Launching VM via VMLAUNCH..............");
        support::vmlaunch()?;
        log::info!("[+] VMLAUNCH successful!");
        
        Ok(())
    }

    /// Devirtualize the CPU using vmxoff
    pub fn devirtualize_cpu(&self) -> Result<(), HypervisorError> {
        support::vmxoff()?;
        Ok(())
    }

    /// Gets the index of the current logical/virtual processor
    pub fn id(&self) -> u32 {
        self.index
    }
}