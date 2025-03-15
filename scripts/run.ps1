# Сборка проекта
cargo build

# Создание образа
cargo bootimage

# Запуск в QEMU
qemu-system-x86_64.exe -drive format=raw, file=target/x86_64-tjornos/debug/bootimage-tjornos.bin -serial stdio 