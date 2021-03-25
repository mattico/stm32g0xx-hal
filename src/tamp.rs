//! Tamper Detection and Backup Registers
use crate::rcc::Rcc;
use crate::rtc::Rtc;
use crate::stm32::TAMP;

pub struct Tamp {
    _rb: TAMP,
    pub backup: [BackupRegister; 5],
}

impl Tamp {
    /// Constrain the TAMP peripheral
    ///
    /// Note that if the RTC is used initializing it resets the entire backup domain,
    /// including the tamper detection and backup registers.
    pub fn new(tamp: TAMP, rcc: &mut Rcc) -> Self {
        rcc.unlock_backup_domain();
        Tamp {
            _rb: tamp,
            backup: [
                BackupRegister { i: 0 },
                BackupRegister { i: 1 },
                BackupRegister { i: 2 },
                BackupRegister { i: 3 },
                BackupRegister { i: 4 },
            ],
        }
    }

    /// Constrain the TAMP peripheral after RTC initialization
    ///
    /// The RTC initializes the backup domain for us but it also resets the backup domain,
    /// clearing the backup registers.
    pub fn new_with_rtc(tamp: TAMP, rtc: &mut Rtc) -> Self {
        let _ = rtc; // Just need to be sure the backup domain is initialized
        Tamp {
            _rb: tamp,
            backup: [
                BackupRegister { i: 0 },
                BackupRegister { i: 1 },
                BackupRegister { i: 2 },
                BackupRegister { i: 3 },
                BackupRegister { i: 4 },
            ],
        }
    }
}

pub struct BackupRegister {
    i: u8,
}

impl BackupRegister {
    pub fn write(&mut self, value: u32) {
        // Unsafe: We only access the bits in TAMP that we have logical ownership of
        // and any bit pattern is valid to write to bkp()
        unsafe {
            let tamp = &*TAMP::ptr();
            match self.i {
                0 => tamp.bkp0r.write(|w| w.bkp().bits(value)),
                1 => tamp.bkp1r.write(|w| w.bkp().bits(value)),
                2 => tamp.bkp2r.write(|w| w.bkp().bits(value)),
                3 => tamp.bkp3r.write(|w| w.bkp().bits(value)),
                4 => tamp.bkp4r.write(|w| w.bkp().bits(value)),
                _ => unreachable!(),
            }
        }
    }

    pub fn read(&self) -> u32 {
        // Unsafe: We only access the bits in TAMP that we have logical ownership of
        let tamp = unsafe { &*TAMP::ptr() };
        match self.i {
            0 => tamp.bkp0r.read().bkp().bits(),
            1 => tamp.bkp1r.read().bkp().bits(),
            2 => tamp.bkp2r.read().bkp().bits(),
            3 => tamp.bkp3r.read().bkp().bits(),
            4 => tamp.bkp4r.read().bkp().bits(),
            _ => unreachable!(),
        }
    }
}
