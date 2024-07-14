global start
extern long_mode_start

section .text
bits 32
start:
	mov esp, stack_top

	call check_multiboot
	call check_cpuid
	call check_long_mode

    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer]

    ; set cs register and jump to label
    ; 64-bit mode does not use cs for segmentation (offset not calculated)
    jmp gdt64.code:long_mode_start

; Print 'ERR' and error code in AL
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt

; Verify bootloader is multiboot
; In multiboot specs, the bootloader must write this magic number to eax
; before jumping to the kernel
check_multiboot:
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "0"
	jmp error

; Check if CPUID is supported
; This is copied. Don't need to understand
check_cpuid:
    pushfd
    pop eax

    mov ecx, eax

    xor eax, 1 << 21

    push eax
    popfd

    pushfd
    pop eax

    push ecx
    popfd

    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp error

; Check if long mode is supported
; Code copied. Don't need to understand
check_long_mode:
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, "2"
    jmp error


setup_page_tables:
    ; map first p4 entry to p3
    mov eax, p3_table
    or eax, 0b11         ; present+writable bit
    mov [p4_table], eax

    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax

    ; map each p2 entry to a 2MiB page
    mov ecx, 0

.map_p2_table:
    mov eax, 0x200000   ; 2 MiB
    mul ecx             ; eax = eax * ecx
    or eax, 0b10000011  ; present + writable + huge
    mov [p2_table + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ret


enable_paging:
    ; load P4 to cr3 register (cpu uses this to access the P4 table)
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret


section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; new
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64


section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 64
stack_top: