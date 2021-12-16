#![no_main]
#![no_std]


use core::{cell::RefCell, sync::atomic::{AtomicU16, Ordering}};

use cortex_m::{interrupt::Mutex};
use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{delay::Delay, gpio::{Edge, gpioc::PC13, Input, PullUp}, interrupt, pac, prelude::*};
use rtt_target::{rtt_init_print, rprintln};


type Button = PC13<Input<PullUp>>;
static ATOMIC_COUNTER: AtomicU16 = AtomicU16::new(0);
static G_BUTTON: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));



#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let mut syscfg = device.SYSCFG.constrain();

    let gpioc = device.GPIOC.split();

    let mut button = gpioc.pc13.into_pull_up_input();
    button.make_interrupt_source(&mut syscfg);
    button.enable_interrupt(&mut device.EXTI);
    button.trigger_on_edge(&mut device.EXTI, Edge::Rising,);

    let mut delay = Delay::new(core.SYST, &clocks);

    cortex_m::interrupt::free(|cs| {
        G_BUTTON.borrow(cs).replace(Some(button));
    });

    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::EXTI15_10);
    } 

    loop {
        delay.delay_ms(200u8);
        rprintln!("Loop: {:?}", ATOMIC_COUNTER);
        // TODO use the `rprintln` macro to print the atomics value from the main loop 
    }
}

#[interrupt]
fn EXTI15_10() {

    static mut ctr : u8 = 0 ;

    cortex_m::interrupt::free(|cs| {
        G_BUTTON
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt_pending_bit();
    });  
    
    ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    rprintln!("ISR: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));

    // TODO increment and print the atomic from within the isr  
}
