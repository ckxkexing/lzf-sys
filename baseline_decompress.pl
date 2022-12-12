use strict;
use warnings;
use Compress::LZF;

sub safeDecomp {
    my ($codeC, @rest) = @_;
    try {
        my $code = Compress::LZF::decompress ($codeC);
        return $code;
    } catch Error with {
        my $ex = shift;
        print STDERR "Error: $ex, for parameters @rest\n";
        return "";
    }
}
open(DATA, "<compressed_results.txt");

my $s;
while(<DATA>) {
    $s .= $_;
}

print $s, "\n";

my $t = safeDecomp($s);

print "\n", $t, "\n";

print ord (substr($s,0,1)), "\n";