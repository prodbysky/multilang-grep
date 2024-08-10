#!/usr/bin/perl
use strict;
use warnings;

my $file_name = $ARGV[0];
my $needle = $ARGV[1];

open(FH, '<', $file_name) or die $!;

while(<FH>) {
    if (index($_, $needle) != -1) {
        print $_;
    }
}
