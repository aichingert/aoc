print_u8 :: fn(n: i32) {
    $asm(
        "mov rax, 1",
        "lea rsi, n",
        "mov rdi, 1",
        "mov rdx, 8",
        "syscall"
    )
}

print_num :: fn(num: i64) {
    n := num

    if n < 0 {
        print_u8(45)
        n = -n
    }

    out := [0,0,0,0,0,0,0,0,0,0]
    len := 0

    while n != 0 {
        $asm("mov rdx, 0")
        div := n / 10

        rem := 0
        $asm("mov rem, rdx")

        out[len] = rem + 48
        n = div
        len = len + 1
    }

    i := 0
    if len == 0 {
        print_u8(48)
    }

    while i < len {
        print_u8(out[len - i - 1])
        i = i + 1
    }

    print_u8(10)
}

main :: fn() {
    block_size :: 5
    keys  := [5, 0, 2, 1, 3, 4, 3, 4, 0, 2, 3, 0, 2, 0, 1]
    key_len :: 15
    locks := [0, 5, 3, 4, 3, 1, 2, 0, 5, 3]
    lock_len :: 10

    i := 0
    ans := 0

    while i < key_len {
        j := 0

        while j < lock_len {
            k := 0
            is_valid := 1

            while k < block_size {
                down :: keys[i + k]
                up   :: locks[j + k]

                if down + up > block_size {
                    is_valid = 0
                }

                k = k + 1
            }

            if is_valid {
                ans = ans + 1
            }
            j = j + block_size
        }

        i = i + block_size
    }

    txt :: [115, 111, 108, 58, 32]
    txt_i := 0

    while txt_i < 5 {
        print_u8(txt[txt_i])
        txt_i = txt_i + 1
    }

    print_num(ans)
}
