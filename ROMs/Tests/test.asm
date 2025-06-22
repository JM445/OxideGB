SECTION "Test code", ROM0[0x100]

nop                             ;
ld b, $1                        ;
sla b                           ;
                                ;
nop                             ;
ld b, $10                       ;
sla b                           ;

nop                             ;
ld b, $FF                       ;
sla b                           ;

loop:
    jr loop
