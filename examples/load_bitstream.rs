use jelly_uidmng as uidmng;
use jelly_fpgautil as fpgautil;

fn main() {
    uidmng::change_user().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <bitstream>", args[0]);
        std::process::exit(1);
    }
    let bitstream = &args[1];
    fpgautil::load_bitstream(bitstream).unwrap();
}
