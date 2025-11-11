COPY    START   1000
        USE     MAIN
FIRST   STL     RETADR
CLOOP   JSUB    RDREC
        LDA     LENGTH
        COMP    ZERO
        JEQ     ENDFIL
        JSUB    WRREC
        J       CLOOP

ENDFIL  LDA     ZERO        
        STA     LENGTH
        RSUB

        USE     DATA
ZERO    WORD    0
RETADR  RESW    1
LENGTH  RESW    1

        USE     CODE
RDREC   RESW    1
WRREC   RESW    1

        END     FIRST
