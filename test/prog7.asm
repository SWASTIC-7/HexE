TEST7   START   6000
        LDB     #TABLE
        BASE    TABLE
        LDA     TABLE
        STA     RESULT
        NOBASE
TABLE   RESW    5
RESULT  RESW    1
        END     TEST7
