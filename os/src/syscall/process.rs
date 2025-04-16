//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next};

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
use::crate::address::*;
use::crate::page_table::*;
/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?

fn vatopa(va:VirtAddr)->Option<PhysAddr>{
    let offset = va.page_offset();
    let vpn = va.floor();
    let ppn = PageTable::from_token(current_user_token())
        .translate(vpn)
        .map(|entry| entry.ppn());
    if let Some(ppn) = ppn {
        Some(PhysAddr::combine(ppn,offset))
    }
    else{
        println!("vatopa() fail");
        None
    }
}
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let va = VirtAddr(ts as usize);
    if let Some(pa) = vatopa(va){
        let us = get_time_us();
        let ts_pa = pa.0 as *mut TimeVal;
        unsafe{
            *ts_pa = TimeVal {
                sec: us/1_000_000,
                usec: us%1_000_000,
            };
        }
        0
    }
    else{
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let va = VirtAddr(_id);
    if let Some(pa) = vatopa(va){
        match trace_request {
            0 => {
                if PageTableEntry::readable(va.floor()){
                    let addr = pa as *const u8;
                    unsafe { *addr as isize }
                }
                else{
                    -1
                }
            }
            1 => {
                if PageTableEntry::writable(va.floor()){
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
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let va = VirtAddr(_start);
    if va.aligned() {
        match _prot{
            0 => {
                -1
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let mut p = _start;
                while p < _start+_len {
                    
                    p += PAGE_SIZE;
                }
            }
            _ => -1,
        }
    }
    else{
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
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
