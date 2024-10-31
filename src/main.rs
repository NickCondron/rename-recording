#[allow(unused_variables, dead_code)]
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "Rename Recording")]
#[command(version = "0.1")]
#[command(about = "Automatically rename Training Mode recording .gci files", long_about = None)]
struct Cli {
    /// File name format string
    ///
    /// Describes how to rename the recording file. Automatically appends the
    /// '.gci' extension. The string is interpreted literally except for the
    /// follwing codes:
    /// %n - Recording name (as seen in game import menu)
    /// %h - Human character name
    /// %c - CPU character name
    /// %d - Date as YYYY-MM-DD
    // TODO
    // %v version
    // %s - Stage name
    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT_STR",
        default_value = "%n",
        verbatim_doc_comment
    )]
    format: String,
    #[arg(short = 'i', long = "in-place", help = "Rename in place")]
    in_place: bool,

    #[arg(required(true), help = "File(s) to rename")]
    files: Vec<PathBuf>,
}

// Decode code taken from https://github.com/AlexanderHarrison/tm_replay
const ENCODE_LUT: [u32; 13] = [
    0x26, 0xFF, 0xE8, 0xEF, 0x42, 0xD6, 0x01, 0x54, 0x14, 0xA3, 0x80, 0xFD, 0x6E,
];

fn deobfuscate_byte(r3: u8, r4: u8) -> u8 {
    let b = r3 as u32;
    let mut r4 = r4 as u32;

    let r5;
    match b % 7 {
        0 => {
            r5 = ((r4 & 0x01) << 0)
                | ((r4 & 0x02) << 1)
                | ((r4 & 0x04) << 2)
                | ((r4 & 0x08) << 3)
                | ((r4 & 0x10) >> 3)
                | ((r4 & 0x20) >> 2)
                | ((r4 & 0x40) >> 1)
                | ((r4 & 0x80) >> 0);
            r4 = r5 & 0xFF;
        }
        1 => {
            r5 = ((r4 & 0x01) << 1)
                | ((r4 & 0x02) << 6)
                | ((r4 & 0x04) << 0)
                | ((r4 & 0x08) >> 3)
                | ((r4 & 0x10) << 1)
                | ((r4 & 0x20) >> 1)
                | ((r4 & 0x40) >> 3)
                | ((r4 & 0x80) >> 1);
            r4 = r5 & 0xFF;
        }
        2 => {
            r5 = ((r4 & 0x01) << 2)
                | ((r4 & 0x02) << 2)
                | ((r4 & 0x04) << 4)
                | ((r4 & 0x08) << 1)
                | ((r4 & 0x10) << 3)
                | ((r4 & 0x20) >> 4)
                | ((r4 & 0x40) >> 6)
                | ((r4 & 0x80) >> 2);
            r4 = r5 & 0xFF;
        }
        3 => {
            r5 = ((r4 & 0x01) << 4)
                | ((r4 & 0x02) >> 1)
                | ((r4 & 0x04) << 3)
                | ((r4 & 0x08) >> 2)
                | ((r4 & 0x10) >> 1)
                | ((r4 & 0x20) << 1)
                | ((r4 & 0x40) << 1)
                | ((r4 & 0x80) >> 5);
            r4 = r5 & 0xFF;
        }
        4 => {
            r5 = ((r4 & 0x01) << 3)
                | ((r4 & 0x02) << 4)
                | ((r4 & 0x04) >> 1)
                | ((r4 & 0x08) << 4)
                | ((r4 & 0x10) << 2)
                | ((r4 & 0x20) >> 3)
                | ((r4 & 0x40) >> 2)
                | ((r4 & 0x80) >> 7);
            r4 = r5 & 0xFF;
        }
        5 => {
            r5 = ((r4 & 0x01) << 5)
                | ((r4 & 0x02) << 5)
                | ((r4 & 0x04) << 5)
                | ((r4 & 0x08) >> 0)
                | ((r4 & 0x10) >> 2)
                | ((r4 & 0x20) >> 5)
                | ((r4 & 0x40) >> 5)
                | ((r4 & 0x80) >> 3);
            r4 = r5 & 0xFF;
        }
        6 => {
            r5 = ((r4 & 0x01) << 6)
                | ((r4 & 0x02) << 0)
                | ((r4 & 0x04) >> 2)
                | ((r4 & 0x08) << 2)
                | ((r4 & 0x10) << 0)
                | ((r4 & 0x20) << 2)
                | ((r4 & 0x40) >> 4)
                | ((r4 & 0x80) >> 4);
            r4 = r5 & 0xFF;
        }
        _ => unreachable!(),
    }

    r4 ^= ENCODE_LUT[(b % 13) as usize];
    r4 ^= r3 as u32;
    return r4 as u8;
}

fn decode_block(src: &mut [u8]) -> i32 {
    let mut checksum = [0u8; 16];
    let mut x = src[15];
    for i in 16..src.len() {
        let y = src[i];
        src[i] = deobfuscate_byte(x, y);
        x = y;
    }
    calculate_checksum(&src[16..], &mut checksum);

    for i in 0..16 {
        if src[i] != checksum[i] {
            return -1;
        }
    }

    return 0;
}

fn calculate_checksum(src: &[u8], result: &mut [u8]) {
    let mut checksum: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];

    for i in 0..src.len() {
        checksum[i % 16] = checksum[i % 16].wrapping_add(src[i]);
    }

    for i in 1..16 {
        if checksum[i - 1] == checksum[i] {
            checksum[i] ^= 0xFF;
        }
    }

    result[0..16].copy_from_slice(&checksum);
}

fn char_name(char: u8) -> Option<&'static str> {
    match char {
        0 => Some("Falcon"),
        1 => Some("DK"),
        2 => Some("Fox"),
        3 => Some("GnW"),
        4 => Some("Kirby"),
        5 => Some("Bowser"),
        6 => Some("Link"),
        7 => Some("Luigi"),
        8 => Some("Mario"),
        9 => Some("Marth"),
        10 => Some("Mewtwo"),
        11 => Some("Ness"),
        12 => Some("Peach"),
        13 => Some("Pikachu"),
        14 => Some("ICs"),
        15 => Some("Puff"),
        16 => Some("Samus"),
        17 => Some("Yoshi"),
        18 => Some("Zelda"),
        19 => Some("Sheik"),
        20 => Some("Falco"),
        21 => Some("YLink"),
        22 => Some("Doc"),
        23 => Some("Roy"),
        24 => Some("Pichu"),
        25 => Some("Ganon"),
        _ => None,
    }
}

fn valid_format_str(format: &str) -> bool {
    if format.len() < 1 {
        return false;
    }

    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('n') | Some('h') | Some('c') | Some('d') => continue,
                _ => return false,
            }
        }
    }
    return true;
}

fn main() {
    let args = Cli::parse();

    if !valid_format_str(&args.format) {
        eprintln!("Invalid format string");
        std::process::exit(1);
    }

    for file in args.files {
        let Ok(mut gci) = std::fs::read(&file) else {
            eprintln!("Could not read input file {:?}", file);
            continue;
        };

        if gci.get(0..6).is_none_or(|s| s != "GTME01".as_bytes())
            || gci.get(8..14).is_none_or(|s| s != "TMREC_".as_bytes())
        {
            eprintln!("{:?} is not a recording file", file);
            continue;
        }

        // decode first block which starts at 0x1EB0 and is 400 bytes long
        let Some(mut block) = gci.get_mut(0x1EB0..0x2040) else {
            eprintln!(
                "{:?} is shorter than expected. Ensure it is a gci file",
                file
            );
            continue;
        };
        if decode_block(&mut block) != 0 {
            eprintln!("Could not decode {:?}. Ensure it is a gci file", file);
            continue;
        }

        // offsets according to training mode code plus 0x20
        let hmn = block[0x28];
        let cpu = block[0x2a];
        let _stage = u16::from_be_bytes([block[0x2c], block[0x2d]]);
        let month = block[0x30];
        let day = block[0x31];
        let year = u16::from_be_bytes([block[0x32], block[0x33]]);
        let name = &block[0x37..0x57];

        let null = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let name = String::from_utf8((&name[..null]).to_vec()).unwrap();

        let mut new_name = String::with_capacity(100);
        let mut format_chars = args.format.chars().peekable();
        while let Some(c) = format_chars.next() {
            if c == '%' {
                match format_chars.next() {
                    Some('n') => new_name.push_str(&name),
                    Some('h') => {
                        let short_char = char_name(hmn).expect("Invalid hmn character");
                        new_name.push_str(short_char);
                    }
                    Some('c') => {
                        let short_char = char_name(cpu).expect("Invalid cpu character");
                        new_name.push_str(short_char);
                    }
                    Some('d') => {
                        let date = format!("{:04}-{:02}-{:02}", year, month, day);
                        new_name.push_str(&date);
                    }
                    _ => unreachable!(), // we checked validity above
                }
            } else {
                new_name.push(c);
            }
        }
        new_name.push_str(".gci");

        if args.in_place {
            match std::fs::rename(&file, &new_name) {
                Ok(_) => {
                    println!("Renamed {:?} to {:?}", file, new_name);
                }
                Err(e) => {
                    eprintln!("Could not rename {:?}: {}", file, e);
                    continue;
                }
            }
        } else {
            match std::fs::copy(&file, &new_name) {
                Ok(_) => {
                    println!("Copied {:?} to {:?}", file, new_name);
                }
                Err(e) => {
                    eprintln!("Could not copy {:?}: {}", file, e);
                    continue;
                }
            }
        }
    }
}
