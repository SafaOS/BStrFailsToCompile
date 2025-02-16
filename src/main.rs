#![no_std]

use bstr::BStr;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
fn main() {
    let s = BStr::new(b"Some string");
    assert_eq!(s.len(), 11);
}
