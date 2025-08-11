#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::{task, Spawner};
use embassy_time::{Delay, Duration, Timer};
use embedded_hal::delay::DelayNs;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Flex, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::time::{Duration as ESPDuration, Instant};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::timer::PeriodicTimer;

use esp_println::println;
use log::info;
use xtensa_lx::timer::delay;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[task]
async fn send_task(mut pin: Output<'static>) {
    let mut delay = Delay;
    loop {
        for _ in 0..7 {
            pin.set_low();
            delay.delay_us(3);
            pin.set_high();
            delay.delay_us(1);
        }
        for _ in 0..2 {
            pin.set_low();
            delay.delay_us(1);
            pin.set_high();
            delay.delay_us(3);
        }
        delay.delay_ms(1000);
    }
}
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();
async fn run(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_240MHz);
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);
    //let timer_group = TimerGroup::new(peripherals.TIMG0);

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    /*let s = send_task(my_pin);

    let _ = spawner.spawn(s);*/
    //let mut timer = PeriodicTimer::new(timer_group.timer0);
    let mut pin = Flex::new(peripherals.GPIO4);
    pin.set_output_enable(false);
    pin.set_input_enable(true);
    pin.apply_input_config(&InputConfig::default().with_pull(Pull::Up));
    loop {
        send_poll(&mut pin);
        let res = read_signal(&mut pin).await;
        info!("{}", res);
    }
}

async fn read_signal(pin: &mut Flex<'static>) -> u32 {
    pin.set_output_enable(false);
    pin.set_input_enable(true);
    pin.apply_input_config(&InputConfig::default().with_pull(Pull::Up));
    let mut out = 0;
    let duration = ESPDuration::from_micros(2);
    for i in 0..32 {
        pin.wait_for_falling_edge().await;
        let now = Instant::now();
        pin.wait_for_rising_edge().await;
        out |= ((Instant::now() - now >= duration) as u32) << i;
    }
    out
}
fn send_poll(pin: &mut Flex<'static>) {
    pin.set_input_enable(false);
    pin.set_output_enable(true);
    pin.apply_output_config(&OutputConfig::default().with_pull(Pull::Up));
    let one_us_with_pin_toggle = 209;
    let one_us_without_pin_toggle = 244;
    for _ in 0..7 {
        pin.set_low();
        delay(one_us_without_pin_toggle * 2 + one_us_with_pin_toggle);
        pin.set_high();
        delay(one_us_with_pin_toggle);
    }
    for _ in 0..2 {
        pin.set_low();
        delay(one_us_with_pin_toggle);
        pin.set_high();
        delay(one_us_without_pin_toggle * 2 + one_us_with_pin_toggle);
    }
}
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    run(spawner).await;
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
