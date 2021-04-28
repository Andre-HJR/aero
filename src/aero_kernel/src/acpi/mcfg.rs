use core::mem;

use spin::Once;

use super::sdt::Sdt;

pub(super) const SIGNATURE: &str = "MCFG";

static MCFG: Once<&'static Mcfg> = Once::new();

#[repr(C, packed)]
pub struct DeviceConfig {
    pub base_address: u64,
    pub pci_seg_group: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub reserved: u32,
}

/// The ACPI MCFG table describes the location of the PCI Express configuration space.
#[repr(C, packed)]
pub struct Mcfg {
    header: Sdt,
    reserved: u64,
}

impl Mcfg {
    pub(super) unsafe fn init(&'static self) {
        MCFG.call_once(|| self);
    }

    pub fn entry_count(&self) -> usize {
        (self.header.length as usize - mem::size_of::<Self>()) / mem::size_of::<DeviceConfig>()
    }
}

/// Returns true if the ACPI table contains the MCFG entry.
///
/// ## Notes
/// Returns false if called before the ACPI tables were initialized.
pub fn is_avaliable() -> bool {
    MCFG.get().is_some()
}

/// Return a immutable reference to the MCFG table.
pub fn get_mcfg_table() -> &'static Mcfg {
    MCFG.get()
        .expect("Attempted to get the MCFG table before it was initialized")
}
