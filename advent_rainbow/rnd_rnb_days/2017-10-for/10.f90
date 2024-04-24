! Some numbers for this day are somewhat hardcoded since I think they are 
! most likely the same lenght for everyone and I did not figure out how
! to do this better. So if you run into any problems try to change the numbers

program day10
implicit none

    integer           :: p1, part_one
    character(len=32) :: p2, part_two

    p1 = part_one()
    p2 = part_two()

    print *, 'Part one: ', p1
    print *, 'PART TWO WARNING: you have to fill in the empty places with 0 and write letters small'
    print *, 'Part two: ', p2
end program day10

function part_one() result(ans)
implicit none

    integer :: io, skip_size = 0, pos = 1, ans
    integer :: l_size = 16, b_size = 256

    integer, dimension(16)  :: lengths
    integer, dimension(256) :: buffer

    open(newunit=io, file='../input/10', status='old', action='read')
    read(io, *) lengths
    close(io)

    do skip_size = 1,b_size
        buffer(skip_size) = skip_size - 1
    end do
    skip_size = 0

    call shuffle(pos, skip_size, lengths, l_size, buffer, b_size)

    ans = buffer(1) * buffer(2)
end function part_one

function part_two() result(hash)
implicit none
    
    integer :: io, idx = 1, pos = 1, skip_size = 0, acc = 0
    integer :: l_size = 0, b_size = 256

    integer, dimension(:), allocatable :: lengths
    integer, dimension(256) :: buffer

    character(len=512) :: input
    character(len=32)  :: hash

    open(newunit=io, file='../input/10', status='old', action='read')
    read(io, '(A)') input
    close(io)

    l_size = len_trim(input) + 5

    allocate ( lengths(l_size) )

    do idx = 1,b_size
        buffer(idx) = idx - 1
    end do

    do idx = 1,l_size - 5
        lengths(idx) = ichar(input(idx:idx))
    end do

    lengths(l_size - 4:l_size) = [17,31,73,47,23]

    do idx = 1,64
        call shuffle(pos, skip_size, lengths, l_size, buffer, b_size)
    end do

    do idx = 0,15
        acc = 0
        do skip_size =1,16
            acc = xor(acc, buffer(idx * 16 + skip_size))
        end do

        write (hash(idx*2+1:idx*2+2), '(Z02)') acc
    end do

    deallocate (lengths)
end function part_two



subroutine shuffle(pos, skip_size, lengths, l_size, buffer, b_size)
implicit none
    
integer, intent (in)  :: l_size, b_size, lengths(l_size)
integer, intent (out) :: skip_size, pos, buffer(b_size)

    integer :: i
    i = 1

    do while (i <= l_size)
        call rev(pos, pos + lengths(i) - 1, buffer, b_size)

        pos = pos + lengths(i) + skip_size
        skip_size = skip_size + 1
        i = i + 1

        if (pos > b_size) pos = modulo(pos, b_size + 1) + pos / (b_size + 1)
    end do
end subroutine shuffle

subroutine rev(src, dst, data, size)
implicit none

integer, intent (in)  :: src, dst, size
integer, intent (inout) :: data(size)

    integer :: lpsz, tmp, sp, dp

    lpsz = (dst - src) / 2
    sp = src
    dp = dst

    do while (lpsz >= 0)
        if (dp > size) then
            dp = modulo(dp, size + 1) + 1
        else if (dp < 1) then
            dp = size
        end if

        if (sp > size) sp = modulo(sp, size + 1) + 1

        tmp = data(sp)
        data(sp) = data(dp)
        data(dp) = tmp

        sp = sp + 1
        dp = dp - 1
        lpsz = lpsz - 1
    end do
end subroutine rev
