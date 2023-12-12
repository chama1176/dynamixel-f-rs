use crate::data_spec::{self, DataSpec};
use core::cell::Cell;

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

impl R {
    /// Reads raw bits from register.
    #[inline(always)]
    pub fn bits(&self) -> Ux {
        self.bits
    }
    pub fn id(&self) -> u8 {
        self.bits[ControlTable::ID.to_address() as usize]
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
    pub fn id(&mut self) -> IdW {
        IdW {
            w: self,
        }
    }
}

pub struct IdW<'a> {
    w: &'a mut W,
}
impl<'a> IdW<'a> {
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits[ControlTable::ID.to_address() as usize] = value;
        self.w
    }
}

pub struct ModelNumberW<'a> {
    w: &'a mut W,
}
impl<'a> ModelNumberW<'a> {
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits[ControlTable::ModelNumber.to_address() as usize] = value;
        self.w
    }
}

pub trait ControlTableW<'a, T> {
    const CT: ControlTable;
    fn bits(self, value: T) -> &'a mut W;
}
pub struct BaseW<'a> {
    w: &'a mut W,
}
impl<'a> ControlTableW<'a, u8> for BaseW<'a> {
    const CT: ControlTable = ControlTable::ModelNumber;
    #[inline(always)]
    fn bits(self, value: u8) -> &'a mut W {
        self.w.bits[Self::CT.to_address() as usize] = value;
        self.w
    }
}


#[cfg(test)]
mod tests {
    use crate::control_table::{ControlTable, ControlTableData, W};

    #[test]
    fn to_address() {
        let name = ControlTable::ModelNumber;
        assert_eq!(name.to_address(), 0);
        assert_eq!(ControlTable::TorqueEnable.to_address(), 64)
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

        // TODO: テストをもう少し修正した方がよい
    }
}
