use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

fn cli_commands() {
    let call_command = Command::new("call")
        .about("Call mitochondrial variants").arg(Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Enter debug mode"),
        )
        .arg(Arg::new("files")
                .help("BAM / CRAM files to run the analysis on. If --bam-file-list is included, this argument is the file containing the list of bam/cram files.").required(true)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("reference")
                .long("reference")
                .value_parser(["hs37d5", "hg19", "hg38", "mm10"])
                .default_value("hs37d5")
                .help("Reference genome version to use. Default: hs37d5"),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .action(ArgAction::Set)
                .help("Output files will be named with PREFIX")
        )
        .arg(
            Arg::new("min_mapping_quality")
                .long("min-mapping-quality")
                .help("Exclude alignments with a mapping quality less than this value. Default: 30")
                .default_value("30")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("min_base_quality")
                .long("min-base-quality")
                .help("Exclude alleles with a base quality less than this value. Default: 24")
                .default_value("24")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("min_alternate_fraction")
                .long("min-alternate-fraction")
                .help("Require at least this fraction of observations supporting an alternate allele. Default: 0.01")
                .default_value("0.01")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            Arg::new("min_alternate_count")
                .long("min-alternate-count")
                .help("Require at least this many observations supporting an alternate allele. Default: 4")
                .default_value("4")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("p")
                .long("p")
                .help("Minimum noise level for calculating QUAL score. Default: 0.002")
                .default_value("0.002")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            Arg::new("normalise")
                .long("normalise")
                .action(ArgAction::SetTrue)
                .help("Run mity normalise on the resulting VCF"),
        )
        .arg(
            Arg::new("output_dir")
                .long("output-dir")
                .help("Output files will be saved in this directory. Default: '.'")
                .default_value(".")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("region")
                .long("region")
                .action(ArgAction::Set)
                .help("Region of MT genome to call variants in. Default: entire MT genome.")
        )
        .arg(
            Arg::new("bam_file_list")
                .long("bam-file-list")
                .action(ArgAction::SetTrue)
                .help("Treat the input file as a text file listing BAM files."),
        )
        .arg(
            Arg::new("keep")
                .short('k')
                .long("keep")
                .action(ArgAction::SetTrue)
                .help("Keep all intermediate files"),
        );

    let normalise_command = Command::new("normalise")
        .about("Normalise & filter mitochondrial variants")
        .arg(
            Arg::new("vcf")
                .help("VCF.GZ file from running mity")
                .required(true),
        )
        .arg(
            Arg::new("output_dir")
                .long("output-dir")
                .default_value(".")
                .help("Output directory. Default: '.'")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .help("Output files will be named with PREFIX"),
        )
        .arg(
            Arg::new("allsamples")
                .long("allsamples")
                .action(ArgAction::SetTrue)
                .help("PASS requires all samples to pass"),
        )
        .arg(
            Arg::new("keep")
                .short('k')
                .long("keep")
                .action(ArgAction::SetTrue)
                .help("Keep all intermediate files"),
        )
        .arg(
            Arg::new("p")
                .long("p")
                .default_value("0.002")
                .value_parser(clap::value_parser!(f64))
                .help("Minimum noise level for QUAL score calculation. Default: 0.002"),
        )
        .arg(
            Arg::new("reference")
                .long("reference")
                .value_parser(["hs37d5", "hg19", "hg38", "mm10"])
                .default_value("hs37d5")
                .help("Reference genome version to use. Default: hs37d5"),
        );

    let matches = Command::new("mity")
        .version("1.0")
        .about("Mity: Mitochondrial variant analysis toolkit")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(call_command)
        .subcommand(normalise_command)
        .get_matches();

    match matches.subcommand() {
        Some(("call", call_matches)) => {
            // Handle the 'call' subcommand
            println!("{:?}", call_matches);
        }
        Some(("normalise", normalise_matches)) => {
            // Handle the 'normalise' subcommand
            println!("{:?}", normalise_matches);
        }
        _ => unreachable!(),
    }
}

fn main() {
    cli_commands();
}
