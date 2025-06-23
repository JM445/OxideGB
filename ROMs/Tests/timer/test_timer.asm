SECTION "Timer Interrupt", ROM0[0x50]

    nop
    jp timer_fn
    nop

SECTION "Entrypoint", ROM0[0x100]
    nop
    jp main


SECTION "Main", ROM0[0x150]
main:
    ld  a, 0x05
    ldh [0xFF07], a
    ld  a, 0x00
    ldh [0xFF04], a
    ld  a, 0b100
    ldh [0xFFFF], a
    ei
    nop

loop:
    nop
    nop
    jr loop


timer_fn:
    ld  a, 0x01
    ldh [0xFF07], a

    ld  a, "O"
    ldh [0xFF01], a

    ld  a, $81
    ldh [0xFF02], a

    ld  a, "k"
    ldh [0xFF01], a

    ld  a, $81
    ldh [0xFF02], a

    ld  a, 0x0A
    ldh [0xFF01], a

    ld  a, $81
    ldh [0xFF02], a

    ld  a, 0x05
    ldh [0xFF07], a
    reti
