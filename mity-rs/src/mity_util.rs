use anyhow::{Context, Result};
use glob::glob;
use log::debug;
use noodles::vcf;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the directory path of the Mity library.
pub fn get_mity_dir() -> Result<PathBuf> {
    let module_path = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = Path::new(&module_path).join("mitylib");
    Ok(path)
}

/// Generate a tabix index for a bgzipped file.
pub fn tabix(file: &str) -> Result<()> {
    let tabix_call = format!("tabix -f {}", file);
    debug!("{}", tabix_call);
    Command::new("sh")
        .arg("-c")
        .arg(tabix_call)
        .status()
        .context("Failed to run tabix command")?;
    Ok(())
}

/// Select the reference genome fasta file.
pub fn select_reference_fasta(
    reference: &str,
    custom_reference_fa: Option<&str>,
) -> Result<String> {
    if let Some(custom_path) = custom_reference_fa {
        if Path::new(custom_path).exists() {
            return Ok(custom_path.to_string());
        }
    }
    let ref_dir = get_mity_dir()?.join("reference");
    let pattern = format!("{}/{}.fa", ref_dir.display(), reference);
    let files: Vec<_> = glob(&pattern)?.collect();
    debug!("{:?}", files);
    if files.len() != 1 {
        anyhow::bail!(
            "Expected exactly one reference fasta file, found: {:?}",
            files
        );
    }
    Ok(files[0].as_ref().unwrap().to_str().unwrap().to_string())
}

/// Select the reference genome .genome file.
pub fn select_reference_genome(
    reference: &str,
    custom_reference_genome: Option<&str>,
) -> Result<String> {
    if let Some(custom_path) = custom_reference_genome {
        if Path::new(custom_path).exists() {
            return Ok(custom_path.to_string());
        }
    }
    let ref_dir = get_mity_dir()?.join("reference");
    let pattern = format!("{}/{}.genome", ref_dir.display(), reference);
    let files: Vec<_> = glob(&pattern)?.collect();
    debug!("{:?}", files);
    if files.len() != 1 {
        anyhow::bail!(
            "Expected exactly one reference genome file, found: {:?}",
            files
        );
    }
    Ok(files[0].as_ref().unwrap().to_str().unwrap().to_string())
}

/// Get the mitochondrial contig name and length from a VCF file.
pub fn vcf_get_mt_contig(vcf_path: &str) -> Result<(String, usize)> {
    let mut reader = vcf::io::reader::Builder::default().build_from_path(vcf_path)?;
    let header = reader.read_header()?;
    let contigs = header.contigs();
    let mito_contig: Vec<&String> = contigs
        .keys()
        .filter(|key| key == &"MT" || key == &"chrM")
        .collect();
    if mito_contig.len() != 1 {
        anyhow::bail!(
            "Expected exactly one mitochondrial contig, found: {:?}",
            mito_contig
        );
    }
    let contig = mito_contig[0].to_string();
    let length = contigs[contig.as_str()].length().unwrap_or(0);
    Ok((contig, length))
}

/// Get the path to an annotation file.
pub fn get_annot_file(annotation_file_path: &str) -> Result<String> {
    let mitylib_dir = get_mity_dir()?;
    let path = mitylib_dir.join("annot").join(annotation_file_path);
    if !path.exists() {
        anyhow::bail!("Annotation file not found: {}", path.display());
    }
    Ok(path.to_string_lossy().into_owned())
}

/// Make a prefix based on the input VCF path.
pub fn make_prefix(vcf_path: &str) -> String {
    Path::new(vcf_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .replace(".mity", "")
        .replace(".call", "")
        .replace(".normalise", "")
        .replace(".merge", "")
        .replace(".report", "")
        .replace(".vcf.gz", "")
}

/// Run gsort.
pub fn gsort(input_path: &str, output_path: &str, genome: &str) -> Result<()> {
    let gsort_cmd = format!(
        "gsort {} {} | bgzip -cf > {}",
        input_path, genome, output_path
    );
    debug!("{}", gsort_cmd);
    Command::new("sh")
        .arg("-c")
        .arg(gsort_cmd)
        .status()
        .context("Failed to run gsort command")?;
    tabix(output_path)?;
    Ok(())
}
