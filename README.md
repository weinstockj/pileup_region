## Pileup region
This is a simple rust application for counting the number of alt-reads and depths at locations
provided in a 1-based region file. It works for CRAM and BAMs. This has not been tested extensively. 

### Usage

`./pileup_counter <region_file> <cram> <fasta> `

The `<region file>` should have the following columns: `chrom, pos, ref, alt, aa_change` . You 
can leave `aa_change` as blank or a dummy value if don't need it. 

### Install
cargo build --release

### Authors
Contact [Josh Weinstock](https://github.com/weinstockj)

