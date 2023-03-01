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
