#!/bin/bash
set -e

# Сборка проекта
cargo build

# Создание образа
cargo bootimage

# Запуск в QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-tjornos/debug/bootimage-tjornos.bin -serial stdio 