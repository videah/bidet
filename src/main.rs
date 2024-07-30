use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{anyhow, bail, Context};
use log::Level;
use rsubs_lib::{SRT, VTT};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bidet", about = "Small .ass subtitle conversion utility.")]
struct Opt {
    /// Input subtitle file (can be either .srt or .vtt)
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output .ass subtitle file
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Disable output on successful conversion
    #[structopt(short, long)]
    quiet: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let level = if opt.quiet { Level::Error } else { Level::Info };

    simple_logger::init_with_level(level)?;

    // Check that the input file exists
    if !opt.input.exists() {
        bail!(
            "Input subtitle file path '{}' does not exist.",
            opt.input
                .to_str()
                .ok_or(anyhow!("Input subtitle file path is not valid UTF-8."))?
        );
    }

    // Get the file extension of the input file.
    let input_extension = opt
        .input
        .extension()
        .ok_or(anyhow!("Input subtitle file does not have an extension."))?
        .to_str()
        .ok_or(anyhow!("Input subtitle file extension is not valid UTF-8."))?;

    let input_file = std::fs::read_to_string(&opt.input)
        .with_context(|| format!("Failed to read input file: {:?}", opt.input))?;

    let converted_input = match input_extension.to_lowercase().as_str() {
        "srt" => SRT::parse(&input_file)?.to_ssa(),
        "vtt" => VTT::parse(&input_file)?.to_ssa(),
        _ => bail!("Unsupported input file format: {}", input_extension),
    };

    let output_path = match opt.output {
        Some(output) => output,
        None => {
            let mut output = opt.input.clone();
            output.set_extension("ass");
            output
        }
    };

    let mut output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create output file: {:?}", output_path))?;

    output_file
        .write_all(converted_input.to_string().as_bytes())
        .with_context(|| format!("Failed to write to output file: {:?}", output_path))?;

    log::info!("Successfully converted:");
    log::info!("    From: {:?}", opt.input.file_name().unwrap());
    log::info!("    To: {:?}", output_path.file_name().unwrap());

    Ok(())
}
