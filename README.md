## Pileup region
This is a simple rust application for counting the number of alt-reads and depths at locations
provided in a 1-based region file. It works for CRAM and BAMs. This has not been tested extensively. 

### Usage

After running `cargo build --release` :

`./target/release/pileup_region <region_file> <cram> <fasta> `

The `<region file>` should have the following columns: `chr, pos, ref, alt, aa_change` . You 
can leave `aa_change` as blank or a dummy value if you don't need it. The column headers need to
match these exactly. 

An example file is provided here for the U2AF1 locus (`u2af1_vars.txt`) 

### Install
cargo build --release

### Authors
Contact [Josh Weinstock](https://github.com/weinstockj)

