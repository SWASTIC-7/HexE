TEST2   START   1000
        LDX     #0
LOOP    LDA     DATA,X
        ADD     #1
        STA     DATA,X
        TIX     #5
        JLT     LOOP
DATA    RESW    10
        END     TEST2
