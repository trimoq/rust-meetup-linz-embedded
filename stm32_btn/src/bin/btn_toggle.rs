#![no_main]
#![no_std]

use core::{cell::RefCell, sync::atomic::{Ordering, AtomicU16}};

use cortex_m::{interrupt::Mutex, asm::wfi};
use cortex_m_rt::entry;
use panic_probe as _;

use stm32f4xx_hal::{delay::Delay, prelude::*, pac, interrupt};

use btn::*;

static G_BUTTON: Mutex<RefCell<Option<BtnPin>>> = Mutex::new(RefCell::new(None));
static ATOMIC_COUNTER: AtomicU16 = AtomicU16::new(0);

#[entry]
fn main() -> ! {

    let mut setup = setup_board_minimal().unwrap();

    let mut led = setup.gpioa.pa5.into_push_pull_output();

    cortex_m::interrupt::free(|cs| {
        let button = Button::new();
        button.init(&cs, &mut setup.exti, setup.gpioc, &mut setup.syscfg);
        *G_BUTTON.borrow(cs).borrow_mut() = Some(button.into_inner(&cs).unwrap());
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }    

    loop {
        // TODO use the main loop to toggle the led
        if(ATOMIC_COUNTER.load(Ordering::SeqCst) % 2 == 0) {
            led.set_high();
        } else {
            led.set_low();
        }
        wfi();
    }
}

#[interrupt]
fn EXTI15_10() {
    static mut BUTTON: Option<BtnPin> = None;

    let button = steal!(BUTTON, G_BUTTON);

    // TODO  clear the interrupt pending bit on the button
    cortex_m::interrupt::free(|_cs| {
        button.clear_interrupt_pending_bit();
    });

    // TODO  do something so that the main loop can toggle the led
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
}

