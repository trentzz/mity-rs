use crate::mity_util::{self, select_reference_fasta, select_reference_genome};
use std::path::Path;
use std::process::Command;
use std::thread;

pub fn mity_check() {
    check_required_commands();
    check_threads();
    check_required_reference_files();
}

fn check_required_commands() {
    let required_commands = ["freebayes", "tabix", "gsort"];
    println!("Checking for required commands...");
    for command in &required_commands {
        if !is_command_available(command) {
            eprintln!(
                "Error: Command '{}' is not installed or not in PATH.",
                command
            );
        }
    }
    println!("");
}

fn is_command_available(command: &str) -> bool {
    Command::new("which") // Use "where" on Windows
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn check_threads() {
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("Number of available threads: {}\n", num_threads);
}

fn check_required_reference_files() {
    let genome_options = ["hs37d5", "hg19", "hg38", "mm10"];
    let mity_dir = mity_util::get_mity_dir();
    match mity_dir {
        Ok(_) => {
            println!("Checking for required genome files...");
            for genome in &genome_options {
                let reference_fasta = select_reference_fasta(genome, None);
                if reference_fasta.is_err() {
                    eprintln!("Reference fasta for '{}' is missing.", genome.to_string());
                }

                let reference_genome = select_reference_genome(genome, None);
                if reference_genome.is_err() {
                    eprintln!("Reference genome for '{}' is missing.", genome.to_string());
                }
            }
        }
        Err(_) => eprintln!("Mity directory not found! Likely an issue with installation."),
    }
}
