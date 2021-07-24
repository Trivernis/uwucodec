use std::fs::OpenOptions;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use uwucodec::encode_stream;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// The input file with the data to encode
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The file to write the uwuencoded data to
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}
fn main() -> io::Result<()> {
    let opt: Opt = Opt::from_args();
    let input_file = OpenOptions::new().read(true).open(opt.input)?;
    let mut input_reader = BufReader::new(input_file);
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(opt.output)?;
    let mut output_writer = BufWriter::new(output_file);
    encode_stream(&mut input_reader, &mut output_writer)?;
    output_writer.flush()
}
