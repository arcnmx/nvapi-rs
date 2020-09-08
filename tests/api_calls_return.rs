// Tests in this file call apis an simply expect them to return without causing 
// segfaults, etc.

// Not actually testing results. Just calling the api.
#![allow(unused_must_use)]

extern crate nvapi;


#[test]
fn physicalgpu_display_ids_connected() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for gpu in gpus {
                // Bug: if there are zero connected displays this may crash.
                gpu.display_ids_connected(nvapi::ConnectedIdsFlags::empty());
            }
        }
    }
}


#[test]
fn physicalgpu_display_ids_all() {
    if let Ok(_) = nvapi::initialize() {
        if let Ok(gpus) = nvapi::PhysicalGpu::enumerate() {
            for gpu in gpus {
                // Bug: if there are zero connected displays this may crash.
                gpu.display_ids_all();
            }
        }
    }
}
