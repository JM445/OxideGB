SECTION "Timer Interrupt", ROM0[0x50]

    jp timer_handler

SECTION "Entrypoint", ROM0[0x100]
    nop
    jp main


SECTION "Main", ROM0[0x150]
main:
    nop
    nop
    ld sp, 0xE000
    ld a, 0x42
    ldh [0xFF02], a
    nop

    ld hl, hello
    call print_string

    /* T-Cycle: 2152 */

    ld a, $FF
    ldh [0xFF06], a             ; Set timer to overflow at each 4 M-Cycles
    ld a, $05
    ldh [0xFF07], a
    ld a, $FF
    ldh [0xFF05], a             ; Set TIMA to FF
    ldh [0xFF04], a             ; Reset DIV
    ei
    ldh [0xFFFF], a

endloop:
    jr endloop


print_char:                     ; Print char in A
    push af
    ldh [0xFF01], a
    ld a, $81
    ldh [0xFF02], a
    pop af
    ret

print_string:                   ; Print string at HL
    push af
    push hl

print_loop:
    ld a, [hl+]
    cp 0
    jr z, .quit
    call print_char
    jp print_loop


.quit:
    pop hl
    pop af
    ret

timer_handler:
    ;; reti
    ld hl, timer_string
    call print_string
    reti


SECTION "Data", ROM0[0x3000]

hello:
    DB "Hello World !\n",0

timer_string:
    DB "Hello from timer interrupt !\n",0
