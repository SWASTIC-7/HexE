TEST3   START   2000
        LDA     #10      ; immediate
        STA     VALUE
        LDA     @VALUE   ; indirect
        ADD     #5
        STA     RESULT
VALUE   WORD    7
RESULT  RESW    1
        END     TEST3
