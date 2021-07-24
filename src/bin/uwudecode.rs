use std::fs::{read_to_string, write};
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use uwucodec::decode;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// The uwuencoded file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// The file to write the decoded output to
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}
fn main() -> io::Result<()> {
    let opt: Opt = Opt::from_args();
    let encoded_data = read_to_string(opt.input)?;
    let data = decode(&encoded_data);
    write(opt.output, data)
}
