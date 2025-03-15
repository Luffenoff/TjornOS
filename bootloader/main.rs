#![no_std]
#![no_main]

use crate::hardware::ACPI;
use crate::memory::MemoryMap;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// Константы для VGA-буфера
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Цвета текста
const COLOR_WHITE: u8 = 0x0F;

#[repr(C, packed)]
pub struct BootInfo {
    memory_map: MemoryMap,
    acpi_tables: ACPI,
    framebuffer: FramebufferInfo,
    kernel_base: u64,
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Очистка экрана
    clear_screen();

    println!("TjornOS Bootloader v0.1.0");
    println!("Initializing system...");

    // Базовая инициализация
    init_cpu();
    init_memory(boot_info);

    // Загрузка ядра
    println!("Loading kernel...");
    load_kernel();

    loop {}
}

fn clear_screen() {
    for i in 0..VGA_HEIGHT * VGA_WIDTH * 2 {
        unsafe {
            *VGA_BUFFER.offset(i) = 0;
        }
    }
}

fn print_string(s: &str) {
    let mut offset = 0;
    for byte in s.bytes() {
        unsafe {
            *VGA_BUFFER.offset(offset) = byte;
            *VGA_BUFFER.offset(offset + 1) = COLOR_WHITE;
        }
        offset += 2;
    }
}

fn check_a20() {
    // Проверка включения A20 линии
    // Это позволит использовать память выше 1 МБ
}

fn load_gdt() {
    // Загрузка таблицы глобальных дескрипторов
    // Необходима для защищенного режима
}

fn jump_to_kernel(kernel_binary: &[u8], boot_info: BootInfo) {
    // Переход к точке входа ядра
}

fn init_cpu() {
    // Проверка поддержки необходимых функций CPU
    check_cpu_features();

    // Настройка защищенного режима
    enable_protected_mode();

    // Настройка длинного режима
    enable_long_mode();
}

fn init_memory(boot_info: &'static BootInfo) {
    // Инициализация карты памяти
}

fn load_kernel() {
    // Загрузка ядра в память
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_string("ПАНИКА: Загрузчик остановлен");
    loop {}
}
