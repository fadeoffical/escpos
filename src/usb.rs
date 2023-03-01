use rusb::{Device, DeviceDescriptor, DeviceHandle, UsbContext};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Descriptor {
    vendor: Vendor,
    product: Product,
}

impl Descriptor {
    pub fn new(vendor: Vendor, product: Product) -> Self {
        Self {
            vendor,
            product,
        }
    }

    pub fn vendor(&self) -> &Vendor {
        &self.vendor
    }

    pub fn product(&self) -> &Product {
        &self.product
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vendor {
    id: u16,
    name: String,
}

impl Vendor {
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    id: u16,
    name: String,
}

impl Product {
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}


pub fn open_device<T: UsbContext>(context: &mut T, descriptor: &Descriptor) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(devices) => devices,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_descriptor = match device.device_descriptor() {
            Ok(descriptor) => descriptor,
            Err(_) => continue,
        };

        if device_descriptor.vendor_id() != descriptor.vendor().id() {
            continue;
        }

        if device_descriptor.product_id() != descriptor.product().id() {
            continue;
        }

        match device.open() {
            Ok(handle) => return Some((device, device_descriptor, handle)),
            Err(e) => panic!("Device found but failed to open: {}", e),
        }
    }

    None
}
