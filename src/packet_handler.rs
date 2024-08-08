use crate::control_table;
use crate::BufferInterface;
use crate::Clock;
use crate::ControlTable;
use crate::ControlTableData;
use crate::Instruction;

use core::fmt;
use core::fmt::Write;
use core::result::Result;
use core::time::Duration;
use heapless::Vec;

pub const MAX_PACKET_LEN: usize = 128;
pub const BROADCAST_ID: u8 = 0xFE;

#[allow(dead_code)]
pub enum Packet {
    Header0,
    Header1,
    Header2,
    Reserved,
    Id,
    LengthL,
    LengthH,
    Instruction,
    Error,
    Parameter0,
}

#[allow(dead_code)]
impl Packet {
    pub fn to_pos(&self) -> usize {
        match self {
            Packet::Header0 => 0,
            Packet::Header1 => 1,
            Packet::Header2 => 2,
            Packet::Reserved => 3,
            Packet::Id => 4,
            Packet::LengthL => 5,
            Packet::LengthH => 6,
            Packet::Instruction => 7,
            Packet::Error => 8,
            Packet::Parameter0 => 8,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ErrorBit {
    ErrNone = 0x00,
    ErrResultFail = 0x01,
    ErrInstruction = 0x02,
    ErrCRC = 0x03,
    ErrDataRange = 0x04,
    ErrDataLength = 0x05,
    ErrDataLimit = 0x06,
    ErrAccess = 0x07,
    ErrAlert = 0x08,
}

impl From<ErrorBit> for u8 {
    #[inline(always)]
    fn from(variant: ErrorBit) -> Self {
        variant as _
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationResult {
    Success,
    PortBusy,
    TxFail,
    RxFail,
    TxError,
    RxWaiting,
    RxTimeout,
    RxCorrupt,
    RxCRCError,
    NotAvailable,
    SomethingWentWrong,
}

impl fmt::Display for CommunicationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommunicationResult::Success => write!(f, "[TxRxResult] Communication success."),
            CommunicationResult::PortBusy => write!(f, "[TxRxResult] Port is in use!"),
            CommunicationResult::TxFail => {
                write!(f, "[TxRxResult] Failed transmit instruction packet!")
            }
            CommunicationResult::RxFail => {
                write!(f, "[TxRxResult] Failed get status packet from device!")
            }
            CommunicationResult::TxError => write!(f, "[TxRxResult] Incorrect instruction packet!"),
            CommunicationResult::RxWaiting => {
                write!(f, "[TxRxResult] Now receiving packet!")
            }
            CommunicationResult::RxTimeout => write!(f, "[TxRxResult] There is no status packet!"),
            CommunicationResult::RxCorrupt => write!(f, "[TxRxResult] Incorrect status packet!"),
            CommunicationResult::RxCRCError => write!(f, "[TxRxResult] Incorrect Rx CRC!"),
            CommunicationResult::NotAvailable => {
                write!(f, "[TxRxResult] Protocol does not support This function!")
            }
            CommunicationResult::SomethingWentWrong => {
                write!(f, "[TxRxResult] Something went wrong!")
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProtocolHandlerParsingState {
    WaitForCommandPacket,
    WaitForOthersResponsePacket, // After this wait for return delay time
    WaitReturnDelayTime,
    Init,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PacketReceivingState {
    Waiting,
    Init,
}

trait DynamixelPacket {
    fn add_stuffing(&mut self);
    fn remove_stuffing(&mut self);
}

impl DynamixelPacket for Vec<u8, MAX_PACKET_LEN> {
    fn add_stuffing(&mut self) {
        let packet_length_in = u16::from_le_bytes([
            self[Packet::LengthL.to_pos()],
            self[Packet::LengthH.to_pos()],
        ]);
        let mut packet_length_out = packet_length_in;

        if packet_length_in < 8 {
            // INSTRUCTION, ADDR_L, ADDR_H, CRC16_L, CRC16_H + FF FF FD
            return;
        }

        let packet_length_before_crc = packet_length_in - 2;
        for i in 3..packet_length_before_crc as usize {
            let check = i + Packet::Instruction.to_pos() - 2;
            if self[check + 0] == 0xFF && self[check + 1] == 0xFF && self[check + 2] == 0xFD {
                packet_length_out += 1;
            }
        }

        if packet_length_in == packet_length_out {
            // no stuffing required
            return;
        }
        self.resize(
            self.len() + packet_length_out as usize - packet_length_in as usize,
            0,
        )
        .unwrap();

        let mut out_index = packet_length_out as usize + 6 - 2; // last index before crc
        let mut in_index = packet_length_in as usize + 6 - 2; // last index before crc
        while out_index != in_index {
            if self[in_index] == 0xFD && self[in_index - 1] == 0xFF && self[in_index - 2] == 0xFF {
                self[out_index] = 0xFD; // byte stuffing
                out_index -= 1;
                if out_index != in_index {
                    self[out_index] = self[in_index]; // FD
                    out_index -= 1;
                    in_index -= 1;
                    self[out_index] = self[in_index]; // FF
                    out_index -= 1;
                    in_index -= 1;
                    self[out_index] = self[in_index]; // FF
                    out_index -= 1;
                    in_index -= 1;
                }
            } else {
                self[out_index] = self[in_index];
                out_index -= 1;
                in_index -= 1;
            }
        }

        self[Packet::LengthL.to_pos()] = packet_length_out.to_le_bytes()[0];
        self[Packet::LengthH.to_pos()] = packet_length_out.to_le_bytes()[1];

        return;
    }

    fn remove_stuffing(&mut self) {
        let packet_length_in = u16::from_le_bytes([
            self[Packet::LengthL.to_pos()],
            self[Packet::LengthH.to_pos()],
        ]);
        let mut packet_length_out = packet_length_in;

        let mut index = Packet::Instruction.to_pos() as usize;
        let mut i = 0;
        // except CRC
        while i < (packet_length_in - 2) as usize {
            if self[i + Packet::Instruction.to_pos()] == 0xFD
                && self[i + Packet::Instruction.to_pos() + 1] == 0xFD
                && self[i + Packet::Instruction.to_pos() - 1] == 0xFF
                && self[i + Packet::Instruction.to_pos() - 2] == 0xFF
            {
                // FF FF FD FD
                packet_length_out -= 1;
                i += 1;
            }
            self[index] = self[i + Packet::Instruction.to_pos()];
            index += 1;
            i += 1;
        }

        self[index] = self[Packet::Instruction.to_pos() + packet_length_in as usize - 2];
        index += 1;
        self[index] = self[Packet::Instruction.to_pos() + packet_length_in as usize - 1];
        index += 1;

        self[Packet::LengthL.to_pos()] = packet_length_out.to_le_bytes()[0];
        self[Packet::LengthH.to_pos()] = packet_length_out.to_le_bytes()[1];
        self.resize(index, 0).unwrap();
    }
}

pub struct DynamixelProtocolHandler<I, C>
where
    I: BufferInterface,
    C: Clock,
{
    pub uart: I,
    clock: C,
    // is_enabled: bool,
    is_using: bool,
    // packet_start_time: Duration,
    // packet_timeout: Duration,
    baudrate: u32,
    // tx_time_per_byte: u64,
    return_packet: Vec<u8, MAX_PACKET_LEN>,
    packet_return_time: Duration,
    pub ctd: ControlTableData,
    wait_length: usize,
    msg: Vec<u8, MAX_PACKET_LEN>, // VecDeque is not implemented in heapless.👺heapless::Dequeに置き換え可能？
    parsing_state: ProtocolHandlerParsingState,
    packet_receiving_state: PacketReceivingState,
    last_received_command: u8,
    last_received_id: u8,
}

#[allow(dead_code)]
impl<I, C> DynamixelProtocolHandler<I, C>
where
    I: BufferInterface,
    C: Clock,
{
    pub fn new(uart: I, clock: C, baudrate: u32, control_table_data: ControlTableData) -> Self {
        Self {
            uart,
            clock,
            // is_enabled: false,
            is_using: false,
            // packet_start_time: Duration::new(0, 0),
            // packet_timeout: Duration::new(0, 0),
            baudrate: baudrate,
            // tx_time_per_byte: ((1_000_000.0 * 8.0 + (baudrate as f32 - 1.0)) / baudrate as f32)
            //     as u64,
            return_packet: Vec::new(),
            packet_return_time: Duration::new(0, 0),
            ctd: control_table_data,
            wait_length: 0,
            msg: Vec::<u8, MAX_PACKET_LEN>::new(),
            parsing_state: ProtocolHandlerParsingState::Init,
            packet_receiving_state: PacketReceivingState::Init,
            last_received_command: Instruction::Unknown.into(),
            last_received_id: 1,
        }
    }

    pub fn parse_data(&mut self) -> Result<(), ()> {
        if self.parsing_state == ProtocolHandlerParsingState::Init
            || self.parsing_state == ProtocolHandlerParsingState::WaitForCommandPacket
        {
            // masterからの指令待ちはタイムアウト不要
            match self.receive_packet(Duration::new(0, 0)) {
                Ok(v) => {
                    // ブロードキャストではなく、自分のIDと異なる場合は何もしなくて良い
                    if v[Packet::Id.to_pos()] != BROADCAST_ID
                        && v[Packet::Id.to_pos()] != self.ctd.read().id()
                    {
                        return Ok(());
                    };

                    match v[Packet::Instruction.to_pos()] {
                        x if x == Instruction::Ping.into() => {
                            // return packetにセットしてまだ送らない
                            self.return_packet = self.ping_response_packet(
                                self.ctd.read().id(),
                                self.ctd.read().model_number(),
                                self.ctd.read().firmware_version(),
                            );
                            self.last_received_command = Instruction::Ping.into();
                            self.parsing_state =
                                ProtocolHandlerParsingState::WaitForOthersResponsePacket;
                        }
                        x if x == Instruction::Read.into() => {
                            let address = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos()],
                                v[Packet::Parameter0.to_pos() + 1],
                            ]) as usize;
                            let length = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos() + 2],
                                v[Packet::Parameter0.to_pos() + 3],
                            ]) as usize;
                            // return packetにセットしてまだ送らない
                            self.return_packet = self.read_response_packet(
                                self.ctd.read().id(),
                                &self.ctd.read().bits()[address..address + length],
                            );
                            self.last_received_command = Instruction::Read.into();
                            self.parsing_state = ProtocolHandlerParsingState::WaitReturnDelayTime;
                        }
                        x if x == Instruction::Write.into() => {
                            let address = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos()],
                                v[Packet::Parameter0.to_pos() + 1],
                            ]) as usize;
                            let data_len = u16::from_le_bytes([
                                v[Packet::LengthL.to_pos()],
                                v[Packet::LengthH.to_pos()],
                            ]) as usize
                                - 5;
                            self.ctd.modify(|_, w| {
                                w.bytes(
                                    address,
                                    &v[(Packet::Parameter0.to_pos() + 2)
                                        ..(Packet::Parameter0.to_pos() + 2 + data_len)],
                                )
                            });
                            // return packetにセットしてまだ送らない
                            self.return_packet = self.write_response_packet(self.ctd.read().id());
                            self.last_received_command = Instruction::Write.into();
                            self.parsing_state = ProtocolHandlerParsingState::WaitReturnDelayTime;
                        }
                        x if x == Instruction::SyncRead.into() => {
                            let id_len = u16::from_le_bytes([
                                v[Packet::LengthL.to_pos()],
                                v[Packet::LengthH.to_pos()],
                            ]) as usize
                                - 7; // 7 = instruction + address(2) + read_length(2) + crc(2)
                            let address = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos()],
                                v[Packet::Parameter0.to_pos() + 1],
                            ]) as usize;
                            let length = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos() + 2],
                                v[Packet::Parameter0.to_pos() + 3],
                            ]) as usize;
                            // return packetにセットしてまだ送らない
                            self.return_packet = self.read_response_packet(
                                self.ctd.read().id(),
                                &self.ctd.read().bits()[address..address + length],
                            );
                            self.last_received_command = Instruction::SyncRead.into();
                            self.parsing_state =
                                ProtocolHandlerParsingState::WaitForOthersResponsePacket;
                        }
                        x if x == Instruction::SyncWrite.into() => {
                            let address = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos()],
                                v[Packet::Parameter0.to_pos() + 1],
                            ]) as usize;
                            let length = u16::from_le_bytes([
                                v[Packet::Parameter0.to_pos() + 2],
                                v[Packet::Parameter0.to_pos() + 3],
                            ]) as usize;
                            let id_len = (u16::from_le_bytes([
                                v[Packet::LengthL.to_pos()],
                                v[Packet::LengthH.to_pos()],
                            ]) as usize
                                - 7)
                                / (length + 1);
                            // 7 = instruction + address(2) + read_length(2) + crc(2)
                            // id + data lengthで詰まっているのでidが一致する場合書き込む
                            for i in 0..id_len {
                                let id_pos = Packet::Parameter0.to_pos() + 4 + i * (length + 1);
                                if v[id_pos] == self.ctd.read().id() {
                                    self.ctd.modify(|_, w| {
                                        w.bytes(address, &v[(id_pos + 1)..(id_pos + 1 + length)])
                                    });
                                }
                            }
                            // 返信は不要
                            self.parsing_state = ProtocolHandlerParsingState::Init;
                            return Ok(());
                        }
                        _ => {
                            return Ok(());
                        }
                    };
                }
                Err(e) => {
                    if e == CommunicationResult::RxWaiting {
                        self.parsing_state = ProtocolHandlerParsingState::WaitForCommandPacket;
                        return Ok(());
                    } else {
                        self.parsing_state = ProtocolHandlerParsingState::Init;
                        return Err(());
                    }
                }
            }
        }

        if self.parsing_state == ProtocolHandlerParsingState::WaitForOthersResponsePacket {
            // 他のサーボ待ち
            for _ in self.last_received_id..self.ctd.read().id() {
                // x byte * 8 / baudrate * 1e6
                // return delayは最大で500us?
                let wait_us = self.return_packet.len() as u32 * 8 * 1_000_000 / self.baudrate + 500;
                match self.receive_packet(Duration::from_micros(wait_us.into())) {
                    Ok(ov) => {
                        self.last_received_id = ov[Packet::Id.to_pos()];
                        if ov[Packet::Id.to_pos()] == self.ctd.read().id() - 1 {
                            // 1つ前のidまで来ていれば抜ける
                            self.parsing_state = ProtocolHandlerParsingState::WaitReturnDelayTime;
                            break;
                        }
                    }
                    Err(e) => {
                        if e == CommunicationResult::RxWaiting {
                            return Ok(());
                        } else if e == CommunicationResult::RxTimeout {
                            // 他のサーボ待ちなのでTimeoutはエラーではない
                            continue;
                        } else {
                            self.parsing_state = ProtocolHandlerParsingState::Init;
                            return Err(());
                        }
                    }
                }
            }
        }

        if self.parsing_state == ProtocolHandlerParsingState::WaitReturnDelayTime {
            if self.ctd.read().return_delay_time() > 0 {
                // waitが必要
            } else {
                // 追加待ちなし
            }
        }
        // 送信
        self.uart.write_bytes(&self.return_packet);
        // 完了なので状態を初期化する
        self.parsing_state = ProtocolHandlerParsingState::Init;
        self.packet_receiving_state = PacketReceivingState::Init;
        self.last_received_id = 1;
        return Ok(());
    }

    pub fn packet_return_time(&self) -> Duration {
        self.packet_return_time.clone()
    }

    pub fn return_packet(&mut self) -> Vec<u8, MAX_PACKET_LEN> {
        self.return_packet.clone()
    }

    fn receive_packet(
        &mut self,
        timeout: Duration,
    ) -> Result<Vec<u8, MAX_PACKET_LEN>, CommunicationResult> {
        if self.packet_receiving_state == PacketReceivingState::Init {
            self.wait_length = 10; // minimum length (HEADER0 HEADER1 HEADER2 RESERVED ID LENGTH_L LENGTH_H INST CRC16_L CRC16_H)
            self.msg = Vec::<u8, MAX_PACKET_LEN>::new(); // VecDeque is not implemented in heapless.
        }

        let result;

        loop {
            let mut res = Vec::<u8, MAX_PACKET_LEN>::new();
            res.resize(self.wait_length - self.msg.len(), 0).unwrap();
            match self.uart.read_bytes(&mut *res) {
                None => {}
                Some(readlen) => {
                    self.msg.extend(res[0..readlen].iter().cloned());
                }
            }

            if self.msg.len() >= self.wait_length {
                let mut idx = 0;
                // find packet header
                while idx < (self.msg.len() - 3) {
                    if self.msg[idx + Packet::Header0.to_pos()] == 0xFF
                        && self.msg[idx + Packet::Header1.to_pos()] == 0xFF
                        && self.msg[idx + Packet::Header2.to_pos()] == 0xFD
                        && self.msg[idx + Packet::Reserved.to_pos()] == 0x00
                    {
                        break;
                    }
                    idx += 1;
                }

                if idx == 0 {
                    // found at the beginning of the packet
                    if self.msg[Packet::Reserved.to_pos()] != 0x00
                        || (self.msg[Packet::Id.to_pos()] > 0xFC
                            && self.msg[Packet::Id.to_pos()] != 0xFE)
                        || u16::from_le_bytes([
                            self.msg[Packet::LengthL.to_pos()],
                            self.msg[Packet::LengthH.to_pos()],
                        ]) as usize
                            > MAX_PACKET_LEN
                    {
                        // remove the first byte in the packet
                        for s in 0..self.msg.len() - 1 {
                            self.msg[s] = self.msg[s + 1];
                        }
                        self.msg.truncate(self.msg.len() - 1);
                        continue;
                    }
                    // re-calculate the exact length of the rx packet
                    if self.wait_length
                        != u16::from_le_bytes([
                            self.msg[Packet::LengthL.to_pos()],
                            self.msg[Packet::LengthH.to_pos()],
                        ]) as usize
                            + Packet::LengthH.to_pos()
                            + 1
                    {
                        self.wait_length = u16::from_le_bytes([
                            self.msg[Packet::LengthL.to_pos()],
                            self.msg[Packet::LengthH.to_pos()],
                        ]) as usize
                            + Packet::LengthH.to_pos()
                            + 1;
                        continue;
                    }

                    if self.msg.len() < self.wait_length {
                        // check timeout
                        if !timeout.is_zero() && self.clock.get_current_time() > timeout {
                            result = CommunicationResult::RxTimeout;
                            break;
                        } else {
                            // continue;
                            // 関数をブロッキングにしないために時間待ちはこのループでは行わない
                            result = CommunicationResult::RxWaiting;
                            break;
                        }
                    }

                    // verify CRC16
                    let crc = u16::from_le_bytes([
                        self.msg[self.msg.len() - 2],
                        self.msg[self.msg.len() - 1],
                    ]);
                    if self.calc_crc_value(&self.msg[..self.msg.len() - 2]) == crc {
                        result = CommunicationResult::Success;
                    } else {
                        result = CommunicationResult::RxCRCError;
                    }
                    break;
                } else {
                    // remove unnecessary packets
                    for s in 0..(self.msg.len() - idx) {
                        self.msg[s] = self.msg[idx + s];
                    }
                    self.msg.truncate(self.msg.len() - idx);
                }
            } else {
                // check timeout
                // スタート時間の考慮が必要👺
                if !timeout.is_zero() && self.clock.get_current_time() > timeout {
                    result = CommunicationResult::RxTimeout;
                    break;
                } else {
                    // continue;
                    // 関数をブロッキングにしないために時間待ちはこのループでは行わない
                    result = CommunicationResult::RxWaiting;
                    break;
                }
            }
        }
        self.is_using = false;

        if result == CommunicationResult::RxWaiting {
            self.packet_receiving_state = PacketReceivingState::Waiting;
        } else {
            self.packet_receiving_state = PacketReceivingState::Init;
        }

        if result == CommunicationResult::Success {
            self.msg.remove_stuffing();
            let mut result_msg = Vec::<u8, MAX_PACKET_LEN>::new();
            result_msg.extend(self.msg.iter().cloned());
            Ok(result_msg)
        } else {
            Err(result)
        }
    }

    fn reserve_msg_header(&self) -> [u8; 4] {
        [0xFF, 0xFF, 0xFD, 0x00] // Header and reserved len
    }

    fn ping_response_packet(
        &self,
        id: u8,
        model_number: u16,
        firmware_version: u8,
    ) -> Vec<u8, MAX_PACKET_LEN> {
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        let length: u16 = 1 + 1 + 3 + 2; // instruction + err + data(3) + crc(2)

        msg.extend(self.reserve_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned()); // Set length temporary
        msg.push(Instruction::Status as u8).unwrap();
        msg.push(0).unwrap(); // err
        msg.extend(model_number.to_le_bytes().iter().cloned());
        msg.extend(firmware_version.to_le_bytes().iter().cloned());

        // add crc
        msg.extend(self.calc_crc_value(&msg).to_le_bytes().iter().cloned());

        msg
    }

    fn read_response_packet(&self, id: u8, data: &[u8]) -> Vec<u8, MAX_PACKET_LEN> {
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        let length: u16 = 1 + 1 + 4 + 2; // instruction + err + data + crc(2)

        msg.extend(self.reserve_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned()); // Set length temporary
        msg.push(Instruction::Status as u8).unwrap();
        msg.push(0).unwrap(); // err
        msg.extend(data.iter().cloned());

        // add crc
        msg.extend(self.calc_crc_value(&msg).to_le_bytes().iter().cloned());

        msg
    }

    fn write_response_packet(&self, id: u8) -> Vec<u8, MAX_PACKET_LEN> {
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        let length: u16 = 1 + 1 + 2; // instruction + err + crc(2)

        msg.extend(self.reserve_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned()); // Set length temporary
        msg.push(Instruction::Status as u8).unwrap();
        msg.push(0).unwrap(); // err

        // add crc
        msg.extend(self.calc_crc_value(&msg).to_le_bytes().iter().cloned());

        msg
    }

    fn calc_crc_value(&self, msg: &[u8]) -> u16 {
        let crc_table = [
            0x0000, 0x8005, 0x800F, 0x000A, 0x801B, 0x001E, 0x0014, 0x8011, 0x8033, 0x0036, 0x003C,
            0x8039, 0x0028, 0x802D, 0x8027, 0x0022, 0x8063, 0x0066, 0x006C, 0x8069, 0x0078, 0x807D,
            0x8077, 0x0072, 0x0050, 0x8055, 0x805F, 0x005A, 0x804B, 0x004E, 0x0044, 0x8041, 0x80C3,
            0x00C6, 0x00CC, 0x80C9, 0x00D8, 0x80DD, 0x80D7, 0x00D2, 0x00F0, 0x80F5, 0x80FF, 0x00FA,
            0x80EB, 0x00EE, 0x00E4, 0x80E1, 0x00A0, 0x80A5, 0x80AF, 0x00AA, 0x80BB, 0x00BE, 0x00B4,
            0x80B1, 0x8093, 0x0096, 0x009C, 0x8099, 0x0088, 0x808D, 0x8087, 0x0082, 0x8183, 0x0186,
            0x018C, 0x8189, 0x0198, 0x819D, 0x8197, 0x0192, 0x01B0, 0x81B5, 0x81BF, 0x01BA, 0x81AB,
            0x01AE, 0x01A4, 0x81A1, 0x01E0, 0x81E5, 0x81EF, 0x01EA, 0x81FB, 0x01FE, 0x01F4, 0x81F1,
            0x81D3, 0x01D6, 0x01DC, 0x81D9, 0x01C8, 0x81CD, 0x81C7, 0x01C2, 0x0140, 0x8145, 0x814F,
            0x014A, 0x815B, 0x015E, 0x0154, 0x8151, 0x8173, 0x0176, 0x017C, 0x8179, 0x0168, 0x816D,
            0x8167, 0x0162, 0x8123, 0x0126, 0x012C, 0x8129, 0x0138, 0x813D, 0x8137, 0x0132, 0x0110,
            0x8115, 0x811F, 0x011A, 0x810B, 0x010E, 0x0104, 0x8101, 0x8303, 0x0306, 0x030C, 0x8309,
            0x0318, 0x831D, 0x8317, 0x0312, 0x0330, 0x8335, 0x833F, 0x033A, 0x832B, 0x032E, 0x0324,
            0x8321, 0x0360, 0x8365, 0x836F, 0x036A, 0x837B, 0x037E, 0x0374, 0x8371, 0x8353, 0x0356,
            0x035C, 0x8359, 0x0348, 0x834D, 0x8347, 0x0342, 0x03C0, 0x83C5, 0x83CF, 0x03CA, 0x83DB,
            0x03DE, 0x03D4, 0x83D1, 0x83F3, 0x03F6, 0x03FC, 0x83F9, 0x03E8, 0x83ED, 0x83E7, 0x03E2,
            0x83A3, 0x03A6, 0x03AC, 0x83A9, 0x03B8, 0x83BD, 0x83B7, 0x03B2, 0x0390, 0x8395, 0x839F,
            0x039A, 0x838B, 0x038E, 0x0384, 0x8381, 0x0280, 0x8285, 0x828F, 0x028A, 0x829B, 0x029E,
            0x0294, 0x8291, 0x82B3, 0x02B6, 0x02BC, 0x82B9, 0x02A8, 0x82AD, 0x82A7, 0x02A2, 0x82E3,
            0x02E6, 0x02EC, 0x82E9, 0x02F8, 0x82FD, 0x82F7, 0x02F2, 0x02D0, 0x82D5, 0x82DF, 0x02DA,
            0x82CB, 0x02CE, 0x02C4, 0x82C1, 0x8243, 0x0246, 0x024C, 0x8249, 0x0258, 0x825D, 0x8257,
            0x0252, 0x0270, 0x8275, 0x827F, 0x027A, 0x826B, 0x026E, 0x0264, 0x8261, 0x0220, 0x8225,
            0x822F, 0x022A, 0x823B, 0x023E, 0x0234, 0x8231, 0x8213, 0x0216, 0x021C, 0x8219, 0x0208,
            0x820D, 0x8207, 0x0202,
        ];

        let mut crc_accum = 0x0000;
        for j in 0..msg.len() {
            let i = ((((crc_accum >> 8) as u8) ^ msg[j]) & 0xFF) as usize;
            crc_accum = (crc_accum << 8) ^ crc_table[i];
        }

        crc_accum
    }

    fn clear_port(&mut self) {
        self.uart.clear_read_buf();
    }
}

#[cfg(test)]
mod tests {
    use crate::control_table;
    use crate::control_table::BitsW;
    use crate::packet_handler::DynamixelPacket;
    use crate::packet_handler::PacketReceivingState;
    use crate::packet_handler::ProtocolHandlerParsingState;
    use crate::packet_handler::MAX_PACKET_LEN;
    use crate::Clock;
    use crate::ControlTable;
    use crate::ControlTableData;
    use crate::DynamixelProtocolHandler;
    use crate::Instruction;
    use crate::QueueInterface;
    use core::cell::RefCell;
    use core::time::Duration;
    use heapless::Deque;
    use heapless::Vec;

    pub struct MockSerial {
        rx_buf: Vec<u8, 256>,
        tx_buf: Deque<u8, 256>,
    }
    impl MockSerial {
        pub fn new() -> Self {
            Self {
                rx_buf: Vec::<u8, 256>::new(),
                tx_buf: Deque::<u8, 256>::new(),
            }
        }
    }
    impl crate::BufferInterface for MockSerial {
        fn write_byte(&mut self, data: u8) {
            self.rx_buf.push(data).unwrap();
        }
        fn write_bytes(&mut self, data: &[u8]) {
            for d in data {
                self.rx_buf.push(*d).unwrap();
            }
        }

        fn read_byte(&mut self) -> Option<u8> {
            self.tx_buf.pop_front()
        }

        fn read_bytes(&mut self, buf: &mut [u8]) -> Option<usize> {
            let m = core::cmp::min(self.tx_buf.len(), buf.len());
            for i in 0..m {
                buf[i] = self.tx_buf.pop_front().unwrap();
            }
            Some(m)
        }
        fn clear_read_buf(&mut self) {
            self.tx_buf.clear();
        }
    }

    pub struct MockClock {
        time_elasped: RefCell<Duration>,
    }
    impl MockClock {
        pub fn new() -> Self {
            Self {
                time_elasped: RefCell::new(Duration::new(0, 0)),
            }
        }
        pub fn tick(&mut self) {
            let dt = Duration::from_millis(1);
            self.time_elasped.replace_with(|&mut old| old + dt);
        }
    }
    impl crate::Clock for MockClock {
        fn get_current_time(&self) -> Duration {
            self.time_elasped.clone().into_inner()
        }
    }

    #[test]
    fn empty() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.model_number().bits(0x0406));
        control_table_data.modify(|_, w| w.firmware_version().bits(0x26));
        control_table_data.modify(|_, w| w.id().bits(1));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // コマンド待ち状態になっていることを確認
        assert_eq!(dxl.packet_receiving_state, PacketReceivingState::Waiting);
        assert_eq!(
            dxl.parsing_state,
            ProtocolHandlerParsingState::WaitForCommandPacket
        );

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        assert_eq!(dxl.return_packet(), []);
        assert_eq!(dxl.uart.rx_buf, []);
    }

    #[test]
    fn not_complete_command() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.model_number().bits(0x0406));
        control_table_data.modify(|_, w| w.firmware_version().bits(0x26));
        control_table_data.modify(|_, w| w.id().bits(1));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);

        // 受信するデータのテストケース
        // Ping Instruction Packet ID : 1
        let instruction = [0xFF, 0xFF, 0xFD];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // コマンド待ち状態になっていることを確認
        assert_eq!(dxl.packet_receiving_state, PacketReceivingState::Waiting);
        assert_eq!(
            dxl.parsing_state,
            ProtocolHandlerParsingState::WaitForCommandPacket
        );

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        assert_eq!(dxl.return_packet(), []);
        assert_eq!(dxl.uart.rx_buf, []);

        // 受信するデータのテストケースをここで完成させる
        // Ping Instruction Packet ID : 1
        let instruction = [0x00, 0x01, 0x03, 0x00, 0x01, 0x19, 0x4E];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        assert_eq!(
            dxl.return_packet(),
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D]
        );
        assert_eq!(
            dxl.uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D]
        );


    }

    #[test]
    fn ping() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.model_number().bits(0x0406));
        control_table_data.modify(|_, w| w.firmware_version().bits(0x26));
        control_table_data.modify(|_, w| w.id().bits(1));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);

        // 受信するデータのテストケース
        // Ping Instruction Packet ID : 1
        let instruction = [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x03, 0x00, 0x01, 0x19, 0x4E];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        assert_eq!(
            dxl.return_packet(),
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D]
        );
        assert_eq!(
            dxl.uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D]
        );
    }

    #[test]
    fn ping_broadcast() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.model_number().bits(0x0406));
        control_table_data.modify(|_, w| w.firmware_version().bits(0x26));
        control_table_data.modify(|_, w| w.id().bits(2));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);
        // 受信するデータのテストケース
        // Ping Instruction Packet ID : 254(Broadcast ID)
        let instruction = [0xFF, 0xFF, 0xFD, 0x00, 0xFE, 0x03, 0x00, 0x01, 0x31, 0x42];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }
        // id1が存在する場合をテスト
        let id1_response = [
            0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D,
        ];
        for data in id1_response {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        assert_eq!(
            dxl.return_packet(),
            [0xFF, 0xFF, 0xFD, 0x00, 0x02, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x6F, 0x6D]
        );
        assert_eq!(
            dxl.uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x02, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x6F, 0x6D]
        );
    }

    #[test]
    fn ping_broadcast_id1_not_response() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.model_number().bits(0x0406));
        control_table_data.modify(|_, w| w.firmware_version().bits(0x26));
        control_table_data.modify(|_, w| w.id().bits(2));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);
        // 受信するデータのテストケース
        // Ping Instruction Packet ID : 254(Broadcast ID)
        let instruction = [0xFF, 0xFF, 0xFD, 0x00, 0xFE, 0x03, 0x00, 0x01, 0x31, 0x42];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }
        // id1が存在しない場合のテスト
        // let id1_response = [
        //     0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D,
        // ];
        // for data in id1_response {
        //     dxl.uart.tx_buf.push_back(data).unwrap();
        // }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        assert_eq!(
            dxl.return_packet(),
            [0xFF, 0xFF, 0xFD, 0x00, 0x02, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x6F, 0x6D]
        );
        // データは用意されているがid1が先に返信するのを待つはず
        assert_eq!(
            dxl.uart.rx_buf,
            []
        );

        // 時計を進める
        dxl.clock.tick();
        dxl.clock.tick();

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // 一定時間(1472us以上)経過後には送信しているはず
        assert_eq!(
            dxl.uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x02, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x6F, 0x6D]
        );

    }

    #[test]
    fn read() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.model_number().bits(0x0406));
        control_table_data.modify(|_, w| w.firmware_version().bits(0x26));
        control_table_data.modify(|_, w| w.id().bits(1));
        control_table_data.modify(|_, w| w.present_position().bits(166));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);

        // 受信するデータのテストケース
        // Read Instruction Packet ID: 1, Present Position(132, 0x0084, 4[byte])
        let instruction = [
            0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x02, 0x84, 0x00, 0x04, 0x00, 0x1D, 0x15,
        ];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        assert_eq!(
            dxl.return_packet(),
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x08, 0x00, 0x55, 0x00, 0xA6, 0x00, 0x00, 0x00, 0x8C,
                0xC0
            ]
        );
        assert_eq!(
            dxl.uart.rx_buf,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x08, 0x00, 0x55, 0x00, 0xA6, 0x00, 0x00, 0x00, 0x8C,
                0xC0
            ]
        );
    }

    #[test]
    fn write() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();
        control_table_data.modify(|_, w| w.id().bits(1));

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);

        // 受信するデータのテストケース
        // ID1(XM430-W210) : Write 512(0x00000200) to Goal Position(116, 0x0074, 4[byte])
        let instruction = [
            0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x09, 0x00, 0x03, 0x74, 0x00, 0x00, 0x02, 0x00, 0x00,
            0xCA, 0x89,
        ];
        for data in instruction {
            dxl.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl.parse_data(), Ok(()));

        // control table dataが更新されていること
        assert_eq!(dxl.ctd.read().goal_position(), 512);

        // 返信すべき時間
        assert_eq!(dxl.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        assert_eq!(
            dxl.return_packet(),
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x55, 0x00, 0xA1, 0x0C,]
        );
        assert_eq!(
            dxl.uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x55, 0x00, 0xA1, 0x0C,]
        );
    }

    #[test]
    fn sync_read() {
        let mut mock_uart1 = MockSerial::new();
        let mut mock_uart2 = MockSerial::new();
        let mock_clock1 = MockClock::new();
        let mock_clock2 = MockClock::new();
        let control_table_data1 = ControlTableData::new();
        control_table_data1.modify(|_, w| w.id().bits(1));
        control_table_data1.modify(|_, w| w.present_position().bits(166));
        let control_table_data2 = ControlTableData::new();
        control_table_data2.modify(|_, w| w.id().bits(2));
        control_table_data2.modify(|_, w| w.present_position().bits(2079));

        let mut dxl1 =
            DynamixelProtocolHandler::new(mock_uart1, mock_clock1, 115200, control_table_data1);
        let mut dxl2 =
            DynamixelProtocolHandler::new(mock_uart2, mock_clock2, 115200, control_table_data2);

        // 受信するデータのテストケース
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        // ID2(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 2,079(0x0000081F)
        let instruction = [
            0xFF, 0xFF, 0xFD, 0x00, 0xFE, 0x09, 0x00, 0x82, 0x84, 0x00, 0x04, 0x00, 0x01, 0x02,
            0xCE, 0xFA,
        ];
        for data in instruction {
            dxl1.uart.tx_buf.push_back(data).unwrap();
            dxl2.uart.tx_buf.push_back(data).unwrap();
        }
        let id1_response = [
            0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x08, 0x00, 0x55, 0x00, 0xA6, 0x00, 0x00, 0x00, 0x8C,
            0xC0,
        ];
        // id2の方にはid1のresponseを入れておく
        for data in id1_response {
            dxl2.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl1.parse_data(), Ok(()));
        assert_eq!(dxl2.parse_data(), Ok(()));

        // 返信すべき時間
        assert_eq!(dxl1.packet_return_time(), Duration::new(0, 0));
        assert_eq!(dxl2.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容
        assert_eq!(
            dxl1.return_packet(),
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x08, 0x00, 0x55, 0x00, 0xA6, 0x00, 0x00, 0x00, 0x8C,
                0xC0
            ]
        );
        assert_eq!(
            dxl1.uart.rx_buf,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x08, 0x00, 0x55, 0x00, 0xA6, 0x00, 0x00, 0x00, 0x8C,
                0xC0
            ]
        );
        assert_eq!(
            dxl2.return_packet(),
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x02, 0x08, 0x00, 0x55, 0x00, 0x1F, 0x08, 0x00, 0x00, 0xBA,
                0xBE
            ]
        );
        assert_eq!(
            dxl2.uart.rx_buf,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x02, 0x08, 0x00, 0x55, 0x00, 0x1F, 0x08, 0x00, 0x00, 0xBA,
                0xBE
            ]
        );
    }

    #[test]
    fn sync_write() {
        let mut mock_uart1 = MockSerial::new();
        let mut mock_uart2 = MockSerial::new();
        let mock_clock1 = MockClock::new();
        let mock_clock2 = MockClock::new();
        let control_table_data1 = ControlTableData::new();
        control_table_data1.modify(|_, w| w.id().bits(1));
        let control_table_data2 = ControlTableData::new();
        control_table_data2.modify(|_, w| w.id().bits(2));

        let mut dxl1 =
            DynamixelProtocolHandler::new(mock_uart1, mock_clock1, 115200, control_table_data1);
        let mut dxl2 =
            DynamixelProtocolHandler::new(mock_uart2, mock_clock2, 115200, control_table_data2);

        // 受信するデータのテストケース
        // ID1(XM430-W210) : Write 150(0x00000096) to Goal Position(116, 0x0074, 4[byte])
        // ID2(XM430-W210) : Write 170(0x000000AA) to Goal Position(116, 0x0074, 4[byte])
        let instruction = [
            0xFF, 0xFF, 0xFD, 0x00, 0xFE, 0x11, 0x00, 0x83, 0x74, 0x00, 0x04, 0x00, 0x01, 0x96,
            0x00, 0x00, 0x00, 0x02, 0xAA, 0x00, 0x00, 0x00, 0x82, 0x87,
        ];
        for data in instruction {
            dxl1.uart.tx_buf.push_back(data).unwrap();
            dxl2.uart.tx_buf.push_back(data).unwrap();
        }

        // パースを周期実行
        assert_eq!(dxl1.parse_data(), Ok(()));
        assert_eq!(dxl2.parse_data(), Ok(()));

        // control table dataが更新されていること
        assert_eq!(dxl1.ctd.read().goal_position(), 150);
        assert_eq!(dxl2.ctd.read().goal_position(), 170);

        // 返信すべき時間
        assert_eq!(dxl1.packet_return_time(), Duration::new(0, 0));
        assert_eq!(dxl2.packet_return_time(), Duration::new(0, 0));
        // 返信すべき内容はない
        assert_eq!(dxl1.return_packet(), []);
        assert_eq!(dxl2.return_packet(), []);
        assert!(dxl1.uart.rx_buf.is_empty());
        assert!(dxl2.uart.rx_buf.is_empty());
    }

    #[test]
    fn crc() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();

        let dxl = DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        msg.extend(
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26,
            ]
            .iter()
            .cloned(),
        );
        assert_eq!(dxl.calc_crc_value(&msg), 0x5D65);
    }

    // #[test]
    // #[ignore]
    // fn calc_crc() {
    //     let mut mock_uart = MockSerial::new();
    //     let mock_clock = MockClock::new();
    //     let dxl = DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200);
    //     let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
    //     msg.extend(
    //         [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x06, 0x02]
    //             .iter()
    //             .cloned(),
    //     );
    //     assert_eq!(dxl.calc_crc_value(&msg), 0x0000);
    // }

    #[test]
    fn stuffing() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let control_table_data = ControlTableData::new();

        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        msg.extend(
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x0B, 0x00, 0x03, 0xE0, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFD, 0x01, 0x00, 0x00,
            ]
            .iter()
            .cloned(),
        );
        msg.add_stuffing();
        assert_eq!(
            *msg,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x0C, 0x00, 0x03, 0xE0, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFD, 0xFD, 0x01, 0x00, 0x00
            ]
        );
        msg.remove_stuffing();
        assert_eq!(
            *msg,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x0B, 0x00, 0x03, 0xE0, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFD, 0x01, 0x00, 0x00
            ]
        );
    }

    // #[test]
    // fn clock() {
    //     let mut mock_uart = MockSerial::new();
    //     let mock_clock = MockClock::new();

    //     let mut dxl = DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200);
    //     dxl.set_packet_timeout_length(10);
    //     assert_eq!(dxl.packet_timeout.as_micros(), 4700);
    //     assert_eq!(dxl.is_packet_timeout(), false);
    //     for _ in 0..4 {
    //         mock_clock.tick();
    //     }
    //     assert_eq!(dxl.is_packet_timeout(), false);
    //     mock_clock.tick();
    //     assert_eq!(dxl.is_packet_timeout(), true);
    // }

    #[test]
    fn clear_port() {
        let mut mock_uart = MockSerial::new();
        let control_table_data = ControlTableData::new();

        mock_uart.tx_buf.push_back(1).unwrap();
        mock_uart.tx_buf.push_back(2).unwrap();
        mock_uart.tx_buf.push_back(3).unwrap();
        let mock_clock = MockClock::new();
        let mut dxl =
            DynamixelProtocolHandler::new(mock_uart, mock_clock, 115200, control_table_data);
        dxl.clear_port();
        assert_eq!(dxl.uart.tx_buf.is_empty(), true);
    }
}
