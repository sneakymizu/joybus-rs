#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::{task, Spawner};
use embassy_time::{Delay, Timer};
use embedded_hal::delay::DelayNs;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Flex;
use esp_hal::gpio::Level;
use esp_hal::gpio::Output;
use esp_hal::gpio::OutputConfig;
use esp_hal::gpio::Pull;
use esp_hal::time::Duration;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::timer::PeriodicTimer;

use log::info;

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

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    //let timer0 = TimerGroup::new(peripherals.TIMG1);
    //esp_hal_embassy::init(timer0.timer0);
    let timer_group = TimerGroup::new(peripherals.TIMG0);

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let mut my_pin = Output::new(
        peripherals.GPIO4,
        Level::High,
        OutputConfig::default().with_pull(Pull::Up),
    );
    //my_pin.set_input_enable(false);
    my_pin.set_high();
    /*let s = send_task(my_pin);

    let _ = spawner.spawn(s);*/
    let mut pin = my_pin;
    let mut timer = PeriodicTimer::new(timer_group.timer0);
    loop {
        for _ in 0..7 {
            pin.set_low();
            timer.start(Duration::from_micros(3));
            timer.wait();
            //Timer::after(Duration::from_micros(3)).await;
            pin.set_high();
            timer.start(Duration::from_micros(1));
            timer.wait();
        }
        for _ in 0..2 {
            pin.set_low();
            timer.start(Duration::from_micros(1));
            timer.wait();
            pin.set_high();
            timer.start(Duration::from_micros(3));
            timer.wait();
        }
        timer.start(Duration::from_secs(1));
        timer.wait();
        //Timer::after(Duration::from_secs(1)).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    run(spawner).await;
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}
