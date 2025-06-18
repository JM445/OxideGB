SECTION "FixedROM", ROM0[$0100]

start:
    ;; Test LD r, r
    ld B, C     ; B should become C (0)
    ld A, E     ; A should become E (0)

    ;; Test LD r, n
    ld A, $12   ; A should become $12
    ld H, $AB   ; H should become $AB

    ;; Test LD (HL), r
    ld A, $34   ; Load A with $34
    ld HL, $C000 ; Load HL with address $C000
    ld [HL], A  ; Memory at $C000 should become $34

    ;; Test LD r, (HL)
    ld HL, $C000 ; Point HL to $C000 (which should contain $34)
    ld B, [HL]  ; B should become $34

    ;; Infinite loop to halt execution
end_loop:
    jp end_loop
