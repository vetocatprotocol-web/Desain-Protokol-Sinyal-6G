#![no_main]

use libfuzzer_sys::fuzz_target;
use sig6g_link::frame::parse_frame;

fuzz_target!(|data: &[u8]| {
    // Parser harus tidak pernah panic, baik input valid maupun invalid.
    let _ = parse_frame(data);
});
