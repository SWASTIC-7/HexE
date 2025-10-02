TEST4   START   3000
        LDA     =C'EOF'
        STA     BUF
        LDA     =X'05'
        STA     BUF+1
BUF     RESB    10
        END     TEST4
