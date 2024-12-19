use log::{debug, info, LevelFilter};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, remove_file};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

// Constants
const P_VAL: f64 = 0.002;
const SB_RANGE_LO: f64 = 0.1;
const SB_RANGE_HI: f64 = 0.9;
const MIN_MQMR: f64 = 30.0;
const MIN_AQR: f64 = 20.0;
const MIN_DP: i32 = 15;
const BLACKLIST: [i32; 20] = [302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 3105, 3106, 3107];

pub struct Normalise {
    debug: bool,
    vcf: String,
    reference_fasta: String,
    genome: String,
    output_dir: String,
    prefix: Option<String>,
    allsamples: bool,
    keep: bool,
    p: f32,
    
    bcftools_norm_path: PathBuf,
    filtered_vcf_path: PathBuf,
    normalised_vcf_path: PathBuf,
}

impl Normalise {
    pub fn new(debug: bool, vcf: String, reference_fasta: String, genome: String, output_dir: String, prefix: Option<String>, allsamples: bool, keep: bool, p: f32) -> Self {
        let mut normalise = Normalise {
            debug,
            vcf: vcf.clone(),
            reference_fasta,
            genome,
            output_dir: output_dir.clone(),
            prefix,
            allsamples,
            keep,
            p,
            
            bcftools_norm_path: PathBuf::new(),
            filtered_vcf_path: PathBuf::new(),
            normalised_vcf_path: PathBuf::new(),
        };
        normalise.set_paths();
        normalise
    }

    pub fn run(&self)-> Result<(), Box<dyn Error>> {
        if self.debug {
            log::set_max_level(LevelFilter::Debug);
            debug!("Entered debug mode.");
        } else {
            log::set_max_level(LevelFilter::Info);
        }

        self.run_bcftools_norm();
        self.run_filtering();

        // Placeholder for MityUtil::gsort logic
        // MityUtil::gsort(self.filtered_vcf_path.clone(), self.normalised_vcf_path.clone(), self.genome.clone());

        self.remove_intermediate_files();

        Ok(())
    }

    fn run_bcftools_norm(&self) {
        // Placeholder for running bcftools command
        debug!("Running bcftools norm");
        // Simulate command: bcftools norm -f {} -m-both {}
        let output = format!("Simulating bcftools norm on {} with reference {}", self.vcf, self.reference_fasta);
        let mut file = File::create(&self.bcftools_norm_path).expect("Failed to create bcftools norm file");
        file.write_all(output.as_bytes()).expect("Failed to write bcftools norm output");
    }

    fn run_filtering(&self) {
        // Placeholder for filtering logic (e.g., parsing VCF and filtering variants)
        debug!("Running filtering");
        
        // Simulate file writing logic
        let mut file = File::create(&self.filtered_vcf_path).expect("Failed to create filtered VCF file");
        file.write_all(b"Simulated filtered VCF data").expect("Failed to write filtered VCF file");
    }

    fn set_paths(&mut self) {
        if self.prefix.is_none() {
            self.prefix = Some(self.make_prefix(&self.vcf));
        }
        
        let prefix = self.prefix.clone().unwrap();
        self.bcftools_norm_path = PathBuf::from(&self.output_dir).join(format!("{}.bcftools.norm.vcf.gz", prefix));
        self.filtered_vcf_path = PathBuf::from(&self.vcf.replace(".vcf.gz", ".filtered.vcf"));
        self.normalised_vcf_path = PathBuf::from(&self.output_dir).join(format!("{}.normalise.vcf.gz", prefix));
    }

    fn make_prefix(&self, vcf: &str) -> String {
        let parts: Vec<&str> = vcf.split('/').collect();
        parts.last().unwrap_or(&"default").to_string()
    }

    fn remove_intermediate_files(&self) {
        if !self.keep {
            if self.filtered_vcf_path.exists() {
                remove_file(&self.filtered_vcf_path).expect("Failed to remove filtered VCF file");
            }
            if self.bcftools_norm_path.exists() {
                remove_file(&self.bcftools_norm_path).expect("Failed to remove bcftools norm file");
            }
        }
    }
}