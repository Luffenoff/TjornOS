# Сборка проекта
cargo build --release

# Создание загрузочного ISO
$IsoDir = "build/iso"
$BootDir = "$IsoDir/boot"

# Создаем директории
New-Item -ItemType Directory -Force -Path $BootDir

# Копируем ядро
Copy-Item "target/x86_64-unknown-none/release/tjornos" -Destination "$BootDir/kernel.bin"

# Создаем конфиг для загрузчика
@"
TIMEOUT=0
DEFAULT=tjornos

LABEL tjornos
    KERNEL /boot/kernel.bin
"@ | Out-File -FilePath "$IsoDir/syslinux.cfg" -Encoding ASCII

# Создаем ISO образ
# Здесь нужно использовать инструменты для создания загрузочного ISO 