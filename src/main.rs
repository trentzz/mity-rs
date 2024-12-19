mod check;
mod mity_util;

mod call;
mod normalise;
mod report;
mod merge;
mod runall;

use call::Call;
use normalise::Normalise;
use clap::{Arg, ArgAction, Command};

fn handle_call_command(call_matches: &clap::ArgMatches) {
    let debug = call_matches.get_flag("debug");
    let files = call_matches
        .get_many::<String>("files")
        .expect("Required argument")
        .map(|s| s.to_string())
        .collect();
    let reference = call_matches
        .get_one::<String>("reference")
        .expect("Required argument");
    let prefix = call_matches
        .get_one::<String>("prefix")
        .map(|s| s.to_string());
    let min_mq = call_matches.get_one::<u32>("min_mapping_quality").copied();
    let min_bq = call_matches.get_one::<u32>("min_base_quality").copied();
    let min_af = call_matches
        .get_one::<f32>("min_alternate_fraction")
        .copied();
    let min_ac = call_matches.get_one::<u32>("min_alternate_count").copied();
    let p_val = call_matches.get_one::<f32>("p").copied();
    let output_dir = call_matches
        .get_one::<String>("output_dir")
        .expect("Required argument")
        .to_string();
    let region = call_matches
        .get_one::<String>("region")
        .map(|s| s.to_string());
    let bam_file_list = call_matches.get_flag("bam_file_list");
    let keep = call_matches.get_flag("keep");
    let normalise = call_matches.get_flag("normalise");

    let reference_fasta = mity_util::select_reference_fasta(reference, None).unwrap();
    let reference_genome = mity_util::select_reference_genome(reference, None).unwrap();

    // Create the Call struct using the new constructor
    let mut call = Call::new(
        debug,
        files,
        reference_fasta,
        Some(reference_genome),
        prefix,
        min_mq,
        min_bq,
        min_af,
        min_ac,
        p_val,
        normalise,
        output_dir,
        region,
        bam_file_list,
        keep,
    );

    // TODO: think of better semantics for error handling and logging
    match call.run() {
        Ok(()) => {
            println!("Call command completed successfully.");
        }
        Err(e) => {
            eprintln!("Error executing call command: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_normalise_command(normalise_matches: &clap::ArgMatches) {
    // Extract and parse command-line arguments
    let debug = normalise_matches.get_flag("debug");
    let vcf = normalise_matches
        .get_one::<String>("vcf")
        .expect("Required argument 'vcf' is missing")
        .to_string();
    let reference = normalise_matches
        .get_one::<String>("reference")
        .expect("Required argument 'reference' is missing")
        .to_string();
    let output_dir = normalise_matches
        .get_one::<String>("output_dir")
        .expect("Required argument 'output_dir' is missing")
        .to_string();
    let prefix = normalise_matches
        .get_one::<String>("prefix")
        .map(|s| s.to_string());
    let allsamples = normalise_matches.get_flag("allsamples");
    let p_val = normalise_matches.get_one::<f32>("p").unwrap().clone();
    let keep = normalise_matches.get_flag("keep");

    // Select reference files using utility functions
    let reference_fasta = mity_util::select_reference_fasta(&reference, None)
        .expect("Failed to select reference FASTA");
    let reference_genome = mity_util::select_reference_genome(&reference, None)
        .expect("Failed to select reference genome");

    // Create the Normalise struct using the constructor
    let normalise = Normalise::new(
        debug,
        vcf,
        reference_fasta,
        reference_genome,
        output_dir,
        prefix,
        allsamples,
        keep,
        p_val,
    );

    // Execute the normalization process and handle any potential errors
    match normalise.run() {
        Ok(()) => {
            println!("Normalisation command completed successfully.");
        }
        Err(e) => {
            eprintln!("Error executing normalisation command: {}", e);
            std::process::exit(1);
        }
    }
}


fn cli_commands() {
    // Reused args
    let debug_arg = Arg::new("debug")
        .short('d')
        .long("debug")
        .action(ArgAction::SetTrue)
        .help("Enter debug mode");

    let keep_arg = Arg::new("keep")
        .short('k')
        .long("keep")
        .action(ArgAction::SetTrue)
        .help("Keep all intermediate files");

    let output_dir_arg = Arg::new("output_dir")
        .long("output-dir")
        .action(ArgAction::Set)
        .value_name("OUTPUT_DIR")
        .default_value(".")
        .help("Output files will be saved in OUTPUT_DIR. Default: '.'");

    let reference_arg = Arg::new("reference")
        .long("reference")
        .action(ArgAction::Set)
        .value_name("GENOME")
        .value_parser(["hs37d5", "hg19", "hg38", "mm10"])
        .default_value("hs37d5")
        .help("Reference genome version to use. Default: hs37d5");

    let files_arg = Arg::new("files")
        .action(ArgAction::Append)
        .required(true)
        .help("BAM / CRAM files to run the analysis on. If --bam-file-list is included, this argument is the file containing the list of BAM/CRAM files");

    let prefix_arg = Arg::new("prefix")
        .long("prefix")
        .action(ArgAction::Set)
        .required(true)
        .help("Output files will be named with PREFIX");

    let vcf_arg = Arg::new("vcf")
        .help("VCF.GZ file from running mity")
        .required(true);

    // Call arguments
    let min_mapping_quality_arg = Arg::new("min_mapping_quality")
        .long("min-mapping-quality")
        .help("Exclude alignments with a mapping quality less than this value. Default: 30")
        .default_value("30")
        .value_parser(clap::value_parser!(u32));

    let min_base_quality_arg = Arg::new("min_base_quality")
        .long("min-base-quality")
        .help("Exclude alleles with a base quality less than this value. Default: 24")
        .default_value("24")
        .value_parser(clap::value_parser!(u32));

    let min_alternate_fraction_arg = Arg::new("min_alternate_fraction")
        .long("min-alternate-fraction")
        .help("Require at least this fraction of observations supporting an alternate allele. Default: 0.01")
        .default_value("0.01")
        .value_parser(clap::value_parser!(f32));

    let min_alternate_count_arg = Arg::new("min_alternate_count")
        .long("min-alternate-count")
        .help("Require at least this many observations supporting an alternate allele. Default: 4")
        .default_value("4")
        .value_parser(clap::value_parser!(u32));

    let call_p_arg = Arg::new("p")
        .long("p")
        .help("Minimum noise level for calculating QUAL score. Default: 0.002")
        .default_value("0.002")
        .value_parser(clap::value_parser!(f32));

    let region_arg = Arg::new("region")
        .long("region")
        .action(ArgAction::Set)
        .help("Region of MT genome to call variants in. Default: entire MT genome.");

    let bam_file_list_arg = Arg::new("bam_file_list")
        .long("bam-file-list")
        .action(ArgAction::SetTrue)
        .help("Treat the input file as a text file listing BAM files.");

    // Report args
    let min_vaf_arg = Arg::new("min_vaf")
        .long("min_vaf")
        .action(ArgAction::Set)
        .value_name("FLOAT")
        .value_parser(clap::value_parser!(f64))
        .default_value("0")
        .help("A variant must have at least this VAF to be included in the report. Default: 0.");

    let contig_arg = Arg::new("contig")
        .long("contig")
        .action(ArgAction::Set)
        .value_name("CONTIG")
        .value_parser(["MT", "chrM"])
        .default_value("MT")
        .help("Contig used for annotation purposes");

    let vcfanno_config = Arg::new("vcfanno_config")
        .long("custom-vcfanno-config")
        .action(ArgAction::Set)
        .value_name("TOML_FILE")
        .help("Provide a custom vcfanno-config.toml for custom annotations.");

    let report_config_arg = Arg::new("report_config")
        .long("custom-report-config")
        .action(ArgAction::Set)
        .value_name("YAML_FILE")
        .help("Provide a custom report-config.yaml for custom report generation.");

    // Main commands
    let call_command = Command::new("call")
        .about("Call mitochondrial variants")
        .arg(debug_arg.clone())
        .arg(files_arg.clone())
        .arg(reference_arg.clone())
        .arg(prefix_arg.clone())
        .arg(min_mapping_quality_arg.clone())
        .arg(min_base_quality_arg.clone())
        .arg(min_alternate_fraction_arg.clone())
        .arg(min_alternate_count_arg.clone())
        .arg(call_p_arg.clone())
        .arg(output_dir_arg.clone())
        .arg(region_arg.clone())
        .arg(bam_file_list_arg.clone())
        .arg(keep_arg.clone())
        .arg(
            Arg::new("normalise")
                .long("normalise")
                .action(ArgAction::SetTrue)
                .help("Run mity normalise on the resulting VCF"),
        );

    let normalise_command = Command::new("normalise")
        .about("Normalise & filter mitochondrial variants")
        .arg(debug_arg.clone())
        .arg(vcf_arg.clone())
        .arg(output_dir_arg.clone())
        .arg(prefix_arg.clone())
        .arg(
            Arg::new("allsamples")
                .long("allsamples")
                .action(ArgAction::SetTrue)
                .help("PASS requires all samples to pass"),
        )
        .arg(keep_arg.clone())
        .arg(call_p_arg.clone())
        .arg(reference_arg.clone());

    let report_command = Command::new("report")
        .about("Generate mity report")
        .arg(debug_arg.clone())
        .arg(prefix_arg.clone())
        .arg(output_dir_arg.clone())
        .arg(vcf_arg.clone())
        .arg(keep_arg.clone())
        .arg(contig_arg.clone())
        .arg(min_vaf_arg.clone())
        .arg(vcfanno_config.clone())
        .arg(report_config_arg.clone());

    let merge_command = Command::new("merge")
        .about("Merge mity and nuclear VCF files")
        .arg(
            Arg::new("mity_vcf")
                .long("mity_vcf")
                .action(ArgAction::Set)
                .value_name("FILE")
                .required(true)
                .help("mity vcf file"),
        )
        .arg(
            Arg::new("nuclear_vcf")
                .long("nuclear_vcf")
                .action(ArgAction::Set)
                .value_name("FILE")
                .required(true)
                .help("nuclear vcf file"),
        )
        .arg(output_dir_arg.clone())
        .arg(prefix_arg.clone())
        .arg(reference_arg.clone())
        .arg(debug_arg.clone())
        .arg(keep_arg.clone());

    let runall_command = Command::new("runall")
        .about("Run analysis on BAM/CRAM files")
        .arg(debug_arg.clone())
        .arg(files_arg.clone())
        .arg(reference_arg.clone())
        .arg(prefix_arg.clone())
        .arg(min_mapping_quality_arg.clone())
        .arg(min_base_quality_arg.clone())
        .arg(min_alternate_fraction_arg.clone())
        .arg(min_alternate_count_arg.clone())
        .arg(call_p_arg.clone())
        .arg(output_dir_arg.clone())
        .arg(region_arg.clone())
        .arg(bam_file_list_arg.clone())
        .arg(keep_arg.clone())
        .arg(min_vaf_arg.clone())
        .arg(contig_arg.clone())
        .arg(vcfanno_config.clone())
        .arg(report_config_arg.clone());

    let check_command =
        Command::new("check").about("Run runtime checks for mity-rs dependencies and features");

    let matches = Command::new("mity-rs")
        .version("1.0")
        .about("Mity RS: Mitochondrial variant analysis toolkit in rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(call_command)
        .subcommand(normalise_command)
        .subcommand(report_command)
        .subcommand(merge_command)
        .subcommand(runall_command)
        .subcommand(check_command)
        .get_matches();

    match matches.subcommand() {
        Some(("call", call_matches)) => {
            handle_call_command(call_matches);
        }
        Some(("normalise", normalise_matches)) => {
            handle_normalise_command(normalise_matches);
        }
        Some(("report", report_matches)) => {
            // Handle the 'normalise' subcommand
            println!("Not implemented yet!");
            println!("{:?}", report_matches);
        }
        Some(("merge", merge_matches)) => {
            // Handle the 'normalise' subcommand
            println!("Not implemented yet!");
            println!("{:?}", merge_matches);
        }
        Some(("runall", runall_matches)) => {
            // Handle the 'normalise' subcommand
            println!("Not implemented yet!");
            println!("{:?}", runall_matches);
        }
        Some(("check", _)) => {
            check::mity_check();
        }
        _ => unreachable!(),
    }
}

fn main() {
    cli_commands();
}
