# Параметры сборки
$BUILD_DIR = "build"
$ISO_DIR = "$BUILD_DIR/iso"
$BOOT_DIR = "$ISO_DIR/boot"
$GRUB_DIR = "$BOOT_DIR/grub"

# Создание директорий
New-Item -ItemType Directory -Force -Path $GRUB_DIR

# Сборка проекта
Write-Host "Building TjornOS..."
cargo build --target x86_64-unknown-none --release

# Проверка успешности сборки
if (-not $?) {
    Write-Host "Build failed!"
    exit 1
}

# Копирование файлов
Write-Host "Preparing ISO structure..."
$KERNEL_PATH = "target/x86_64-unknown-none/release/tjornos"
if (Test-Path $KERNEL_PATH) {
    Copy-Item $KERNEL_PATH -Destination "$BOOT_DIR/kernel.bin"
}
else {
    Write-Host "Kernel binary not found!"
    exit 1
}

# Создание конфига GRUB
@"
set timeout=0
set default=0

menuentry "TjornOS" {
    multiboot /boot/kernel.bin
    boot
}
"@ | Out-File -FilePath "$GRUB_DIR/grub.cfg" -Encoding ASCII

# Создание ISO
Write-Host "Creating ISO image..."
$xorriso_path = "tools/xorriso/xorriso.exe"
& $xorriso_path -as mkisofs -R -f -b boot/grub/i386-pc/eltorito.img -no-emul-boot -boot-load-size 4 -boot-info-table -o "tjornos.iso" $ISO_DIR

Write-Host "ISO created successfully: tjornos.iso" 