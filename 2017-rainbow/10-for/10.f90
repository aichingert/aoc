program day10
implicit none

    integer :: io
    integer :: idx = 1
    integer :: pos = 1
    integer :: skip_size = 0

    integer :: over = 0
    integer :: len_size = 16
    integer :: buffer_size = 256

    integer, dimension(16) :: lengths ! 16
    integer, dimension(256) :: buffer  ! 256

    open (newunit=io, file='../input/10', status='old', action='read')
    read (io, *) lengths
    close(io)

    do idx = 1,buffer_size
        buffer(idx) = idx - 1
    end do

    idx = 1
    do while (idx <= len_size)
        call rev(pos, pos + lengths(idx) - 1, buffer, buffer_size)

        pos = pos + lengths(idx) + skip_size
        skip_size = skip_size + 1
        idx = idx + 1

        if (pos > buffer_size) then 
            over = pos / buffer_size
            pos = modulo(pos, buffer_size + 1) + over
        end if
    end do

    print *, "Part one: ", buffer(1) * buffer(2)
end program day10

subroutine rev(src, dst, data, size)
implicit none

integer, intent (in)  :: src, dst, size
integer, intent (out) :: data(size)

    integer :: lpsz, tmp, sp, dp, ov

    lpsz = (dst - src) / 2
    sp = src
    dp = dst

    do while (lpsz > 0)
        if (dp > size) then
            ov = dp / size
            dp = modulo(dp, size + 1) + ov
        else if (dp < 1) then
            dp = size
        end if
        if (sp > size) then
            sp = modulo(sp, size + 1) + 1
        end if

        tmp = data(sp)
        data(sp) = data(dp)
        data(dp) = tmp

        sp = sp + 1
        dp = dp - 1
        lpsz = lpsz - 1
    end do
end subroutine rev
