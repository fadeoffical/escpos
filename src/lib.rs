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

    fn set_default_line_spacing(&mut self) -> Result<(), String>;

    fn set_line_spacing(&mut self, spacing: u8) -> Result<(), String>;
}

pub enum Model {
    EUm30,
    TmJ2000,
    TmJ2100,
    TmL90,
    TmL90Lfc,
    TmL100,
    Tmm10,
    Tmm30,
    Tmm30ii,
    Tmm30iiH,
    Tmm30iii,
    Tmm30iiiH,
    Tmm30iiNt,
    Tmm30iiS,
    Tmm30iiSl,
    Tmm50,
    TmP20,
    TmP20ii,
    TmP60,
    TmP60ii,
    TmP80,
    TmP80i,
    TmT20,
    TmT20ii,
    TmT20iii,
    TmT20iiil,
    TmT20x,
    TmT70,
    TmT70ii,
    TmT81iii,
    TmT82ii,
    TmT82iii,
    TmT82iiil,
    TmT82x,
    TmT83iii,
    TmT88iv,
    TmT88v,
    TmT88vi,
    TmT88vii,
    TmT90,
    TmT100,
    TmU220,
    TmU230,
}

impl Model {
    pub fn get_manufacturer(&self) -> Manufacturer {
        match self {
            Model::TmT88v => Manufacturer::Epson,
            _ => panic!("Not implemented!")
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Model::TmT88v => String::from("TM-T88V"),
            _ => panic!("Not implemented!")
        }
    }

    pub fn get(&self) -> Box<dyn Printer> {
        match self {
            Model::TmT88v => Box::new(epson::TmT88v::new()),
            _ => panic!("Not implemented!")
        }
    }

    pub fn get_usb_descriptor(&self) -> Descriptor {
        match self {
            Model::TmT88v => Descriptor::new(
                Vendor::new(0x04b8, String::from("Seiko Epson Corp.")),
                Product::new(0x0202, String::from("TM-T88V")),
            ),
            _ => panic!("Not implemented!")
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
