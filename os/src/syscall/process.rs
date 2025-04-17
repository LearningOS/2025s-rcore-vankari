//! Process management syscalls
use crate::task::{cur_mmap,cur_unmmap,current_user_token,change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,systrace_ret};
use crate::timer::get_time_us;
use crate::config::PAGE_SIZE;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}
use crate::mm::{VirtAddr,PageTable};
use crate::mm::translated_struct_ptr;
/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?


pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = translated_struct_ptr(current_user_token(), _ts);
        
        *ts = TimeVal {
            sec: us/1_000_000,
            usec: us%1_000_000,
        };
        
        0
    
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let va = VirtAddr(_id);
    let vpn = va.floor();
    let offset = va.page_offset();
    if let Some(pte) = PageTable::from_token(current_user_token()).find_pte(vpn){
        match _trace_request {
            0 => {
                if pte.readable() && pte.user_accessible(){
                    let ppn = pte.ppn();
                    let pa = usize::from(ppn)*PAGE_SIZE+offset;
                    let addr = pa as *const u8;
                    unsafe { *addr as isize }
                }
                else{
                    -1
                }
            }
            1 => {
                if pte.writable()&&pte.user_accessible(){
                    let ppn = pte.ppn();
                    let pa = usize::from(ppn)*PAGE_SIZE+offset;
                    let addr = pa as *mut u8;
                    unsafe {
                        *addr = _data as u8;
                    }
                    0
                }
                else{
                    -1
                }
            }
            2 => {
                let res = systrace_ret(_id);
                res as isize
            }
            _ => -1,
        }
    }
    else{
        -1
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    if _len == 0 {
        return 0;
    }
    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    cur_mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    cur_unmmap(_start,_len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
