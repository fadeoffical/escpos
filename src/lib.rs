use crate::usb::{Descriptor, Product, Vendor};

pub mod epson;

mod usb;
pub mod job;

pub trait Printer {
    fn get_usb_descriptor(&self) -> Descriptor;

    fn write_direct(&mut self, bytes: &[u8]) -> Result<(), String>;

    fn write_byte(&mut self, byte: u8) -> Result<(), String>;

    fn write(&mut self, bytes: &[u8]) -> Result<(), String>;

    fn writeln(&mut self, bytes: &[u8]) -> Result<(), String>;

    fn flush(&mut self) -> Result<(), String>;

    fn feed(&mut self, lines: u8) -> Result<(), String>;

    fn cut(&mut self) -> Result<(), String>;

    fn init(&mut self) -> Result<(), String>;

    fn finish(&mut self) -> Result<(), String>;
}

pub enum Model {
    TmT88v,
}

impl Model {
    pub fn get_manufacturer(&self) -> Manufacturer {
        match self {
            Model::TmT88v => Manufacturer::Epson,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Model::TmT88v => String::from("TM-T88V"),
        }
    }

    pub fn printer(&self) -> Box<dyn Printer> {
        match self {
            Model::TmT88v => Box::new(epson::TmT88v::new()),
        }
    }

    pub fn get_usb_descriptor(&self) -> Descriptor {
        match self {
            Model::TmT88v => Descriptor::new(
                Vendor::new(0x04b8, String::from("Seiko Epson Corp.")),
                Product::new(0x0202, String::from("TM-T88V"))
            ),
        }
    }
}


pub enum Manufacturer {
    Epson,
}

impl Manufacturer {
    pub fn get_name(&self) -> String {
        match self {
            Manufacturer::Epson => String::from("Epson"),
        }
    }

    pub fn get_printers(&self) -> Vec<Model> {
        match self {
            Manufacturer::Epson => vec![Model::TmT88v],
        }
    }
}
