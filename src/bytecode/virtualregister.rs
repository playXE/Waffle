pub struct VirtualRegister(i32);

impl VirtualRegister {
    pub const INVALID_VIRTUAL_REGISTER: i32 = 0x3fffffff;
    pub fn is_local(self) -> bool {
        self.0 < 0
    }

    pub fn is_argument(self) -> bool {
        self.0 >= 0
    }
}
