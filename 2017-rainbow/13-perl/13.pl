#!/usr/bin/perl

use strict;
use warnings;

sub parse
{
    open(FH, '<', $_[0]) or die $!;

    my @pos = ();
    my @dpt = ();

    while(<FH>) {
        my @vals = split(": ", $_);
        push(@pos, $vals[0]);
        push(@dpt, $vals[1]);
    }

    return \@pos, \@dpt;
}

sub part_one
{
    my @pos = @{$_[0]};
    my @dpt = @{$_[1]};
    my $score = 0;

    for (my $i = 0; $i <= $#pos; $i++) {
        if (!($pos[$i] % (($dpt[$i] - 1) * 2))) {
            $score += $pos[$i] * $dpt[$i];
        }
    }

    return $score;
}

sub part_two
{
    my @pos = @{$_[0]};
    my @dpt = @{$_[1]};
    my $delay = 0;

    for (my $i = 0; $i <= $#pos; $i++) {
        if (!(($pos[$i] + $delay) % (($dpt[$i] - 1) * 2))) {
            $i = -1;
            $delay++;
        }
    }

    return $delay;
}

my (@pos, @dpt) = parse('../input/13');

print("Part one: ", part_one(@pos, @dpt), "\n");
print("Part two: ", part_two(@pos, @dpt), "\n");
