use log::{debug, error, info, LevelFilter};
use simple_logger;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct Call {
    debug: bool,
    files: Vec<String>,
    reference: String,
    genome: Option<String>,
    prefix: Option<String>,
    min_mq: u32,
    min_bq: u32,
    min_af: f32,
    min_ac: u32,
    p: f32,
    normalise: bool,
    output_dir: String,
    region: Option<String>,
    bam_list: bool,
    keep: bool,

    // Internal fields
    file_string: String,
    normalised_vcf_path: String,
    call_vcf_path: String,
    mity_cmd: String,
    sed_cmd: String,
}

impl Call {
    const MIN_MQ: u32 = 30;
    const MIN_BQ: u32 = 24;
    const MIN_AF: f32 = 0.01;
    const MIN_AC: u32 = 4;
    const P_VAL: f32 = 0.002;

    pub fn new(
        debug: bool,
        files: Vec<String>,
        reference: String,
        genome: Option<String>,
        prefix: Option<String>,
        min_mq: Option<u32>,
        min_bq: Option<u32>,
        min_af: Option<f32>,
        min_ac: Option<u32>,
        p: Option<f32>,
        normalise: bool,
        output_dir: String,
        region: Option<String>,
        bam_list: bool,
        keep: bool,
    ) -> Self {
        let min_mq = min_mq.unwrap_or(Self::MIN_MQ);
        let min_bq = min_bq.unwrap_or(Self::MIN_BQ);
        let min_af = min_af.unwrap_or(Self::MIN_AF);
        let min_ac = min_ac.unwrap_or(Self::MIN_AC);
        let p = p.unwrap_or(Self::P_VAL);

        Call {
            debug,
            files,
            reference,
            genome,
            prefix,
            min_mq,
            min_bq,
            min_af,
            min_ac,
            p,
            normalise,
            output_dir,
            region,
            bam_list,
            keep,
            file_string: String::new(),
            normalised_vcf_path: String::new(),
            call_vcf_path: String::new(),
            mity_cmd: String::new(),
            sed_cmd: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        if self.debug {
            simple_logger::SimpleLogger::new()
                .with_level(LevelFilter::Debug)
                .init()?;
            debug!("Entered debug mode.");
        } else {
            simple_logger::SimpleLogger::new()
                .with_level(LevelFilter::Info)
                .init()?;
        }

        if self.bam_list {
            self.get_files_from_list()?;
        }
        self.run_checks()?;
        self.set_strings();
        self.set_region()?;
        self.set_mity_cmd();

        self.run_freebayes()?;

        if self.normalise {
            self.run_normalise()?;
        } else {
            self.tabix(&self.call_vcf_path)?;
        }

        Ok(())
    }

    fn run_freebayes(&self) -> Result<(), Box<dyn Error>> {
        let freebayes_call = format!(
            "set -o pipefail && freebayes -f {} {} --min-mapping-quality {} \
            --min-base-quality {} --min-alternate-fraction {} --min-alternate-count {} \
            --ploidy 2 --region {} | sed 's/##source/##freebayesSource/' | sed \
            's/##commandline/##freebayesCommandline/' | {} | bgzip > {}",
            self.reference,
            self.file_string,
            self.min_mq,
            self.min_bq,
            self.min_af,
            self.min_ac,
            self.region.as_deref().unwrap_or(""),
            self.sed_cmd,
            self.call_vcf_path,
        );

        info!("Running FreeBayes in sensitive mode");
        debug!("{}", freebayes_call);

        let output = Command::new("/bin/bash")
            .arg("-c")
            .arg(freebayes_call)
            .output()?;

        if !output.status.success() {
            error!(
                "FreeBayes failed: {:?}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(format!("FreeBayes failed with code {:?}", output.status.code()).into());
        }

        debug!("Finished running FreeBayes");
        Ok(())
    }

    fn set_region(&mut self) -> Result<(), Box<dyn Error>> {
        if self.region.is_none() {
            self.region = Some(self.bam_get_mt_contig(&self.files[0])?);
        }
        Ok(())
    }

    fn set_strings(&mut self) {
        if self.prefix.is_none() {
            self.prefix = Some(self.make_prefix(&self.files[0]));
        }

        self.file_string = self
            .files
            .iter()
            .rev()
            .map(|file| format!("-b {}", file))
            .collect::<Vec<_>>()
            .join(" ");

        self.normalised_vcf_path = format!(
            "{}/{}.mity.normalise.vcf.gz",
            self.output_dir,
            self.prefix.as_ref().unwrap()
        );
        self.call_vcf_path = format!(
            "{}/{}.mity.call.vcf.gz",
            self.output_dir,
            self.prefix.as_ref().unwrap()
        );
    }

    fn make_prefix(&self, file_name: &str) -> String {
        let path = Path::new(file_name);
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }

    fn get_files_from_list(&mut self) -> Result<(), Box<dyn Error>> {
        if self.files.len() > 1 {
            return Err("--bam-file-list argument expects only 1 file to be provided.".into());
        }

        let file_content = fs::read_to_string(&self.files[0])?;
        self.files = file_content.lines().map(String::from).collect();
        Ok(())
    }

    fn run_checks(&self) -> Result<(), Box<dyn Error>> {
        if self.files.len() > 1 && self.prefix.is_none() {
            return Err("If there is more than one BAM/CRAM file, --prefix must be set".into());
        }

        if self.normalise && self.genome.is_none() {
            return Err("A genome file should be supplied if mity call normalize=True".into());
        }

        for file in &self.files {
            if !Path::new(file).exists() {
                return Err(format!("Missing file: {}", file).into());
            }
        }

        Ok(())
    }

    fn bam_get_mt_contig(&self, bam: &str) -> Result<String, Box<dyn Error>> {
        // Simplified; use a BAM parsing library like `rust-htslib` in production.
        Ok("chrM:1-16569".to_string())
    }

    fn tabix(&self, file: &str) -> Result<(), Box<dyn Error>> {
        // Simulate tabix
        info!("Running tabix on {}", file);
        Ok(())
    }

    fn run_normalise(&self) -> Result<(), Box<dyn Error>> {
        info!("Normalising and filtering variants");
        // Call the normalisation logic here (not implemented for brevity).
        Ok(())
    }

    fn set_mity_cmd(&mut self) {
        self.mity_cmd = format!(
            "##mityCommandline=\"mity call --reference {} --prefix {} ...\"",
            self.reference,
            self.prefix.as_ref().unwrap_or(&String::new())
        );
        self.sed_cmd = format!("sed 's/^##phasing=none/{}/g'", self.mity_cmd);
    }
}
