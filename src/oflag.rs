#[repr(C)]
#[allow(dead_code)]
pub enum OFlag {
    ReadOnly = 0x0000,
    WriteOnly = 0x0001,
    ReadWrite = 0x0002,
    NonBlock = 0x0800,
    CloseOnExec = 0x0080000,
}