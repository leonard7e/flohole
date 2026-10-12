# flohole
This command line tools tries to be useful for Flutemakers. It calculates the locations for fingerholes.
It takes several parameters to calculate fingerhole positions for a flute.

It requires the following positional argument:

*   `LENGTH`: The total length of the flute in mm (positional argument, required).

Options:

*   `-b`, `--labium`: The length of the labium (embouchure hole) in mm.
*   `-f`, `--fraction`: A fraction used in the calculation of fingerhole positions.
*   `-k`, `--key`: Key transposition in semitones (default: 0.0). Must be greater than or equal to 0.
*   `-p`, `--position`: A position value in halftones, used in the calculation.
*   `-s`, `--scale`: The scale to be used for the flute (e.g., "major", "minor").
*   `-n`, `--nholes`: The number of fingerholes to calculate.
*   `-t`, `--tune`: Tuning adjustments for individual fingerholes in cents. Use the format "h0=value, h1=-value, … , h_n=value". Values are cents. E.g., to adjust hole 0 by +50 cents and hole 2 by -10 cents, use "h0=50, h2=-10".

The program outputs the calculated positions of the fingerholes, taking into account the specified parameters and tuning adjustments.
