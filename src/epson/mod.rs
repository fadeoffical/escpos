use std::time::Duration;
use rusb::{Context, Device, DeviceDescriptor, DeviceHandle};
use crate::Printer;
use crate::usb::{Descriptor, Product, Vendor};

pub(crate) struct TmT88v {
    descriptor: Descriptor,

    device: Device<Context>,
    device_descriptor: DeviceDescriptor,
    device_handle: DeviceHandle<Context>,

    buffer: Vec<u8>,
}

impl TmT88v {
    pub fn new() -> Self {
        let descriptor = Descriptor::new(
            Vendor::new(0x04b8, String::from("Seiko Epson Corp.")),
            Product::new(0x0202, String::from("TM-T88V")),
        );

        let mut context = match Context::new() {
            Ok(context) => context,
            Err(error) => panic!("main(): {}", error),
        };

        let (mut device, device_descriptor, mut device_handle) = match crate::usb::open_device(&mut context, &descriptor) {
            Some((device, descriptor, handle)) => (device, descriptor, handle),
            None => panic!("main(): no device found"),
        };

        println!("main(): device found: {:04x}:{:04x}", device_descriptor.vendor_id(), device_descriptor.product_id());

        let active = match device_handle.kernel_driver_active(0) {
            Ok(active) => active,
            Err(error) => panic!("main(): {}", error),
        };

        if active {
            println!("main(): kernel driver active, detaching");
            match device_handle.detach_kernel_driver(0) {
                Ok(_) => println!("main(): kernel driver detached"),
                Err(error) => panic!("main(): {}", error),
            }
        }

        match device_handle.claim_interface(0) {
            Ok(_) => println!("main(): interface claimed"),
            Err(error) => panic!("main(): {}", error),
        }

        Self {
            descriptor,

            device,
            device_descriptor,
            device_handle,

            buffer: Vec::new(),
        }
    }
}

impl Printer for TmT88v {
    fn get_usb_descriptor(&self) -> Descriptor {
        self.descriptor.clone()
    }

    fn write_direct(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.device_handle.write_bulk(0x01, bytes, Duration::new(1,0)).unwrap();
        Ok(())
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), String> {
        self.write(&[byte])?;
        Ok(())
    }


    fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    fn writeln(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.write(bytes)?;
        self.write_byte("\n".as_bytes()[0])?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        self.device_handle.write_bulk(0x01, &self.buffer, Duration::new(1,0)).unwrap();
        self.buffer.clear();
        Ok(())
    }

    fn feed(&mut self, lines: u8) -> Result<(), String> {
        self.write_direct(&[0x0a_u8].repeat(lines as usize))?;
        Ok(())
    }

    fn cut(&mut self) -> Result<(), String>  {
        self.write_direct(&[0x1b_u8, 0x69_u8])?;
        Ok(())
    }

    fn init(&mut self) -> Result<(), String> {
        Ok(()) // todo
    }

    fn finish(&mut self) -> Result<(), String> {
        self.write_direct(&[0x0c_u8])?;
        Ok(())
    }
}
