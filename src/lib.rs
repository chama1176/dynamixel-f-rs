#![no_std]
//! This crate is for dynamixel protocol firmware.
//! Use this crate to share same bus line with dynamixels and communicate.
//!
#![allow(unused_imports)]
pub mod control_data;
pub mod control_table;
pub mod control_table_data;
mod instruction;
pub mod packet_handler;
pub mod utils;
mod data_spec;

use instruction::Instruction;
pub use packet_handler::DynamixelProtocolHandler;
pub use control_data::*;
pub use control_table::ControlTable;
pub use packet_handler::CommunicationResult;
use packet_handler::MAX_PACKET_LEN;
pub use utils::DegRad;

use core::result::Result;
use core::time::Duration;
use heapless::Vec;

pub trait Interface {
    fn write_byte(&mut self, data: u8);
    fn write_bytes(&mut self, data: &[u8]);
    fn read_byte(&mut self) -> Option<u8>;
    fn read_bytes(&mut self, buf: &mut [u8]) -> Option<usize>;
    fn clear_read_buf(&mut self);
}
pub trait Clock {
    fn get_current_time(&self) -> Duration;
}


// 送られてくるパケットに自分宛てのものがあるかチェックする。
// 自分宛てのものがあったら最後のパケットからreturn delay time後に返信を返す
