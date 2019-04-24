pub struct Operand {
    pub value: f64,
    pub x_factor: u8,
}

impl Operand {
    pub fn new(value: f64, x_factor: u8) -> Operand {
        Operand {
            value,
            x_factor,
        }
    }

    pub fn can_add_or_sub(&self, other: &Operand) -> bool {
        self.x_factor == other.x_factor
    }

    pub fn add(mut self, other: Operand) -> Operand {
        self.value +=  other.value;
        self
    }

    pub fn sub(mut self, other: Operand) -> Operand {
        self.value -=  other.value;
        self
    }

    pub fn mul(mut self, other: Operand) -> Operand {
        self.value *=  other.value;
        self.x_factor +=  other.x_factor;
        self
    }

    pub fn div(mut self, other: Operand) -> Operand {
        self.value /=  other.value;
        self.x_factor -=  other.x_factor;
        self
    }
}