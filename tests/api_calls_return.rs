// Tests in this file call apis and simply expect them to return without causing
// segfaults, etc.

// Not actually testing results. Just calling the api.
#![allow(unused_must_use)]

use nvapi::{PhysicalGpu, ConnectedIdsFlags};

#[test]
fn physicalgpu_display_ids_connected() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = PhysicalGpu::enumerate() {
            for gpu in gpus {
                // Bug: if there are zero connected displays this may crash.
                gpu.display_ids_connected(ConnectedIdsFlags::empty());
            }
        }
    }
}


#[test]
fn physicalgpu_display_ids_all() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = PhysicalGpu::enumerate() {
            for gpu in gpus {
                // Bug: if there are zero connected displays this may crash.
                gpu.display_ids_all();
            }
        }
    }
}
