use std::fs::{read, write};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use uwucodec::encode;

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
    let data = read(opt.input)?;
    let encoded_data = encode(&data);
    write(opt.output, encoded_data)
}
