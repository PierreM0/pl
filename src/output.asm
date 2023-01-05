
BITS 64
%define SYS_EXIT 60
segment .text
global _start
put:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 64
        mov     QWORD [rbp-56], rdi
        mov     DWORD [rbp-4], 1
        mov     eax, DWORD [rbp-4]
        cdqe
        mov     edx, 32
        sub     rdx, rax
        mov     BYTE [rbp-48+rdx], 10
.L0:
        mov     rcx, QWORD [rbp-56]
        mov     rdx, 7378697629483820647
        mov     rax, rcx
        imul    rdx
        sar     rdx, 2
        mov     rax, rcx
        sar     rax, 63
        sub     rdx, rax
        mov     rax, rdx
        sal     rax, 2
        add     rax, rdx
        add     rax, rax
        sub     rcx, rax
        mov     rdx, rcx
        mov     eax, edx
        lea     ecx, [rax+48]
        mov     eax, DWORD [rbp-4]
        lea     edx, [rax+1]
        mov     DWORD [rbp-4], edx
        cdqe
        mov     edx, 31
        sub     rdx, rax
        mov     eax, ecx
        mov     BYTE [rbp-48+rdx], al
        mov     rcx, QWORD [rbp-56]
        mov     rdx, 7378697629483820647
        mov     rax, rcx
        imul    rdx
        mov     rax, rdx
        sar     rax, 2
        sar     rcx, 63
        mov     rdx, rcx
        sub     rax, rdx
        mov     QWORD [rbp-56], rax
        cmp     QWORD [rbp-56], 0
        jg      .L0
        mov     eax, DWORD [rbp-4]
        cdqe
        mov     edx, DWORD [rbp-4]
        movsxd  rdx, DWORD edx
        mov     ecx, 32
        sub     rcx, rdx
        lea     rdx, [rbp-48]
        add     rcx, rdx
        mov     rdx, rax
        mov     rsi, rcx
        mov     edi, 1
        mov     rax, 1
        syscall
        nop
        leave
        ret
_start:
        push    rbp
        mov     rbp, rsp
        sub     rsp, 16

.L2:
        mov    rbx, 35
        mov    r10, 34
        add    r10, rbx
        mov    rdi, r10
        call put

.L3:
        mov    rbx, 500
        mov    r10, 80
        sub    rbx, r10
        mov    rdi, rbx
        call put

.L4:
        mov    rbx, 4
        mov    r10, 3
        mov    rax, r10
        mul    rbx
        mov    r10, rax
        mov    rdi, r10
        call put

.L5:
        mov    rbx, 32
        mov    r10, 3
        mov    rdx, 0
        mov    rax, rbx
        div    r10
        mov    rbx, rax
        mov    rdi, rbx
        call put

.L6:
        mov    rbx, 3
        mov    r10, 4
        mov    rax, r10
        mul    rbx
        mov    r10, rax
        mov    r11, 21
        add    r11, r10
        mov    rdi, r11
        call put
.LEND:
        mov     rdi, 0
        mov    rax, 60
        syscall