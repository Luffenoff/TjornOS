#![no_std]
// Так как это ядро ОС, мы не используем стандартную библиотеку
#![no_main] // Не используем стандартную точку входа
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("TjornOS - Starting...");

    // Инициализация базовых подсистем
    unsafe {
        tjorn_core::init();
        tjorn_memory::init(boot_info);
        tjorn_security::init();
    }

    println!("Core systems initialized");

    // Запуск основных сервисов
    start_services();

    println!("TjornOS is ready!");

    loop {
        // Основной цикл системы
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    loop {}
}

fn start_services() {
    // Запуск критических сервисов
    tjorn_fs::init();
    tjorn_network::init();
    tjorn_gui::init();
}
