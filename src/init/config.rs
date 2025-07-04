pub const USER: &[u8] = b"guest";
pub const HOST: &[u8] = b"rou2ex";

pub static mut PATH: &[u8] = b"/";
pub static mut PATH_CLUSTER: u16 = 0;

pub static mut PATH_BUF: [u8; 256] = [0u8; 256];
pub static mut PATH_LEN: usize = 1;

pub fn set_path(new_path: &[u8]) {
    unsafe {
        let len = new_path.len().min(256);
        PATH_BUF[..len].copy_from_slice(&new_path[..len]);
        PATH_LEN = len;
    }
}

pub fn get_path() -> &'static [u8] {
    unsafe {
        core::slice::from_raw_parts(PATH_BUF.as_ptr(), PATH_LEN)
    }
}

//
//
//

extern "C" {
    pub static mut p4_table: [u64; 512];
}

extern "C" {
    pub static p3_table: u64;
}

extern "C" {
    pub static p2_table: u64;
}

extern "C" {
    pub static mut p3_fb_table: [u64; 512];
    pub static mut p2_fb_table: [u64; 512];
    pub static mut p1_fb_table: [u64; 512];
    pub static mut p1_fb_table_2: [u64; 512];
}

extern "C" {
    pub static multiboot_ptr: u32;
}

extern "C" {
    static debug_flag: u8;
}

pub fn debug_enabled() -> bool {
    unsafe { 
        debug_flag != 0 
    }
}

extern "C" {
    static __stack_start: u8;
    static __stack_end: u8;
}

