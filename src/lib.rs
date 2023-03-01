use rusb::{Context, DeviceHandle};
use crate::usb::Descriptor;

mod command;
mod usb;

pub enum Manufacturer {
    Epson,
}

pub mod epson {
    pub enum Model {
        TmT88v,
    }
}

impl Manufacturer {
    pub fn get_model(&self, model: epson::Model) -> Box<dyn Printer> {
        match self {
            Manufacturer::Epson => {
                match model {
                    // the epson tm-t88v printer is not yet implemented
                    // epson::Model::TmT88v => Box::new(EpsonTmT88v::new()),
                    _ => panic!("Manufacturer::get_model(): model not found"),
                }
            }
        }
    }
}

struct PrinterHandle {
    usb: DeviceHandle<Context>
}

pub trait Printer {
    fn get_usb_descriptor(&self) -> Descriptor;
}

pub struct EpsonTmT88v {
    handle: Option<PrinterHandle>,

    manufacturer: String,
    name: String,
    model: String,
}
