//! A minimal device crate for a generic target.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub use self::Interrupt as interrupt;
pub use cortex_m_rt::interrupt;

/// Number of bits available in the NVIC for configuring priority
#[cfg(armv7m)]
pub const NVIC_PRIO_BITS: u8 = 3;
/// Number of bits available in the NVIC for configuring priority
#[cfg(not(armv7m))]
pub const NVIC_PRIO_BITS: u8 = 2;

/// Enumeration of all interrupts
///
/// **NOTE**: When compiling this crate for the `thumbv6m-none-eabi` target this enumeration will
/// only contain the variants from `GPIOA` to `GPIOG`.
// NOTE: See Table 2-9 Interrupts. Section 2.5.2 "Exception types"
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Interrupt {
    /// GPIO Port A
    GPIOA = 0,
    /// GPIO Port B
    GPIOB = 1,
    /// GPIO Port C
    GPIOC = 2,
    /// GPIO Port D
    GPIOD = 3,
    /// GPIO Port E
    GPIOE = 4,
    /// UART0
    UART0 = 5,
    /// UART1
    UART1 = 6,
    /// SSI0
    SSI0 = 7,
    /// I2C0
    I2C0 = 8,
    /// PWM fault
    PWM_FAULT = 9,
    /// PWM Generator 0
    PWM_GENERATOR_0 = 10,
    /// PWM Generator 1
    PWM_GENERATOR_1 = 11,
    /// PWM Generator 2
    PWM_GENERATOR_2 = 12,
    /// QEI0
    QEI0 = 13,
    /// ADC0 Sequence 0
    ADC0_SEQUENCE_0 = 14,
    /// ADC0 Sequence 1
    ADC0_SEQUENCE_1 = 15,
    /// ADC0 Sequence 2
    ADC0_SEQUENCE_2 = 16,
    /// ADC0 Sequence 3
    ADC0_SEQUENCE_3 = 17,
    /// Watchdog Timer 0
    WATCHDOG_TIMER_0 = 18,
    /// Timer 0A
    TIMER_0A = 19,
    /// Timer 0B
    TIMER_0B = 20,
    /// Timer 1A
    TIMER_1A = 21,
    /// Timer 1B
    TIMER_1B = 22,
    /// Timer 2A
    TIMER_2A = 23,
    /// Timer 2B
    TIMER_2B = 24,
    /// Analog Comparator 0
    ANALOG_COMPARATOR_0 = 25,
    /// Analog Comparator 1
    ANALOG_COMPARATOR_1 = 26,
    /// System Control
    SYSTEM_CONTROL = 28,
    /// Flash Memory Control
    FLASH_MEMORY_CONTROL = 29,
    /// GPIO Port F
    GPIOF = 30,
    /// GPIO Port G
    GPIOG = 31,
    /// UART2
    #[cfg(armv7m)]
    UART2 = 33,
    /// Timer 3A
    #[cfg(armv7m)]
    TIMER_3A = 35,
    /// Timer 3B
    #[cfg(armv7m)]
    TIMER_3B = 36,
    /// I2C1
    #[cfg(armv7m)]
    I2C1 = 37,
    /// QEI1
    #[cfg(armv7m)]
    QEI1 = 38,
    /// Ethernet Controller
    #[cfg(armv7m)]
    ETHERNET = 42,
    /// Hibernation Module
    #[cfg(armv7m)]
    HIBERNATION = 43,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}

extern "C" {
    fn GPIOA();
    fn GPIOB();
    fn GPIOC();
    fn GPIOD();
    fn GPIOE();
    fn UART0();
    fn UART1();
    fn SSI0();
    fn I2C0();
    fn PWM_FAULT();
    fn PWM_GENERATOR_0();
    fn PWM_GENERATOR_1();
    fn PWM_GENERATOR_2();
    fn QEI0();
    fn ADC0_SEQUENCE_0();
    fn ADC0_SEQUENCE_1();
    fn ADC0_SEQUENCE_2();
    fn ADC0_SEQUENCE_3();
    fn WATCHDOG_TIMER_0();
    fn TIMER_0A();
    fn TIMER_0B();
    fn TIMER_1A();
    fn TIMER_1B();
    fn TIMER_2A();
    fn TIMER_2B();
    fn ANALOG_COMPARATOR_0();
    fn ANALOG_COMPARATOR_1();
    fn SYSTEM_CONTROL();
    fn FLASH_MEMORY_CONTROL();
    fn GPIOF();
    fn GPIOG();
    #[cfg(armv7m)]
    fn UART2();
    #[cfg(armv7m)]
    fn TIMER_3A();
    #[cfg(armv7m)]
    fn TIMER_3B();
    #[cfg(armv7m)]
    fn I2C1();
    #[cfg(armv7m)]
    fn QEI1();
    #[cfg(armv7m)]
    fn ETHERNET();
    #[cfg(armv7m)]
    fn HIBERNATION();
}

union Vector {
    handler: unsafe extern "C" fn(),
    reserved: u32,
}

#[cfg(armv7m)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static __INTERRUPTS: [Vector; 44] = [
    Vector { handler: GPIOA },
    Vector { handler: GPIOB },
    Vector { handler: GPIOC },
    Vector { handler: GPIOD },
    Vector { handler: GPIOE },
    Vector { handler: UART0 },
    Vector { handler: UART1 },
    Vector { handler: SSI0 },
    Vector { handler: I2C0 },
    Vector { handler: PWM_FAULT },
    Vector {
        handler: PWM_GENERATOR_0,
    },
    Vector {
        handler: PWM_GENERATOR_1,
    },
    Vector {
        handler: PWM_GENERATOR_2,
    },
    Vector { handler: QEI0 },
    Vector {
        handler: ADC0_SEQUENCE_0,
    },
    Vector {
        handler: ADC0_SEQUENCE_1,
    },
    Vector {
        handler: ADC0_SEQUENCE_2,
    },
    Vector {
        handler: ADC0_SEQUENCE_3,
    },
    Vector {
        handler: WATCHDOG_TIMER_0,
    },
    Vector { handler: TIMER_0A },
    Vector { handler: TIMER_0B },
    Vector { handler: TIMER_1A },
    Vector { handler: TIMER_1B },
    Vector { handler: TIMER_2A },
    Vector { handler: TIMER_2B },
    Vector {
        handler: ANALOG_COMPARATOR_0,
    },
    Vector {
        handler: ANALOG_COMPARATOR_1,
    },
    Vector { reserved: 0 },
    Vector {
        handler: SYSTEM_CONTROL,
    },
    Vector {
        handler: FLASH_MEMORY_CONTROL,
    },
    Vector { handler: GPIOF },
    Vector { handler: GPIOG },
    Vector { reserved: 0 },
    Vector { handler: UART2 },
    Vector { reserved: 0 },
    Vector { handler: TIMER_3A },
    Vector { handler: TIMER_3B },
    Vector { handler: I2C1 },
    Vector { handler: QEI1 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: ETHERNET },
    Vector {
        handler: HIBERNATION,
    },
];

#[cfg(not(armv7m))]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
static __INTERRUPTS: [Vector; 32] = [
    Vector { handler: GPIOA },
    Vector { handler: GPIOB },
    Vector { handler: GPIOC },
    Vector { handler: GPIOD },
    Vector { handler: GPIOE },
    Vector { handler: UART0 },
    Vector { handler: UART1 },
    Vector { handler: SSI0 },
    Vector { handler: I2C0 },
    Vector { handler: PWM_FAULT },
    Vector {
        handler: PWM_GENERATOR_0,
    },
    Vector {
        handler: PWM_GENERATOR_1,
    },
    Vector {
        handler: PWM_GENERATOR_2,
    },
    Vector { handler: QEI0 },
    Vector {
        handler: ADC0_SEQUENCE_0,
    },
    Vector {
        handler: ADC0_SEQUENCE_1,
    },
    Vector {
        handler: ADC0_SEQUENCE_2,
    },
    Vector {
        handler: ADC0_SEQUENCE_3,
    },
    Vector {
        handler: WATCHDOG_TIMER_0,
    },
    Vector { handler: TIMER_0A },
    Vector { handler: TIMER_0B },
    Vector { handler: TIMER_1A },
    Vector { handler: TIMER_1B },
    Vector { handler: TIMER_2A },
    Vector { handler: TIMER_2B },
    Vector {
        handler: ANALOG_COMPARATOR_0,
    },
    Vector {
        handler: ANALOG_COMPARATOR_1,
    },
    Vector { reserved: 0 },
    Vector {
        handler: SYSTEM_CONTROL,
    },
    Vector {
        handler: FLASH_MEMORY_CONTROL,
    },
    Vector { handler: GPIOF },
    Vector { handler: GPIOG },
];

/// All the peripherals
///
/// Yep, this is currently empty!
pub struct Peripherals {
    _0: (),
}

impl Peripherals {
    /// Kind of useless as there's no register API but this is required by RTFM
    #[inline]
    pub unsafe fn steal() -> Self {
        Peripherals { _0: () }
    }
}
