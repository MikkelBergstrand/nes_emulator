
pub trait Mapper {
    fn write(addr: u16);
    fn read(addr: u16) -> u8;
}
