//! This crate contains register definitions for
#![cfg_attr(feature = "at90can128", doc = "**at90can128**,")]
#![cfg_attr(feature = "at90can64", doc = "**at90can64**,")]
#![cfg_attr(feature = "at90can32", doc = "**at90can32**,")]
#![cfg_attr(feature = "at90usb1286", doc = "**at90usb1286**,")]
#![cfg_attr(feature = "atmega1280", doc = "**atmega1280**,")]
#![cfg_attr(feature = "atmega1284p", doc = "**atmega1284p**,")]
#![cfg_attr(feature = "atmega128a", doc = "**atmega128a**,")]
#![cfg_attr(feature = "atmega128rfa1", doc = "**atmega128rfa1**,")]
#![cfg_attr(feature = "atmega16", doc = "**atmega16**,")]
#![cfg_attr(feature = "atmega164pa", doc = "**atmega164pa**,")]
#![cfg_attr(feature = "atmega168", doc = "**atmega168**,")]
#![cfg_attr(feature = "atmega16u2", doc = "**atmega16u2**,")]
#![cfg_attr(feature = "atmega2560", doc = "**atmega2560**,")]
#![cfg_attr(feature = "atmega8", doc = "**atmega8**,")]
#![cfg_attr(feature = "atmega8u2", doc = "**atmega8u2**,")]
#![cfg_attr(feature = "atmega324pa", doc = "**atmega324pa**,")]
#![cfg_attr(feature = "atmega328p", doc = "**atmega328p**,")]
#![cfg_attr(feature = "atmega328pb", doc = "**atmega328pb**,")]
#![cfg_attr(feature = "atmega32a", doc = "**atmega32a**,")]
#![cfg_attr(feature = "atmega32u2", doc = "**atmega32u2**,")]
#![cfg_attr(feature = "atmega32u4", doc = "**atmega32u4**,")]
#![cfg_attr(feature = "atmega3208", doc = "**atmega3208**,")]
#![cfg_attr(feature = "atmega3209", doc = "**atmega3209**,")]
#![cfg_attr(feature = "atmega4808", doc = "**atmega4808**,")]
#![cfg_attr(feature = "atmega4809", doc = "**atmega4809**,")]
#![cfg_attr(feature = "atmega48p", doc = "**atmega48p**,")]
#![cfg_attr(feature = "atmega64", doc = "**atmega64**,")]
#![cfg_attr(feature = "atmega64m1", doc = "**atmega64m1**,")]
#![cfg_attr(feature = "atmega644", doc = "**atmega644**,")]
#![cfg_attr(feature = "atmega88p", doc = "**atmega88p**,")]
#![cfg_attr(feature = "attiny13a", doc = "**attiny13a**,")]
#![cfg_attr(feature = "attiny167", doc = "**attiny167**,")]
#![cfg_attr(feature = "attiny1604", doc = "**attiny1604**,")]
#![cfg_attr(feature = "attiny1606", doc = "**attiny1606**,")]
#![cfg_attr(feature = "attiny1614", doc = "**attiny1614**,")]
#![cfg_attr(feature = "attiny1626", doc = "**attiny1626**,")]
#![cfg_attr(feature = "attiny202", doc = "**attiny202**,")]
#![cfg_attr(feature = "attiny204", doc = "**attiny204**,")]
#![cfg_attr(feature = "attiny212", doc = "**attiny212**,")]
#![cfg_attr(feature = "attiny214", doc = "**attiny214**,")]
#![cfg_attr(feature = "attiny2313", doc = "**attiny2313**,")]
#![cfg_attr(feature = "attiny2313a", doc = "**attiny2313a**,")]
#![cfg_attr(feature = "attiny26", doc = "**attiny26**,")]
#![cfg_attr(feature = "attiny261a", doc = "**attiny261a**,")]
#![cfg_attr(feature = "attiny402", doc = "**attiny402**,")]
#![cfg_attr(feature = "attiny404", doc = "**attiny404**,")]
#![cfg_attr(feature = "attiny412", doc = "**attiny412**,")]
#![cfg_attr(feature = "attiny414", doc = "**attiny414**,")]
#![cfg_attr(feature = "attiny416", doc = "**attiny416**,")]
#![cfg_attr(feature = "attiny44a", doc = "**attiny44a**,")]
#![cfg_attr(feature = "attiny461a", doc = "**attiny461a**,")]
#![cfg_attr(feature = "attiny804", doc = "**attiny804**,")]
#![cfg_attr(feature = "attiny816", doc = "**attiny816**,")]
#![cfg_attr(feature = "attiny828", doc = "**attiny828**,")]
#![cfg_attr(feature = "attiny84", doc = "**attiny84**,")]
#![cfg_attr(feature = "attiny841", doc = "**attiny841**,")]
#![cfg_attr(feature = "attiny84a", doc = "**attiny84a**,")]
#![cfg_attr(feature = "attiny85", doc = "**attiny85**,")]
#![cfg_attr(feature = "attiny861", doc = "**attiny861**,")]
#![cfg_attr(feature = "attiny861a", doc = "**attiny861a**,")]
#![cfg_attr(feature = "attiny88", doc = "**attiny88**,")]
#![cfg_attr(feature = "avr64du32", doc = "**avr64du32**,")]
#![cfg_attr(feature = "avr64du28", doc = "**avr64du28**,")]
#![cfg_attr(feature = "avr128db28", doc = "**avr128db28**,")]
//! and a few things which apply to AVR microcontrollers generally.
//!
#![cfg_attr(
    feature = "docsrs",
    doc = "**Warning**: The doc-build here on docs.rs is only for a subset of supported chips.  Please build documentation locally if your MCU's registers are not documented here.\n\n"
)]
//! Which chips the crate is built for depends on the feature flag used.
//! The following chips are available (using feature flags of the same name):
//! `at90can128`,
//! `at90can64`,
//! `at90can32`,
//! `at90usb1286`,
//! `atmega1280`,
//! `atmega1284p`,
//! `atmega128a`,
//! `atmega128rfa1`,
//! `atmega16`,
//! `atmega164pa`,
//! `atmega168`,
//! `atmega16u2`,
//! `atmega2560`,
//! `atmega8`,
//! `atmega8u2`,
//! `atmega324pa`
//! `atmega328p`,
//! `atmega328pb`,
//! `atmega32a`
//! `atmega32u2`,
//! `atmega32u4`,
//! `atmega3208`,
//! `atmega3209`,
//! `atmega4808`,
//! `atmega4809`,
//! `atmega48p`,
//! `atmega64`,
//! `atmega64m1`,
//! `atmega644`,
//! `atmega88p`,
//! `attiny13a`,
//! `attiny167`,
//! `attiny1604`,
//! `attiny1614`,
//! `attiny1626`,
//! `attiny202`,
//! `attiny204`,
//! `attiny212`,
//! `attiny214`,
//! `attiny2313`,
//! `attiny2313a`,
//! `attiny26`,
//! `attiny261a`,
//! `attiny402`,
//! `attiny404`,
//! `attiny412`,
//! `attiny414`,
//! `attiny416`,
//! `attiny44a`,
//! `attiny461a`,
//! `attiny804`,
//! `attiny816`,
//! `attiny828`,
//! `attiny84`,
//! `attiny841`,
//! `attiny84a`,
//! `attiny85`,
//! `attiny861`,
//! `attiny861a`,
//! `attiny88`,
//! `avr64du32`,
//! `avr64du28`,
//! `avr128db28`,
//!
//! # How to use this crate?
//!
//! In most cases you probably don't want to use this crate directly.
//!
//! This is a low level peripheral access crate (PAC).
//! There are more high level crates, like `avr-hal`, that implement a more convenient
//! and higher level API built ontop of `avr-device`.
//! However, sometimes it's required to operate on bare device register level.
//! That's what this crate is for.
//!
//! ## Main program entry point
//!
//! ```
//! #[avr_device::entry] // requires avr_device's rt feature.
//! fn main() -> ! {
//!     loop {
//!         // Your code here.
//!     }
//! }
//! ```
//!
//! ## Get access to the device peripherals
//!
//! ```ignore
//! // To get access to the Peripherals struct, use this *once*:
//! let dp = avr_device::atmega328p::Peripherals::take().unwrap();
//! ```
//!
//! ## Example: Digital I/O port access
//!
//! ```ignore
//! // Configure bit 5 of port B as output:
//! dp.PORTB.ddrb.write(|w| w.pb5().set_bit());
//! // Clear bit 5 of port B:
//! dp.PORTB.portb.write(|w| w.pb5().clear_bit());
//! // Set bit 5 of port B:
//! dp.PORTB.portb.write(|w| w.pb5().set_bit());
//!
//! // Configure bit 6 of port B as input with pullup:
//! dp.PORTB.ddrb.write(|w| w.pb6().clear_bit());
//! dp.PORTB.portb.write(|w| w.pb6().set_bit());
//! // Read bit 6 of pin B:
//! let _mybit = dp.PORTB.pinb.read().pb6().bit_is_set();
//! // Read bit 6 and write to bit 5 of port B:
//! dp.PORTB.portb.modify(|r, w| w.pb6().bit(r.pb5().bit_is_set()))
//! ```
//!
//! ## Example: Other peripheral register access
//!
//! Other peripheral register accesses are similar to I/O port access.
//! Please read the documentation of the `struct R` and `struct W`
//! for the register of interest.
//!
//! e.g. [crate::atmega328p::spi::spcr::W]
//!
//! ```ignore
//! // Set SPE in SPCR (Enable SPI):
//! dp.SPI.spcr.write(|w| w.spe().set_bit());
//! ```
//!
//! # Crate feature flags
//!
//! * Device selection: To enable your device, select the crate feature that matches your device.
//!   For a full list of supported devices, see the list at the beginning of this documentation.
//! * To enable the crate's runtime environment, use the `rt` feature.
//! * To enable `ufmt` formatting (e.g. `uDebug`), use the `ufmt` feature.

#![no_std]
#![cfg_attr(target_arch = "avr", feature(asm_experimental_arch))] // for experimental AVR asm! macro.

pub mod asm;
pub mod interrupt;

pub(crate) use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic {
    include!(concat!(env!("OUT_DIR"), "/pac/generic.rs"));
}

/// Attribute to declare an interrupt service routine
///
/// ```
/// #[avr_device::interrupt(atmega32u4)]
/// fn INT6() {
///     // ...
/// }
/// ```
///
/// # Constraints
/// - The name of the function must be the name of an interrupt.  Each chip's
///   module has a `Interrupt` enum defining the available names.
/// - The attribute needs the chip-name to correctly map the interrupt to its
///   vector.  This is an unfortunate requirement of the current crate
///   architecture and might change in the future.
/// - The function must have a signature of `[unsafe] fn() [-> !]`.
/// - This macro requires the avr-device `rt` crate feature.
#[cfg(feature = "rt")]
pub use avr_device_macros::interrupt;

/// Attribute to declare the entry point of the program
///
/// Exactly one entry point must be declared in the entire dependency tree.
///
/// ```
/// #[avr_device::entry]
/// fn main() -> ! {
///     loop {
///         // ...
///     }
/// }
/// ```
///
/// # Constraints
/// - The entry function must have a signature of `[unsafe] fn() -> !`.
/// - This macro requires the avr-device `rt` crate feature.
#[cfg(feature = "rt")]
pub use avr_device_macros::entry;

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "This crate requires you to specify your target chip as a feature.

    Please select one of the following:
    * atmega1280
    * atmega1284p
    * atmega128a
    * atmega128rfa1
    * atmega16
    * atmega164pa
    * atmega168
    * atmega16u2
    * atmega2560
    * atmega324pa
    * atmega328p
    * atmega328pb
    * atmega32a
    * atmega32u2
    * atmega32u4
    * atmega3208
    * atmega3209
    * atmega4808
    * atmega4809
    * atmega48p
    * atmega64
    * atmega64m1
    * atmega644
    * atmega8
    * atmega8u2
    * atmega88p
    * attiny13a
    * attiny167
    * attiny1604
    * attiny1606
    * attiny1614
    * attiny1626
    * attiny202
    * attiny204
    * attiny212
    * attiny214
    * attiny2313
    * attiny2313a
    * attiny26
    * attiny261a
    * attiny402
    * attiny404
    * attiny412
    * attiny414
    * attiny416
    * attiny44a
    * attiny461a
    * attiny804
    * attiny816
    * attiny828
    * attiny84
    * attiny841
    * attiny84a
    * attiny85
    * attiny861
    * attiny861a
    * attiny88
    * avr64du32
    * avr64du28
    * avr128db28
    "
);

#[allow(non_camel_case_types, unused_attributes, unreachable_patterns)]
mod devices;

include!(concat!(env!("OUT_DIR"), "/pac/vector.rs"));

#[cfg(feature = "at90can128")]
pub use crate::devices::at90can128;
#[cfg(feature = "at90can64")]
pub use crate::devices::at90can64;
#[cfg(feature = "at90can32")]
pub use crate::devices::at90can32;
#[cfg(feature = "at90usb1286")]
pub use crate::devices::at90usb1286;
#[cfg(feature = "atmega1280")]
pub use crate::devices::atmega1280;
#[cfg(feature = "atmega1284p")]
pub use crate::devices::atmega1284p;
#[cfg(feature = "atmega128a")]
pub use crate::devices::atmega128a;
#[cfg(feature = "atmega128rfa1")]
pub use crate::devices::atmega128rfa1;
#[cfg(feature = "atmega16")]
pub use crate::devices::atmega16;
#[cfg(feature = "atmega164pa")]
pub use crate::devices::atmega164pa;
#[cfg(feature = "atmega168")]
pub use crate::devices::atmega168;
#[cfg(feature = "atmega16u2")]
pub use crate::devices::atmega16u2;
#[cfg(feature = "atmega2560")]
pub use crate::devices::atmega2560;
#[cfg(feature = "atmega3208")]
pub use crate::devices::atmega3208;
#[cfg(feature = "atmega3209")]
pub use crate::devices::atmega3209;
#[cfg(feature = "atmega324pa")]
pub use crate::devices::atmega324pa;
#[cfg(feature = "atmega328p")]
pub use crate::devices::atmega328p;
#[cfg(feature = "atmega328pb")]
pub use crate::devices::atmega328pb;
#[cfg(feature = "atmega32a")]
pub use crate::devices::atmega32a;
#[cfg(feature = "atmega32u2")]
pub use crate::devices::atmega32u2;
#[cfg(feature = "atmega32u4")]
pub use crate::devices::atmega32u4;
#[cfg(feature = "atmega4808")]
pub use crate::devices::atmega4808;
#[cfg(feature = "atmega4809")]
pub use crate::devices::atmega4809;
#[cfg(feature = "atmega48p")]
pub use crate::devices::atmega48p;
#[cfg(feature = "atmega64")]
pub use crate::devices::atmega64;
#[cfg(feature = "atmega64m1")]
pub use crate::devices::atmega64m1;
#[cfg(feature = "atmega644")]
pub use crate::devices::atmega644;
#[cfg(feature = "atmega8")]
pub use crate::devices::atmega8;
#[cfg(feature = "atmega88p")]
pub use crate::devices::atmega88p;
#[cfg(feature = "atmega8u2")]
pub use crate::devices::atmega8u2;
#[cfg(feature = "attiny13a")]
pub use crate::devices::attiny13a;
#[cfg(feature = "attiny1604")]
pub use crate::devices::attiny1604;
#[cfg(feature = "attiny1606")]
pub use crate::devices::attiny1606;
#[cfg(feature = "attiny1614")]
pub use crate::devices::attiny1614;
#[cfg(feature = "attiny1626")]
pub use crate::devices::attiny1626;
#[cfg(feature = "attiny167")]
pub use crate::devices::attiny167;
#[cfg(feature = "attiny202")]
pub use crate::devices::attiny202;
#[cfg(feature = "attiny204")]
pub use crate::devices::attiny204;
#[cfg(feature = "attiny212")]
pub use crate::devices::attiny212;
#[cfg(feature = "attiny214")]
pub use crate::devices::attiny214;
#[cfg(feature = "attiny2313")]
pub use crate::devices::attiny2313;
#[cfg(feature = "attiny2313a")]
pub use crate::devices::attiny2313a;
#[cfg(feature = "attiny26")]
pub use crate::devices::attiny26;
#[cfg(feature = "attiny261a")]
pub use crate::devices::attiny261a;
#[cfg(feature = "attiny402")]
pub use crate::devices::attiny402;
#[cfg(feature = "attiny404")]
pub use crate::devices::attiny404;
#[cfg(feature = "attiny412")]
pub use crate::devices::attiny412;
#[cfg(feature = "attiny414")]
pub use crate::devices::attiny414;
#[cfg(feature = "attiny416")]
pub use crate::devices::attiny416;
#[cfg(feature = "attiny44a")]
pub use crate::devices::attiny44a;
#[cfg(feature = "attiny461a")]
pub use crate::devices::attiny461a;
#[cfg(feature = "attiny804")]
pub use crate::devices::attiny804;
#[cfg(feature = "attiny816")]
pub use crate::devices::attiny816;
#[cfg(feature = "attiny828")]
pub use crate::devices::attiny828;
#[cfg(feature = "attiny84")]
pub use crate::devices::attiny84;
#[cfg(feature = "attiny841")]
pub use crate::devices::attiny841;
#[cfg(feature = "attiny84a")]
pub use crate::devices::attiny84a;
#[cfg(feature = "attiny85")]
pub use crate::devices::attiny85;
#[cfg(feature = "attiny861")]
pub use crate::devices::attiny861;
#[cfg(feature = "attiny861a")]
pub use crate::devices::attiny861a;
#[cfg(feature = "attiny88")]
pub use crate::devices::attiny88;
#[cfg(feature = "avr64du28")]
pub use crate::devices::avr64du28;
#[cfg(feature = "avr64du32")]
pub use crate::devices::avr64du32;
#[cfg(feature = "avr128db28")]
pub use crate::devices::avr128db28;
