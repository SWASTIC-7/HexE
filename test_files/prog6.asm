TEST6   START   5000
        +JSUB   SUBRTN
        +LDA    BIGVAL
        RSUB
SUBRTN  LDA     #123
        RSUB
BIGVAL  WORD    12345
        END     TEST6
