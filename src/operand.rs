#[derive(Debug, Clone)]
pub struct Operand {
    pub value: f64,
    pub x_power: isize,
}

impl Operand {
    pub fn new(value: f64, x_power: isize) -> Operand {
        Operand { value, x_power }
    }

    pub fn add(mut self, other: Operand) -> Operand {
        self.value += other.value;
        self
    }

    pub fn sub(mut self, other: Operand) -> Operand {
        self.value -= other.value;
        self
    }

    pub fn mul(mut self, other: Operand) -> Operand {
        self.value *= other.value;
        self.x_power += other.x_power;
        self
    }

    pub fn div(mut self, other: Operand) -> Operand {
        self.value /= other.value;
        self.x_power -= other.x_power;
        self
    }

    pub fn to_string(&self) -> String {
        if self.value == 1.0 && self.x_power >= 1 {
            if self.x_power > 1 {
                return format!("X^{}", self.x_power);
            }
            else {
                return String::from("X");
            }
        }
        let value: String = self.value.to_string();
        match self.x_power {
            0 => value,
            1 => format!("{} * X", self.value),
            _ => format!("{} * X^{}", value, self.x_power),
        }
    }
}
