use anyhow::Result;
use sgx_oc::ocall;
use sgx_trts::veh::{
    register, unregister, CpuContext, ExceptionInfo, ExceptionType, ExceptionVector, Handle,
    HandleResult,
};

pub(crate) struct ExceptionHandler {
    handle: Handle,
}

impl ExceptionHandler {
    pub fn new() -> Result<Self, anyhow::Error> {
        match register(handle_nts_exception) {
            Ok(handle) => Ok(Self { handle }),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to register exception handler: {:?}",
                e
            )),
        }
    }
}

impl Drop for ExceptionHandler {
    fn drop(&mut self) {
        unregister(self.handle);
    }
}

#[no_mangle]
pub extern "C" fn handle_nts_exception(info: &mut ExceptionInfo) -> HandleResult {
    const CPUID_OPCODE: u16 = 0xA20F;

    let mut result = HandleResult::Search;
    if info.vector == ExceptionVector::UD && info.exception_type == ExceptionType::Hardware {
        let ip_opcode_ptr = info.context.rip as *const u16;
        let ip_opcode = unsafe { ip_opcode_ptr.read_unaligned() };
        if ip_opcode == CPUID_OPCODE {
            result = handle_cpuid_exception(&mut info.context);
        }
    }
    result
}

fn handle_cpuid_exception(context: &mut CpuContext) -> HandleResult {
    let leaf = context.rax as u32;
    match unsafe { ocall::cpuid(leaf) } {
        Ok(cpuid_result) => {
            context.rax = cpuid_result.eax as u64;
            context.rbx = cpuid_result.ebx as u64;
            context.rcx = cpuid_result.ecx as u64;
            context.rdx = cpuid_result.edx as u64;
            context.rip += 2;
            HandleResult::Execution
        }
        Err(_) => HandleResult::Search,
    }
}
