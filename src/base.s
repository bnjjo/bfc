.intel_syntax noprefix

.bss
arr:
        .space 30000

.SECTION .text
        .global _start

_start:
        // REGISTERS
        // rax -> used for sys_write
        // rbx -> array pointer
        // rcx -> stores values of each cell
        // rdx -> used for sys_write

        // set up array pointer
        mov     rbx, OFFSET arr
        // assign reference to first array position to rcx
        mov     [rbx], rcx
        // start off at the middle of the array
        add     rbx, 14999
        // prep for sys_write
        push    rcx
        mov     rax, 1
        mov     rdi, 1
        mov     rsi, rsp
        mov     rdx, 1
        // this is necessary (pop null value from stack)
        pop     rcx
        syscall
l1: # time for loops
        



        // exit instructions
        mov     ax, 60
        syscall

; .intel_syntax noprefix
;
; .SECTION .data
; mem_buffer:
;         .zero 30000
;
; .SECTION .text
;         .global _start
;
; _start:
;         // set up array pointer
;         mov     rbx, OFFSET mem_buffer
;
;         // add ascii 'a' to rcx register
;         add     rcx, 97
;         // move the value of 'a' to first position
;         mov     [rbx], rcx
;         push    rcx
;
;         // prep for sys_write
;         mov     rax, 1
;         mov     rdi, 1
;         mov     rsi, rsp
;         mov     rdx, 1
;
;         // write to stdout (.)
;         syscall
;         pop     rcx
;
;         // increment array pointer by one and add 98
;         add     rbx, 1
;         mov     rcx, [rbx]
;         add     rcx, 98
;         mov     [rbx], rcx
;
;         // print
;         push    rcx
;         syscall
;         pop     rcx
;
;         // decrement array pointer by one
;         sub     rbx, 1
;         mov     rcx, [rbx]
;
;         // print
;         push    rcx
;         syscall
;         pop     rcx
;
;         add     rbx, 1
;         mov     rcx, [rbx]
;         push    rcx
;         syscall
;         pop     rcx
;
;         // exit instructions
;         mov     rax, 60
;         syscall
;
;         // FOR SYSCALL SYS_WRITE
;         // rax has to be 1
;         // rdi has to be 1
;         // rsi has to be the value of the thing being printed (use lea)
;         // rdx has to be the length of the value being printed (in bytes I think?)
