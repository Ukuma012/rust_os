; asmfunc.asmfunc
;
; System V AMD64 Calling Convention
; Registers: RDI, RSI, RDX, RCX, R8, R9

bits 64
section .text
global IoOut32  ; IoOut32(addr: u16, data: u32) -> ()
IoOut32:
    mov dx, di      ; dx = addr
    mov eax, esi    ; eax = data
    out dx, eax
    ret

global IoIn32   ; IoIn32(addr: u16) -> u32
IoIn32:
    mov dx, di      ; dx = addr
    in eax, dx
    ret
