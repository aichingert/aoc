inp = open("../../../input/2024/25").read()

keys = ""
key_len = 0
locks = ""
lock_len = 0

for m in inp.strip().split("\n\n"):
    cur = [0,0,0,0,0]
    lines = m.split("\n")

    for i in range(1, len(lines) - 1, 1):
        for j in range(len(lines[i].strip())):
            if lines[i][j] == "#":
                cur[j] += 1

    if m[0][0] == "#":
        for e in cur:
            if len(locks) > 0:
                locks += ", "
            locks += str(e)
        lock_len += len(cur)
    else:
        for e in cur:
            if len(keys) > 0:
                keys += ", "
            keys += str(e)
        key_len += len(cur)

content = f"""print_u8 :: (n: i32) {{
    $asm(
        "mov rax, 1",
        "lea rsi, n",
        "mov rdi, 1",
        "mov rdx, 8",
        "syscall"
    )
}}

print_num :: (num: i64) {{
    n := num

    if n < 0 {{
        print_u8(45)
        n = -n
    }}

    out := [0,0,0,0,0,0,0,0,0,0]
    len := 0

    while n != 0 {{
        $asm("mov rdx, 0")
        div := n / 10

        rem := 0
        $asm("mov rem, rdx")

        out[len] = rem + 48
        n = div
        len = len + 1
    }}

    i := 0
    if len == 0 {{
        print_u8(48)
    }}

    while i < len {{
        idx := len
        idx = idx - i
        idx = idx - 1

        print_u8(out[idx])
        i = i + 1
    }}

    print_u8(10)
}}

main :: () {{
    block_size :: 5
    keys  := [{keys}]
    key_len :: {key_len}
    locks := [{locks}]
    lock_len :: {lock_len}

    i := 0
    ans := 0

    while i < key_len {{
        j := 0

        while j < lock_len {{
            k := 0
            is_valid := 1

            while k < block_size {{
                down :: keys[i + k]
                up   :: locks[j + k]

                if down + up > block_size {{
                    is_valid = 0
                }}

                k = k + 1
            }}

            if is_valid {{
                ans = ans + 1
            }}
            j = j + block_size
        }}

        i = i + block_size
    }}

    txt :: [115, 111, 108, 58, 32]
    txt_i := 0

    while txt_i < 5 {{
        print_u8(txt[txt_i])
        txt_i = txt_i + 1
    }}

    print_num(ans)
}}
"""

f = open("comp.mo", "w")
f.write(content)
