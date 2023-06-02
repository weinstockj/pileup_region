use std::env;
use rust_htslib::bam;
use rust_htslib::bam::{Read};
/* use log::{info}; */
/* use chrono::prelude::*; */
use csv;
use serde::Deserialize;
use std::str;
use std::char;
use std::io;

#[derive(Deserialize, Debug)]
struct PosRecord {
    aa_change: String,
    #[serde(rename = "chr")] // rename chr -> chrom
    chrom: String,
    pos: u32,
    r#ref: String, // 'ref' is a reserved word, so escape with r#
    alt: String,
}


fn main() {
    // ./pileup_region  <region_file, 1-based pos> <cram> <fasta>
    let args: Vec<String>  = env::args().collect();
    let region_path = &args[1];
    let cram_path = &args[2];
    let fasta_path = &args[3];

    
    /* info!("now running for bam/cram {}", cram_path); */

    let mut bam_reader = bam::IndexedReader::from_path(&cram_path).unwrap();
    let bam_header = bam_reader.header().clone();

    let is_bam_file = cram_path.contains("bam");

    bam_reader.set_reference(fasta_path).unwrap();

    /* let max_read_length = 151; */

    let mut pos_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(&region_path).unwrap();

    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(io::BufWriter::new(io::stdout()));

    for (_i, record) in pos_reader.deserialize().enumerate() {
        
        let record: PosRecord = record.unwrap();

        // pileup over all covered ites
        let tid = bam_header.tid(record.chrom.as_bytes()).unwrap();


        let start = (record.pos - 1) as u64; // bam is 0-based, region_path is 1 based
                                             //
        if is_bam_file {
            bam_reader.fetch(tid, start, start + 1).unwrap(); // bam has half-closed-half-open
        } else {
            bam_reader.fetch(tid, start, start).unwrap(); // cram file has closed intervals?
        }

        let mut alt_count = 0;

        // println!("here 1.1, tid = {}, start = {}", tid, start);
        // used https://github.com/rust-bio/rust-bio-tools/blob/81eb3dc9655a6bcd0ed50b3ee9cf28ecaddcb778/src/bam/depth.rs as reference
        for p in bam_reader.pileup() {
            // println!("p = {:?}", p);
            let pileup = p.unwrap();

            if pileup.pos() == record.pos - 1 {

                /* println!("{:?}:{} depth {}",  */
                /*     str::from_utf8(bam_header.tid2name(pileup.tid())).unwrap(), */
                /*     pileup.pos(), */
                /*     pileup.depth() */
                /* ); */
                
                for alignment in pileup.alignments() {
                    if !alignment.is_del() && !alignment.is_refskip() {
                        /* println!("Base {}", alignment.record().seq()[alignment.qpos().unwrap()]); */
                        let base = alignment.record().seq()[alignment.qpos().unwrap()] as char;
                        if base.to_string().to_uppercase() == record.alt {
                            alt_count += 1;
                        }
                    }
                }

                csv_writer.serialize(
                    (&record.chrom, record.pos, &record.r#ref, &record.alt, &record.aa_change, pileup.depth(), alt_count)
                ).unwrap();

            }

            /*     // mark indel start */
            /*     match alignment.indel() { */
            /*         bam::pileup::Indel::Ins(len) => println!("Insertion of length {} between this and next position.", len), */
            /*             bam::pileup::Indel::Del(len) => println!("Deletion of length {} between this and next position.", len), */
            /*             bam::pileup::Indel::None => () */
            /*     } */
            /* } */
        }

    } 
}
