use std::time::Duration;
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, UsbContext};

fn main() {
    let buf = ["hello world\n\n\n\n\n\n".as_bytes(), &[0x1b_u8, 0x69_u8]].concat();

    let mut context = match Context::new() {
        Ok(context) => context,
        Err(error) => panic!("main(): {}", error),
    };

    let (mut _device, descriptor, mut handle) = match open_device(&mut context, 0x04b8, 0x0202) {
        Some((device, descriptor, handle)) => (device, descriptor, handle),
        None => panic!("main(): no device found"),
    };

    println!("main(): device found: {:04x}:{:04x}", descriptor.vendor_id(), descriptor.product_id());

    let active = match handle.kernel_driver_active(0) {
        Ok(active) => active,
        Err(error) => panic!("main(): {}", error),
    };

    if active {
        println!("main(): kernel driver active, detaching");
        match handle.detach_kernel_driver(0) {
            Ok(_) => println!("main(): kernel driver detached"),
            Err(error) => panic!("main(): {}", error),
        }
    }

    match handle.claim_interface(0) {
        Ok(_) => println!("main(): interface claimed"),
        Err(error) => panic!("main(): {}", error),
    }

    handle.write_bulk(0x01, &buf, Duration::from_secs(1)).unwrap();
}

fn open_device<T: UsbContext>(context: &mut T, vendor_id: u16, product_id: u16) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(devices) => devices,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let descriptor = match device.device_descriptor() {
            Ok(descriptor) => descriptor,
            Err(_) => continue,
        };

        if descriptor.vendor_id() != vendor_id || descriptor.product_id() != product_id {
            continue;
        }

        match device.open() {
            Ok(handle) => return Some((device, descriptor, handle)),
            Err(e) => panic!("Device found but failed to open: {}", e),
        }
    }

    None
}
