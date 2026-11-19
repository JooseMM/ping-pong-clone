; --- Bootloader Header ---
[org 0x7C00]
    ; --- Step Switch to VGA Graphics Mode 13h ---
    ; Mode 13h = 320x200 resolution, 256 colors
    mov ah, 0x00		; Set video mode function
    mov al, 0x13		; Mode 13h
    int 0x10			; Call BIOS video interrupt

    mov ah, 0x0C		; Write graphics pixel function
    mov al, 0x07		; Color index (0x04 is Red in the default palette)
    mov bh, 0x00		; Page number (usually 0)
    mov cx, 0			; x position

    call game_loop

game_loop:
    call draw_field

check_keys:
    ; 1. Check if a key is waiting in the buffer (non-blocking)
    mov ah, 0x01
    int 0x16
    jz check_keys          	; If ZF = 1, no key was pressed, loop back

    ; 2. Read the key out of the buffer
    mov ah, 0x00
    int 0x16               	; AH = Scancode, AL = ASCII character

    ; 3. Compare AH against the Up Arrow scancode (0x48)
    cmp ah, 0x48           
    je move_up             	; Jump to your handler if it matches

    jmp check_keys         	; Otherwise, keep listening

move_up: 
    
    ret				; return

draw_field:
    ; --- Draw Top Frame ---
    call draw_h_line

    ; --- Draw Left Vertical Frame ---
    mov cx, 0
    mov dx, 5
    call draw_vl_line

    ; --- Draw Right Vertical Frame ---
    mov cx, 315
    mov dx, 5
    call draw_vr_line

    ; --- Draw Bottom Frame ---
    mov cx, 0
    mov dx, 195
    call draw_h_line

    ; --- Draw Center Net ---
    mov cx, 158
    mov dx, 5
    call draw_vc_line

    ret				; return



draw_vc_line:
    int 0x10 			; Draw to cx,dx

    inc cx			; increase x:cx
    cmp cx, 163			; compare cx to 163
    jnz draw_vc_line		; if not 163 repeat

    inc dx			; incrase y:dx
    mov cx, 158			; reset x:cx to 158

    cmp dx, 195			; compare y:dx to 195
    jnz draw_vc_line		; if not 195 repeat

    ret				; return

draw_vl_line:
    int 0x10 			; Draw to cx,dx

    inc cx			; increase x:cx
    cmp cx, 5			; compare cx to 5
    jnz draw_vl_line		; if not 5 repeat

    inc dx			; incrase y:dx
    mov cx, 0			; reset x:cx to 0

    cmp dx, 195			; compare y:dx to 195
    jnz draw_vl_line		; if not 195 repeat

    ret				; return

draw_vr_line:
    int 0x10			; Draw to cx,dx

    inc cx			; increase x:cx
    cmp cx, 320			; compare cx to 5
    jnz draw_vr_line		; if not repeat

    inc dx			; increase y:dx
    mov cx, 315			; reset x:dx to 315

    cmp dx, 195			; compare y:dx to 195
    jnz draw_vr_line		; if not repeat

    ret				; return

draw_h_line:
    int 0x10			; draw cx,dx

    inc cx			; increase x,cx
    cmp cx, 1600		; compare x:cx to 1600
    jnz draw_h_line		; if not repeat

    mov cx, 0			; reset x:cx to 0
    ret				; return

; --- Bootloader Footer ---
times 510-($-$$) db 0 
dw 0xAA55
