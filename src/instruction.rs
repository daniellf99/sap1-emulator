pub trait Instruction {
    fn new(instruction: u8);
    fn execute(&self, system: None);
}

struct ADD;
impl Instruction for ADD {
    fn new(instruction: u8) {
        todo!()
    }

    fn execute(&self, system: None) {

    }
}

struct LDA;
impl Instruction for LDA {
    fn execute(&self, system: None) {

    }
}

struct SUB;
impl Instruction for SUB {
    fn execute(&self, system: None) {

    }
}

struct OUT;
impl Instruction for OUT {
    fn execute(&self, system: None) {

    }
}

struct HLT;
impl Instruction for HLT {
    fn execute(&self, system: None) {

    }
}
