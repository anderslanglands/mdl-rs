#[repr(C)]
pub struct Uuid {
    id1: u32,
    id2: u32,
    id3: u32,
    id4: u32,
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "<{:x}, {:x}, {:x}, {:x}>",
            self.id1, self.id2, self.id3, self.id4
        )
    }
}
