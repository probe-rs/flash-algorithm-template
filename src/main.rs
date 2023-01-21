#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use flash_algorithm::*;

struct Algorithm;

algorithm!(Algorithm);

impl FlashAlgorithm for Algorithm {
    fn new(_address: u32, _clock: u32, _function: Function) -> Result<Self, ErrorCode> {
        // TODO: Add setup code for the flash algorithm.
        Ok(Self)
    }

    fn erase_all(&mut self) -> Result<(), ErrorCode> {
        // TODO: Add code here that erases the entire flash.
        Err(ErrorCode::new(0x70d0).unwrap())
    }

    fn erase_sector(&mut self, _addr: u32) -> Result<(), ErrorCode> {
        // TODO: Add code here that erases a page to flash.
        Ok(())
    }

    fn program_page(&mut self, _addr: u32, _size: u32, _data: *const u8) -> Result<(), ErrorCode> {
        // TODO: Add code here that writes a page to flash.
        Ok(())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        // TODO: Add code here to uninitialize the flash algorithm.
    }
}
