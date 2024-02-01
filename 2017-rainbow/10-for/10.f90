program day10
implicit none

    integer :: io
    integer :: idx = 1
    integer :: pos = 1
    integer :: skip_size = 0

    integer, dimension(4) :: lengths ! 16
    integer, dimension(5) :: buffer  ! 256

    open (newunit=io, file='../input/10', status='old', action='read')
    read (io, *) lengths
    close(io)

    do idx = 1,5
        buffer(idx) = idx - 1
    end do
    

    idx = 1
    do while (idx < 5)
        call rev(pos, lengths(idx), buffer, 5)
        pos = modulo(pos + lengths(idx) + skip_size, 6)
        skip_size = skip_size + 1
        idx = idx + 1
        print *, buffer
    end do

end program day10

subroutine rev(src, dst, data, size)
implicit none

integer, intent (in)  :: src, dst, size
integer, intent (out) :: data(size)

    integer :: tmp, sp, dp

    sp = src
    dp = dst

    do while (sp /= dp .and. sp + 1 /= dp)
        tmp = data(sp)
        data(sp) = data(dp)
        data(dp) = tmp

        sp = modulo(sp + 1, size + 1)
        dp = dp - 1

        if (dp < 0) dp = size
    end do
end subroutine rev
