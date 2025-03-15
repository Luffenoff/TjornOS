# Параметры виртуальной машины
$VM_NAME = "TjornOS-Test"
$ISO_PATH = "tjornos.iso"

# Создание виртуальной машины
VBoxManage createvm --name $VM_NAME --ostype Other_64 --register

# Настройка памяти и CPU
VBoxManage modifyvm $VM_NAME --memory 1024 --cpus 2

# Настройка загрузки
VBoxManage storagectl $VM_NAME --name "IDE Controller" --add ide
VBoxManage storageattach $VM_NAME --storagectl "IDE Controller" --port 0 --device 0 --type dvddrive --medium $ISO_PATH

# Настройка сети
VBoxManage modifyvm $VM_NAME --nic1 nat

Write-Host "Virtual machine configured successfully" 