#![feature(fs_try_exists)]
use std::{io::BufReader, fs::{File, self}, time::Duration};

use chrono::{Local, format::{DelayedFormat, StrftimeItems}};
use clap::{Parser, builder::ArgAction};
use rodio::{OutputStream, Decoder, Sink};
use rand::{thread_rng, Rng};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Audio file to be played
    file: String, 

    /// Minimum amount of delay
    #[arg(short, long, default_value_t = 0)]
    from: u64,

    /// Maximum amount of delay
    #[arg(short, long, default_value_t = 1200)]
    until: u64,

    /// Disables random delay
    #[arg(short='r', long="disable_random", action=ArgAction::SetFalse, default_value_t = true)]
    random: bool,

    /// Delay that should be used if random is off. This is added to the length of the sound
    #[arg(short, long)]
    delay: Option<u64>
}

fn main() {
    let args = Args::parse();

    match fs::try_exists(args.file.clone()) {
        Ok(b) => {
            if !b {
                eprintln!("File doesn't exist!");
                return
            }
        },
        Err(_) => {
            eprintln!("File doesn't exist!");
            return
        },
    }

    match Decoder::new(BufReader::new(File::open(args.file.clone()).expect("file to exist"))) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("File is not a valid music file!");
            return
        },
    };

    let (_stream, stream_handle) = OutputStream::try_default().expect("there to be a default device");
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut rng = thread_rng();

    loop {
        let file = BufReader::new(File::open(args.file.clone()).expect("file to exist"));
        let source = Decoder::new(file).expect("file to be valid");

        println!("{} What the dog doin", get_current_time_format());

        sink.append(source);

        sink.sleep_until_end();
        let delay: u64 = if args.random {
            rng.gen_range(args.from..args.until)
        } else {
            match args.delay {
                Some(a) => a,
                None => {
                    eprintln!("Delay not specified! Delay has to be specified when using --disable_random");
                    return
                },
            }
        };
        std::thread::sleep(Duration::from_secs(delay));
    }
}

fn get_current_time_format<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    let now = Local::now();

    now.format("[%H:%M:%S]")
}
