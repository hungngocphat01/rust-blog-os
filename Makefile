arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

ld_script := src/arch/x86_64/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg

rust_os := target/$(arch)-blog_os/debug/libblog_os.a

docker_run := docker run \
	--rm \
	--platform linux/$(arch) \
	-v $(shell pwd):/workspace \
	-w /workspace \
	nasm-x86

asm_src := $(wildcard src/arch/$(arch)/*.asm)
asm_obj := $(patsubst src/arch/$(arch)/%.asm, \
			build/arch/$(arch)/%.o, $(asm_src))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -rf build target

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

kernel:
	@cargo build

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@$(docker_run) grub-mkrescue -o $(iso) build/isofiles

$(kernel): $(asm_obj) $(ld_script) $(rust_os)
	@$(docker_run) ld --gc-sections -n -T $(ld_script) -o $(kernel) $(asm_obj) $(rust_os)

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@$(docker_run) nasm -felf64 $< -o $@
