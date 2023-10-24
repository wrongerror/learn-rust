#![no_std]
#![no_main]

use aya_bpf::{macros::kprobe, programs::ProbeContext, BpfContext, macros::map, maps::HashMap};
use aya_log_ebpf::info;

#[map]
static CONNECTIONS: HashMap<u32, u64> = HashMap::with_max_entries(10240, 0);

#[kprobe]
pub fn kprobetcp(ctx: ProbeContext) -> u32 {
    match try_kprobetcp(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_kprobetcp(ctx: ProbeContext) -> Result<u32, u32> {
    let pid = ctx.pid();
    let count = unsafe { CONNECTIONS.get(&pid).unwrap_or(&0) } + 1;
    CONNECTIONS.insert(&pid, &count, 0).unwrap();
    info!(&ctx, "function tcp_connect called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
