my_proc_macro::boot_page! {
    Sv39;
    (0xffffffff_80000000 => 0x00000000_80000000, rwx);
    (0xffffffff_00000000 => 0x00000000_00000000, rwx);
    (0x00000000_80000000 => 0x00000000_80000000, rwx);
}

fn main() {
    println!("Hello, world!");
}
