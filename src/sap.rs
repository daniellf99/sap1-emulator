struct SAP1 {
    accumulator: u8,
    b_register: u8,
    output_register: u8,
    instruction_register: u8,
    ram: [u8; 16],
    bus: u8,
    pogram_counter: Bit4,
}

struct Bit4 {
    value: u8
}

impl Bit4 {
    pub fn new(value: u8) -> Self {
        // Ensure we only use the lower 4 bits
        return Bit4 { value: value & 0b1111 }
    }

    pub fn value(&self) -> u8{
        return self.value;
    }
}
