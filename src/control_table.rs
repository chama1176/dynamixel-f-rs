use crate::data_spec::{self, DataSpec};
use core::cell::Cell;
use core::{marker, mem};

#[allow(dead_code)]
pub enum ControlTable {
    ModelNumber,
    ModelInformation,
    FirmwareVersion,
    ID,
    BaudRate,
    ReturnDelayTime,
    DriveMode,
    OperatingMode,
    SecondaryID,
    ProtocolType,
    HomingOffset,
    MovingThreshold,
    TemperatureLimit,
    MaxVoltageLimit,
    MinVoltageLimit,
    PWMLimit,
    CurrentLimit,
    VelocityLimit,
    MaxPositionLimit,
    MinPositionLimit,
    StartupConfiguration,
    PWMSlope,
    Shutdown,
    TorqueEnable,
    LED,
    StatusReturnLevel,
    RegisteredInstruction,
    HardwareErrorStatus,
    VelocityIGain,
    VelocityPgain,
    PositionDGain,
    PositionIGain,
    PositionPGain,
    Feedforward2ndGain,
    Feedforward1stGain,
    BusWatchdog,
    GoalPWM,
    GoalCurrent,
    GoalVelocity,
    ProfileAccleration,
    ProfileVelocity,
    GoalPosition,
    RealtimeTick,
    Moving,
    MovingStatus,
    PresentPWM,
    PresentCurrent,
    PresentVelocity,
    PresentPosition,
    VelocityTrajectory,
    PositionTrajectory,
    PresentInputVoltage,
    PresentTemperature,
    BackupReady,
    IndirectAddress1,
    IndirectAddress2,
    IndirectAddress3,
    IndirectAddress4,
    IndirectAddress5,
    IndirectAddress6,
    IndirectAddress7,
    IndirectAddress8,
    IndirectAddress9,
    IndirectAddress10,
    IndirectAddress11,
    IndirectAddress12,
    IndirectAddress13,
    IndirectAddress14,
    IndirectAddress15,
    IndirectAddress16,
    IndirectAddress17,
    IndirectAddress18,
    IndirectAddress19,
    IndirectAddress20,
    IndirectData1,
    IndirectData2,
    IndirectData3,
    IndirectData4,
    IndirectData5,
    IndirectData6,
    IndirectData7,
    IndirectData8,
    IndirectData9,
    IndirectData10,
    IndirectData11,
    IndirectData12,
    IndirectData13,
    IndirectData14,
    IndirectData15,
    IndirectData16,
    IndirectData17,
    IndirectData18,
    IndirectData19,
    IndirectData20,
}

pub trait CustomInt<const N: usize> {
    type Ty;
}
impl CustomInt<{ ControlTable::ModelNumber as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::ModelInformation as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::FirmwareVersion as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::ID as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::BaudRate as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::ReturnDelayTime as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::DriveMode as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::OperatingMode as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::SecondaryID as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::ProtocolType as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::HomingOffset as usize }> for () {
    type Ty = i32;
}
impl CustomInt<{ ControlTable::MovingThreshold as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::TemperatureLimit as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::MaxVoltageLimit as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::MinVoltageLimit as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::PWMLimit as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::CurrentLimit as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::VelocityLimit as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::MaxPositionLimit as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::MinPositionLimit as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::StartupConfiguration as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::PWMSlope as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::Shutdown as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::TorqueEnable as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::LED as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::StatusReturnLevel as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::RegisteredInstruction as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::HardwareErrorStatus as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::VelocityIGain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::VelocityPgain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::PositionDGain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::PositionIGain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::PositionPGain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::Feedforward2ndGain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::Feedforward1stGain as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::BusWatchdog as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::GoalPWM as usize }> for () {
    type Ty = i16;
}
impl CustomInt<{ ControlTable::GoalCurrent as usize }> for () {
    type Ty = i16;
}
impl CustomInt<{ ControlTable::GoalVelocity as usize }> for () {
    type Ty = i32;
}
impl CustomInt<{ ControlTable::ProfileAccleration as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::ProfileVelocity as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::GoalPosition as usize }> for () {
    type Ty = u32;
}
impl CustomInt<{ ControlTable::RealtimeTick as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::Moving as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::MovingStatus as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::PresentPWM as usize }> for () {
    type Ty = i16;
}
impl CustomInt<{ ControlTable::PresentCurrent as usize }> for () {
    type Ty = i16;
}
impl CustomInt<{ ControlTable::PresentVelocity as usize }> for () {
    type Ty = i32;
}
impl CustomInt<{ ControlTable::PresentPosition as usize }> for () {
    type Ty = i32;
}
impl CustomInt<{ ControlTable::VelocityTrajectory as usize }> for () {
    type Ty = i32;
}
impl CustomInt<{ ControlTable::PositionTrajectory as usize }> for () {
    type Ty = i32;
}
impl CustomInt<{ ControlTable::PresentInputVoltage as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::PresentTemperature as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::BackupReady as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectAddress1 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress2 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress3 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress4 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress5 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress6 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress7 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress8 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress9 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress10 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress11 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress12 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress13 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress14 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress15 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress16 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress17 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress18 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress19 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectAddress20 as usize }> for () {
    type Ty = u16;
}
impl CustomInt<{ ControlTable::IndirectData1 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData2 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData3 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData4 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData5 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData6 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData7 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData8 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData9 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData10 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData11 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData12 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData13 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData14 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData15 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData16 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData17 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData18 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData19 as usize }> for () {
    type Ty = u8;
}
impl CustomInt<{ ControlTable::IndirectData20 as usize }> for () {
    type Ty = u8;
}

macro_rules! size_of {
    ($input:expr) => {
        core::mem::size_of::<<() as CustomInt<{ $input as usize }>>::Ty>() as u16
    };
}

#[allow(dead_code)]
impl ControlTable {
    pub fn to_address(&self) -> u16 {
        match self {
            ControlTable::ModelNumber => 0,
            ControlTable::ModelInformation => 2,
            ControlTable::FirmwareVersion => 6,
            ControlTable::ID => 7,
            ControlTable::BaudRate => 8,
            ControlTable::ReturnDelayTime => 9,
            ControlTable::DriveMode => 10,
            ControlTable::OperatingMode => 11,
            ControlTable::SecondaryID => 12,
            ControlTable::ProtocolType => 13,
            ControlTable::HomingOffset => 20,
            ControlTable::MovingThreshold => 24,
            ControlTable::TemperatureLimit => 31,
            ControlTable::MaxVoltageLimit => 32,
            ControlTable::MinVoltageLimit => 34,
            ControlTable::PWMLimit => 36,
            ControlTable::CurrentLimit => 38,
            ControlTable::VelocityLimit => 44,
            ControlTable::MaxPositionLimit => 48,
            ControlTable::MinPositionLimit => 52,
            ControlTable::StartupConfiguration => 60,
            ControlTable::PWMSlope => 62,
            ControlTable::Shutdown => 63,
            ControlTable::TorqueEnable => 64,
            ControlTable::LED => 65,
            ControlTable::StatusReturnLevel => 68,
            ControlTable::RegisteredInstruction => 69,
            ControlTable::HardwareErrorStatus => 70,
            ControlTable::VelocityIGain => 76,
            ControlTable::VelocityPgain => 78,
            ControlTable::PositionDGain => 80,
            ControlTable::PositionIGain => 82,
            ControlTable::PositionPGain => 84,
            ControlTable::Feedforward2ndGain => 88,
            ControlTable::Feedforward1stGain => 90,
            ControlTable::BusWatchdog => 98,
            ControlTable::GoalPWM => 100,
            ControlTable::GoalCurrent => 102,
            ControlTable::GoalVelocity => 104,
            ControlTable::ProfileAccleration => 108,
            ControlTable::ProfileVelocity => 112,
            ControlTable::GoalPosition => 116,
            ControlTable::RealtimeTick => 120,
            ControlTable::Moving => 122,
            ControlTable::MovingStatus => 123,
            ControlTable::PresentPWM => 124,
            ControlTable::PresentCurrent => 126,
            ControlTable::PresentVelocity => 128,
            ControlTable::PresentPosition => 132,
            ControlTable::VelocityTrajectory => 136,
            ControlTable::PositionTrajectory => 140,
            ControlTable::PresentInputVoltage => 144,
            ControlTable::PresentTemperature => 146,
            ControlTable::BackupReady => 147,
            ControlTable::IndirectAddress1 => 168,
            ControlTable::IndirectAddress2 => 170,
            ControlTable::IndirectAddress3 => 172,
            ControlTable::IndirectAddress4 => 174,
            ControlTable::IndirectAddress5 => 176,
            ControlTable::IndirectAddress6 => 178,
            ControlTable::IndirectAddress7 => 180,
            ControlTable::IndirectAddress8 => 182,
            ControlTable::IndirectAddress9 => 184,
            ControlTable::IndirectAddress10 => 186,
            ControlTable::IndirectAddress11 => 188,
            ControlTable::IndirectAddress12 => 190,
            ControlTable::IndirectAddress13 => 192,
            ControlTable::IndirectAddress14 => 194,
            ControlTable::IndirectAddress15 => 196,
            ControlTable::IndirectAddress16 => 198,
            ControlTable::IndirectAddress17 => 200,
            ControlTable::IndirectAddress18 => 202,
            ControlTable::IndirectAddress19 => 204,
            ControlTable::IndirectAddress20 => 206,
            ControlTable::IndirectData1 => 208,
            ControlTable::IndirectData2 => 209,
            ControlTable::IndirectData3 => 210,
            ControlTable::IndirectData4 => 211,
            ControlTable::IndirectData5 => 212,
            ControlTable::IndirectData6 => 213,
            ControlTable::IndirectData7 => 214,
            ControlTable::IndirectData8 => 215,
            ControlTable::IndirectData9 => 216,
            ControlTable::IndirectData10 => 217,
            ControlTable::IndirectData11 => 218,
            ControlTable::IndirectData12 => 219,
            ControlTable::IndirectData13 => 220,
            ControlTable::IndirectData14 => 221,
            ControlTable::IndirectData15 => 222,
            ControlTable::IndirectData16 => 223,
            ControlTable::IndirectData17 => 224,
            ControlTable::IndirectData18 => 225,
            ControlTable::IndirectData19 => 226,
            ControlTable::IndirectData20 => 227,
        }
    }

    pub fn to_size(&self) -> u16 {
        match self {
            ControlTable::ModelNumber => 2,
            ControlTable::ModelInformation => 4,
            ControlTable::FirmwareVersion => 1,
            ControlTable::ID => 1,
            ControlTable::BaudRate => 1,
            ControlTable::ReturnDelayTime => 1,
            ControlTable::DriveMode => 1,
            ControlTable::OperatingMode => 1,
            ControlTable::SecondaryID => 1,
            ControlTable::ProtocolType => 1,
            ControlTable::HomingOffset => 4,
            ControlTable::MovingThreshold => 4,
            ControlTable::TemperatureLimit => 1,
            ControlTable::MaxVoltageLimit => 2,
            ControlTable::MinVoltageLimit => 2,
            ControlTable::PWMLimit => 2,
            ControlTable::CurrentLimit => 2,
            ControlTable::VelocityLimit => 4,
            ControlTable::MaxPositionLimit => 4,
            ControlTable::MinPositionLimit => 4,
            ControlTable::StartupConfiguration => 1,
            ControlTable::PWMSlope => 1,
            ControlTable::Shutdown => 1,
            ControlTable::TorqueEnable => 1,
            ControlTable::LED => 1,
            ControlTable::StatusReturnLevel => 1,
            ControlTable::RegisteredInstruction => 1,
            ControlTable::HardwareErrorStatus => 1,
            ControlTable::VelocityIGain => 2,
            ControlTable::VelocityPgain => 2,
            ControlTable::PositionDGain => 2,
            ControlTable::PositionIGain => 2,
            ControlTable::PositionPGain => 2,
            ControlTable::Feedforward2ndGain => 2,
            ControlTable::Feedforward1stGain => 2,
            ControlTable::BusWatchdog => 1,
            ControlTable::GoalPWM => 2,
            ControlTable::GoalCurrent => 2,
            ControlTable::GoalVelocity => 4,
            ControlTable::ProfileAccleration => 4,
            ControlTable::ProfileVelocity => 4,
            ControlTable::GoalPosition => 4,
            ControlTable::RealtimeTick => 2,
            ControlTable::Moving => 1,
            ControlTable::MovingStatus => 1,
            ControlTable::PresentPWM => 2,
            ControlTable::PresentCurrent => 2,
            ControlTable::PresentVelocity => 4,
            ControlTable::PresentPosition => 4,
            ControlTable::VelocityTrajectory => 4,
            ControlTable::PositionTrajectory => 4,
            ControlTable::PresentInputVoltage => 2,
            ControlTable::PresentTemperature => 1,
            ControlTable::BackupReady => 1,
            ControlTable::IndirectAddress1 => 2,
            ControlTable::IndirectAddress2 => 2,
            ControlTable::IndirectAddress3 => 2,
            ControlTable::IndirectAddress4 => 2,
            ControlTable::IndirectAddress5 => 2,
            ControlTable::IndirectAddress6 => 2,
            ControlTable::IndirectAddress7 => 2,
            ControlTable::IndirectAddress8 => 2,
            ControlTable::IndirectAddress9 => 2,
            ControlTable::IndirectAddress10 => 2,
            ControlTable::IndirectAddress11 => 2,
            ControlTable::IndirectAddress12 => 2,
            ControlTable::IndirectAddress13 => 2,
            ControlTable::IndirectAddress14 => 2,
            ControlTable::IndirectAddress15 => 2,
            ControlTable::IndirectAddress16 => 2,
            ControlTable::IndirectAddress17 => 2,
            ControlTable::IndirectAddress18 => 2,
            ControlTable::IndirectAddress19 => 2,
            ControlTable::IndirectAddress20 => 2,
            ControlTable::IndirectData1 => 1,
            ControlTable::IndirectData2 => 1,
            ControlTable::IndirectData3 => 1,
            ControlTable::IndirectData4 => 1,
            ControlTable::IndirectData5 => 1,
            ControlTable::IndirectData6 => 1,
            ControlTable::IndirectData7 => 1,
            ControlTable::IndirectData8 => 1,
            ControlTable::IndirectData9 => 1,
            ControlTable::IndirectData10 => 1,
            ControlTable::IndirectData11 => 1,
            ControlTable::IndirectData12 => 1,
            ControlTable::IndirectData13 => 1,
            ControlTable::IndirectData14 => 1,
            ControlTable::IndirectData15 => 1,
            ControlTable::IndirectData16 => 1,
            ControlTable::IndirectData17 => 1,
            ControlTable::IndirectData18 => 1,
            ControlTable::IndirectData19 => 1,
            ControlTable::IndirectData20 => 1,
        }
    }
}

type Ux = [u8; 8];
pub struct ControlTableData {
    value: Cell<Ux>,
    // value: [Cell<u8>; 8],だと要素ごとに.getしないといけないのが大変そうなので上で進めてみる
}

impl ControlTableData {
    pub fn new() -> Self {
        Self {
            value: Cell::new([0; 8]),
        }
    }
    pub fn read(&self) -> R {
        R {
            bits: self.value.get(),
        }
    }
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.value.get();
        self.value.set(f(&R { bits }, &mut W { bits }).bits);
    }

    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.value.set(f(&mut W { bits: [0; 8] }).bits);
    }
}

/// Register reader.
///
/// Result of the `read` methods of registers. Also used as a closure argument in the `modify`
/// method.
pub struct R {
    bits: Ux,
}

trait ParseData<T> {
    fn to_data(&self, ct: ControlTable) -> T;
}

impl ParseData<u8> for R {
    fn to_data(&self, ct: ControlTable) -> u8 {
        self.bits[ct.to_address() as usize]
    }
}
impl ParseData<u16> for R {
    fn to_data(&self, ct: ControlTable) -> u16 {
        u16::from_le_bytes([
            self.bits[ct.to_address() as usize],
            self.bits[ct.to_address() as usize + 1],
        ])
    }
}

impl ParseData<i16> for R {
    fn to_data(&self, ct: ControlTable) -> i16 {
        i16::from_le_bytes([
            self.bits[ct.to_address() as usize],
            self.bits[ct.to_address() as usize + 1],
        ])
    }
}

impl ParseData<u32> for R {
    fn to_data(&self, ct: ControlTable) -> u32 {
        u32::from_le_bytes([
            self.bits[ct.to_address() as usize],
            self.bits[ct.to_address() as usize + 1],
            self.bits[ct.to_address() as usize + 2],
            self.bits[ct.to_address() as usize + 3],
        ])
    }
}

impl ParseData<i32> for R {
    fn to_data(&self, ct: ControlTable) -> i32 {
        i32::from_le_bytes([
            self.bits[ct.to_address() as usize],
            self.bits[ct.to_address() as usize + 1],
            self.bits[ct.to_address() as usize + 2],
            self.bits[ct.to_address() as usize + 3],
        ])
    }
}

impl R {
    /// Reads raw bits from register.
    #[inline(always)]
    pub fn bits(&self) -> Ux {
        self.bits
    }
    pub fn model_number(&self) -> <() as CustomInt<{ ControlTable::ModelNumber as usize }>>::Ty {
        self.to_data(ControlTable::ModelNumber)
    }
    pub fn model_information(
        &self,
    ) -> <() as CustomInt<{ ControlTable::ModelInformation as usize }>>::Ty {
        self.to_data(ControlTable::ModelInformation)
    }
    pub fn firmware_version(
        &self,
    ) -> <() as CustomInt<{ ControlTable::FirmwareVersion as usize }>>::Ty {
        self.to_data(ControlTable::FirmwareVersion)
    }
    pub fn id(&self) -> <() as CustomInt<{ ControlTable::ID as usize }>>::Ty {
        self.to_data(ControlTable::ID)
    }
    pub fn baud_rate(&self) -> <() as CustomInt<{ ControlTable::BaudRate as usize }>>::Ty {
        self.to_data(ControlTable::BaudRate)
    }
    pub fn return_delay_time(&self) -> <() as CustomInt<{ ControlTable::ReturnDelayTime as usize }>>::Ty {
        self.to_data(ControlTable::ReturnDelayTime)
    }
    pub fn drive_mode(&self) -> <() as CustomInt<{ ControlTable::DriveMode as usize }>>::Ty {
        self.to_data(ControlTable::DriveMode)
    }
    pub fn operating_mode(&self) -> <() as CustomInt<{ ControlTable::OperatingMode as usize }>>::Ty {
        self.to_data(ControlTable::OperatingMode)
    }
    pub fn secondary_id(&self) -> <() as CustomInt<{ ControlTable::SecondaryID as usize }>>::Ty {
        self.to_data(ControlTable::SecondaryID)
    }
    pub fn protocol_type(&self) -> <() as CustomInt<{ ControlTable::ProtocolType as usize }>>::Ty {
        self.to_data(ControlTable::ProtocolType)
    }
    pub fn homing_offset(&self) -> <() as CustomInt<{ ControlTable::HomingOffset as usize }>>::Ty {
        self.to_data(ControlTable::HomingOffset)
    }
    pub fn moving_threshold(&self) -> <() as CustomInt<{ ControlTable::MovingThreshold as usize }>>::Ty {
        self.to_data(ControlTable::MovingThreshold)
    }
    pub fn temperature_limit(&self) -> <() as CustomInt<{ ControlTable::TemperatureLimit as usize }>>::Ty {
        self.to_data(ControlTable::TemperatureLimit)
    }
    pub fn max_voltage_limit(&self) -> <() as CustomInt<{ ControlTable::MaxVoltageLimit as usize }>>::Ty {
        self.to_data(ControlTable::MaxVoltageLimit)
    }
    pub fn min_voltage_limit(&self) -> <() as CustomInt<{ ControlTable::MinVoltageLimit as usize }>>::Ty {
        self.to_data(ControlTable::MinVoltageLimit)
    }
    pub fn pwm_limit(&self) -> <() as CustomInt<{ ControlTable::PWMLimit as usize }>>::Ty {
        self.to_data(ControlTable::PWMLimit)
    }
    pub fn current_limit(&self) -> <() as CustomInt<{ ControlTable::CurrentLimit as usize }>>::Ty {
        self.to_data(ControlTable::CurrentLimit)
    }
    pub fn velocity_limit(&self) -> <() as CustomInt<{ ControlTable::VelocityLimit as usize }>>::Ty {
        self.to_data(ControlTable::VelocityLimit)
    }
    pub fn max_position_limit(&self) -> <() as CustomInt<{ ControlTable::MaxPositionLimit as usize }>>::Ty {
        self.to_data(ControlTable::MaxPositionLimit)
    }
    pub fn min_position_limit(&self) -> <() as CustomInt<{ ControlTable::MinPositionLimit as usize }>>::Ty {
        self.to_data(ControlTable::MinPositionLimit)
    }
    pub fn startup_configuration(&self) -> <() as CustomInt<{ ControlTable::StartupConfiguration as usize }>>::Ty {
        self.to_data(ControlTable::StartupConfiguration)
    }
    pub fn pwm_slope(&self) -> <() as CustomInt<{ ControlTable::PWMSlope as usize }>>::Ty {
        self.to_data(ControlTable::PWMSlope)
    }
    pub fn shutdown(&self) -> <() as CustomInt<{ ControlTable::Shutdown as usize }>>::Ty {
        self.to_data(ControlTable::Shutdown)
    }
    pub fn torque_enable(&self) -> <() as CustomInt<{ ControlTable::TorqueEnable as usize }>>::Ty {
        self.to_data(ControlTable::TorqueEnable)
    }
    pub fn led(&self) -> <() as CustomInt<{ ControlTable::LED as usize }>>::Ty {
        self.to_data(ControlTable::LED)
    }
    pub fn status_return_level(&self) -> <() as CustomInt<{ ControlTable::StatusReturnLevel as usize }>>::Ty {
        self.to_data(ControlTable::StatusReturnLevel)
    }
    pub fn registered_instruction(&self) -> <() as CustomInt<{ ControlTable::RegisteredInstruction as usize }>>::Ty {
        self.to_data(ControlTable::RegisteredInstruction)
    }
    pub fn hardware_error_status(&self) -> <() as CustomInt<{ ControlTable::HardwareErrorStatus as usize }>>::Ty {
        self.to_data(ControlTable::HardwareErrorStatus)
    }
    pub fn velocity_igain(&self) -> <() as CustomInt<{ ControlTable::VelocityIGain as usize }>>::Ty {
        self.to_data(ControlTable::VelocityIGain)
    }
    pub fn velocity_pgain(&self) -> <() as CustomInt<{ ControlTable::VelocityPgain as usize }>>::Ty {
        self.to_data(ControlTable::VelocityPgain)
    }
    pub fn position_dgain(&self) -> <() as CustomInt<{ ControlTable::PositionDGain as usize }>>::Ty {
        self.to_data(ControlTable::PositionDGain)
    }
    pub fn position_igain(&self) -> <() as CustomInt<{ ControlTable::PositionIGain as usize }>>::Ty {
        self.to_data(ControlTable::PositionIGain)
    }
    pub fn position_pgain(&self) -> <() as CustomInt<{ ControlTable::PositionPGain as usize }>>::Ty {
        self.to_data(ControlTable::PositionPGain)
    }
    pub fn feedforward2nd_gain(&self) -> <() as CustomInt<{ ControlTable::Feedforward2ndGain as usize }>>::Ty {
        self.to_data(ControlTable::Feedforward2ndGain)
    }
    pub fn feedforward1st_gain(&self) -> <() as CustomInt<{ ControlTable::Feedforward1stGain as usize }>>::Ty {
        self.to_data(ControlTable::Feedforward1stGain)
    }
    pub fn bus_watchdog(&self) -> <() as CustomInt<{ ControlTable::BusWatchdog as usize }>>::Ty {
        self.to_data(ControlTable::BusWatchdog)
    }
    pub fn goal_pwm(&self) -> <() as CustomInt<{ ControlTable::GoalPWM as usize }>>::Ty {
        self.to_data(ControlTable::GoalPWM)
    }
    pub fn goal_current(&self) -> <() as CustomInt<{ ControlTable::GoalCurrent as usize }>>::Ty {
        self.to_data(ControlTable::GoalCurrent)
    }
    pub fn goal_velocity(&self) -> <() as CustomInt<{ ControlTable::GoalVelocity as usize }>>::Ty {
        self.to_data(ControlTable::GoalVelocity)
    }
    pub fn profile_accleration(&self) -> <() as CustomInt<{ ControlTable::ProfileAccleration as usize }>>::Ty {
        self.to_data(ControlTable::ProfileAccleration)
    }
    pub fn profile_velocity(&self) -> <() as CustomInt<{ ControlTable::ProfileVelocity as usize }>>::Ty {
        self.to_data(ControlTable::ProfileVelocity)
    }
    pub fn goal_position(&self) -> <() as CustomInt<{ ControlTable::GoalPosition as usize }>>::Ty {
        self.to_data(ControlTable::GoalPosition)
    }
    pub fn realtime_tick(&self) -> <() as CustomInt<{ ControlTable::RealtimeTick as usize }>>::Ty {
        self.to_data(ControlTable::RealtimeTick)
    }
    pub fn moving(&self) -> <() as CustomInt<{ ControlTable::Moving as usize }>>::Ty {
        self.to_data(ControlTable::Moving)
    }
    pub fn moving_status(&self) -> <() as CustomInt<{ ControlTable::MovingStatus as usize }>>::Ty {
        self.to_data(ControlTable::MovingStatus)
    }
    pub fn present_pwm(&self) -> <() as CustomInt<{ ControlTable::PresentPWM as usize }>>::Ty {
        self.to_data(ControlTable::PresentPWM)
    }
    pub fn present_current(&self) -> <() as CustomInt<{ ControlTable::PresentCurrent as usize }>>::Ty {
        self.to_data(ControlTable::PresentCurrent)
    }
    pub fn present_velocity(&self) -> <() as CustomInt<{ ControlTable::PresentVelocity as usize }>>::Ty {
        self.to_data(ControlTable::PresentVelocity)
    }
    pub fn present_position(&self) -> <() as CustomInt<{ ControlTable::PresentPosition as usize }>>::Ty {
        self.to_data(ControlTable::PresentPosition)
    }
    pub fn velocity_trajectory(&self) -> <() as CustomInt<{ ControlTable::VelocityTrajectory as usize }>>::Ty {
        self.to_data(ControlTable::VelocityTrajectory)
    }
    pub fn position_trajectory(&self) -> <() as CustomInt<{ ControlTable::PositionTrajectory as usize }>>::Ty {
        self.to_data(ControlTable::PositionTrajectory)
    }
    pub fn present_input_voltage(&self) -> <() as CustomInt<{ ControlTable::PresentInputVoltage as usize }>>::Ty {
        self.to_data(ControlTable::PresentInputVoltage)
    }
    pub fn present_temperature(&self) -> <() as CustomInt<{ ControlTable::PresentTemperature as usize }>>::Ty {
        self.to_data(ControlTable::PresentTemperature)
    }
    pub fn backup_ready(&self) -> <() as CustomInt<{ ControlTable::BackupReady as usize }>>::Ty {
        self.to_data(ControlTable::BackupReady)
    }
    pub fn indirect_address1(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress1 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress1)
    }
    pub fn indirect_address2(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress2 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress2)
    }
    pub fn indirect_address3(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress3 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress3)
    }
    pub fn indirect_address4(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress4 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress4)
    }
    pub fn indirect_address5(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress5 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress5)
    }
    pub fn indirect_address6(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress6 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress6)
    }
    pub fn indirect_address7(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress7 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress7)
    }
    pub fn indirect_address8(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress8 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress8)
    }
    pub fn indirect_address9(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress9 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress9)
    }
    pub fn indirect_address10(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress10 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress10)
    }
    pub fn indirect_address11(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress11 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress11)
    }
    pub fn indirect_address12(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress12 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress12)
    }
    pub fn indirect_address13(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress13 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress13)
    }
    pub fn indirect_address14(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress14 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress14)
    }
    pub fn indirect_address15(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress15 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress15)
    }
    pub fn indirect_address16(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress16 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress16)
    }
    pub fn indirect_address17(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress17 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress17)
    }
    pub fn indirect_address18(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress18 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress18)
    }
    pub fn indirect_address19(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress19 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress19)
    }
    pub fn indirect_address20(&self) -> <() as CustomInt<{ ControlTable::IndirectAddress1 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectAddress1)
    }
    pub fn indirect_data1(&self) -> <() as CustomInt<{ ControlTable::IndirectData1 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData1)
    }
    pub fn indirect_data2(&self) -> <() as CustomInt<{ ControlTable::IndirectData2 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData2)
    }
    pub fn indirect_data3(&self) -> <() as CustomInt<{ ControlTable::IndirectData3 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData3)
    }
    pub fn indirect_data4(&self) -> <() as CustomInt<{ ControlTable::IndirectData4 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData4)
    }
    pub fn indirect_data5(&self) -> <() as CustomInt<{ ControlTable::IndirectData5 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData5)
    }
    pub fn indirect_data6(&self) -> <() as CustomInt<{ ControlTable::IndirectData6 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData6)
    }
    pub fn indirect_data7(&self) -> <() as CustomInt<{ ControlTable::IndirectData7 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData7)
    }
    pub fn indirect_data8(&self) -> <() as CustomInt<{ ControlTable::IndirectData8 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData8)
    }
    pub fn indirect_data9(&self) -> <() as CustomInt<{ ControlTable::IndirectData9 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData9)
    }
    pub fn indirect_data10(&self) -> <() as CustomInt<{ ControlTable::IndirectData10 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData10)
    }
    pub fn indirect_data11(&self) -> <() as CustomInt<{ ControlTable::IndirectData11 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData11)
    }
    pub fn indirect_data12(&self) -> <() as CustomInt<{ ControlTable::IndirectData12 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData12)
    }
    pub fn indirect_data13(&self) -> <() as CustomInt<{ ControlTable::IndirectData13 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData13)
    }
    pub fn indirect_data14(&self) -> <() as CustomInt<{ ControlTable::IndirectData14 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData14)
    }
    pub fn indirect_data15(&self) -> <() as CustomInt<{ ControlTable::IndirectData15 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData15)
    }
    pub fn indirect_data16(&self) -> <() as CustomInt<{ ControlTable::IndirectData16 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData16)
    }
    pub fn indirect_data17(&self) -> <() as CustomInt<{ ControlTable::IndirectData17 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData17)
    }
    pub fn indirect_data18(&self) -> <() as CustomInt<{ ControlTable::IndirectData18 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData18)
    }
    pub fn indirect_data19(&self) -> <() as CustomInt<{ ControlTable::IndirectData19 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData19)
    }
    pub fn indirect_data20(&self) -> <() as CustomInt<{ ControlTable::IndirectData20 as usize }>>::Ty {
        self.to_data(ControlTable::IndirectData20)
    }

}

/// Register writer.
///
/// Used as an argument to the closures in the `write` and `modify` methods of the register.
pub struct W {
    ///Writable bits
    bits: Ux,
}

impl W {
    /// Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: Ux) -> &mut Self {
        self.bits = bits;
        self
    }
    pub fn model_number(
        &mut self,
    ) -> BaseW<<() as CustomInt<{ ControlTable::ModelNumber as usize }>>::Ty> {
        BaseW {
            w: self,
            ct: ControlTable::ModelNumber,
            _type: marker::PhantomData,
        }
    }
    pub fn model_information(
        &mut self,
    ) -> BaseW<<() as CustomInt<{ ControlTable::ModelInformation as usize }>>::Ty> {
        BaseW {
            w: self,
            ct: ControlTable::ModelInformation,
            _type: marker::PhantomData,
        }
    }
    pub fn firmware_version(
        &mut self,
    ) -> BaseW<<() as CustomInt<{ ControlTable::FirmwareVersion as usize }>>::Ty> {
        BaseW {
            w: self,
            ct: ControlTable::FirmwareVersion,
            _type: marker::PhantomData,
        }
    }
    pub fn id(&mut self) -> BaseW<<() as CustomInt<{ ControlTable::ID as usize }>>::Ty> {
        BaseW {
            w: self,
            ct: ControlTable::ID,
            _type: marker::PhantomData,
        }
    }


}

pub struct BaseW<'a, T> {
    w: &'a mut W,
    ct: ControlTable,
    _type: marker::PhantomData<T>,
}

pub trait BitsW<'a, P> {
    fn bits(self, value: P) -> &'a mut W;
}

impl<'a> BitsW<'a, u8> for BaseW<'a, u8> {
    #[inline(always)]
    fn bits(self, value: u8) -> &'a mut W {
        self.w.bits[self.ct.to_address() as usize] = value;
        self.w
    }
}

impl<'a> BitsW<'a, u16> for BaseW<'a, u16> {
    #[inline(always)]
    fn bits(self, value: u16) -> &'a mut W {
        let v = value.to_le_bytes();
        self.w.bits[self.ct.to_address() as usize] = v[0];
        self.w.bits[self.ct.to_address() as usize + 1] = v[1];
        self.w
    }
}

impl<'a> BitsW<'a, i16> for BaseW<'a, i16> {
    #[inline(always)]
    fn bits(self, value: i16) -> &'a mut W {
        let v = value.to_le_bytes();
        self.w.bits[self.ct.to_address() as usize] = v[0];
        self.w.bits[self.ct.to_address() as usize + 1] = v[1];
        self.w
    }
}

impl<'a> BitsW<'a, u32> for BaseW<'a, u32> {
    #[inline(always)]
    fn bits(self, value: u32) -> &'a mut W {
        let v = value.to_le_bytes();
        self.w.bits[self.ct.to_address() as usize] = v[0];
        self.w.bits[self.ct.to_address() as usize + 1] = v[1];
        self.w.bits[self.ct.to_address() as usize + 2] = v[2];
        self.w.bits[self.ct.to_address() as usize + 3] = v[3];
        self.w
    }
}

impl<'a> BitsW<'a, i32> for BaseW<'a, i32> {
    #[inline(always)]
    fn bits(self, value: i32) -> &'a mut W {
        let v = value.to_le_bytes();
        self.w.bits[self.ct.to_address() as usize] = v[0];
        self.w.bits[self.ct.to_address() as usize + 1] = v[1];
        self.w.bits[self.ct.to_address() as usize + 2] = v[2];
        self.w.bits[self.ct.to_address() as usize + 3] = v[3];
        self.w
    }
}

#[cfg(test)]
mod tests {
    use crate::control_table::{BitsW, ControlTable, ControlTableData, W};
    use crate::control_table::CustomInt;

    #[test]
    fn to_address() {
        let name = ControlTable::ModelNumber;
        assert_eq!(name.to_address(), 0);
        assert_eq!(ControlTable::TorqueEnable.to_address(), 64)
    }

    #[test]
    fn to_size() {
        let name = ControlTable::ModelNumber;
        assert_eq!(name.to_size(), 2);
        assert_eq!(ControlTable::ModelInformation.to_size(), 4)
    }

    #[test]
    fn to_size_and_type() {
        const NAME: ControlTable = ControlTable::ModelNumber;
        assert_eq!(NAME.to_size(), size_of!(NAME));
    }

    #[test]
    fn read() {
        let ctd = ControlTableData::new();
        assert_eq!(ctd.read().bits(), [0; 8])
    }

    #[test]
    fn write() {
        let ctd = ControlTableData::new();
        ctd.write(|w| w.bits([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(ctd.read().bits(), [1, 2, 3, 4, 5, 6, 7, 8])
    }

    #[test]
    fn modify() {
        let ctd = ControlTableData::new();
        ctd.write(|w| w.bits([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(ctd.read().bits(), [1, 2, 3, 4, 5, 6, 7, 8]);
        ctd.modify(|_, w| w.bits([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(ctd.read().bits(), [1, 2, 3, 4, 5, 6, 7, 8]);
        ctd.write(|w| w.id().bits(2));
        assert_eq!(ctd.read().bits(), [0, 0, 0, 0, 0, 0, 0, 2]);
        ctd.write(|w| w.model_number().bits(0x4321));
        assert_eq!(ctd.read().bits(), [0x21, 0x43, 0, 0, 0, 0, 0, 0]);
        assert_eq!(ctd.read().model_number(), 0x4321);
        ctd.modify(|_, w| w.id().bits(2));
        assert_eq!(ctd.read().bits(), [0x21, 0x43, 0, 0, 0, 0, 0, 2]);

        // TODO: テストをもう少し修正した方がよい
    }
}
