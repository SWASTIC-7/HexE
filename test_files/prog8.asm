TEST8   START   7000
ONE     EQU     1
TWO     EQU     ONE+1
        ORG     7100
        LDA     #TWO
        STA     NUM
        ORG
NUM     RESW    1
        END     TEST8
