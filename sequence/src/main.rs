extern crate chan;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate rand;

use std::io::{self, Write};
use std::u8;

use clap::{App, Arg};
use rand::{Rng, XorShiftRng};

use errors::*;

#[allow(unused_doc_comment)]
mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

fn main() {
    run().unwrap();
}

const N: usize = 24;

fn run() -> Result<()> {
    let brightness = Arg::with_name("brightness")
        .short("b")
        .required(true)
        .takes_value(true);
    let color = Arg::with_name("color").required(true).index(1);
    let fps = Arg::with_name("fps")
        .short("f")
        .value_name("FPS")
        .takes_value(true);
    let matches = App::new("sequence")
        .subcommand(App::new("crescendo").arg(color.clone()).arg(fps.clone()))
        .subcommand(App::new("rainbow").arg(brightness).arg(fps.clone()))
        .subcommand(App::new("random").arg(fps.clone()))
        .subcommand(App::new("roulette").arg(color.clone()).arg(fps.clone()))
        .subcommand(App::new("single").arg(color.clone()))
        .get_matches();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    if let Some(crescendo) = matches.subcommand_matches("crescendo") {
        let color = parse(crescendo.value_of("color").unwrap())?;

        let fps: u8 = crescendo.value_of("fps").unwrap().parse().unwrap();
        let timer = chan::tick_ms(1_000 / fps as u32);

        let mut pos = 0;
        let mut sz = 1;
        let mut grow = true;
        loop {
            let mut bytes = [0; 3 * N];
            for (i, chunk) in bytes.chunks_mut(3).enumerate() {
                if pos + sz <= N {
                    if i >= pos && i < pos + sz {
                        chunk.copy_from_slice(&color);
                    }
                } else {
                    if i < (pos + sz) % N || i >= pos {
                        chunk.copy_from_slice(&color);
                    }
                }
            }

            stdout.write_all(&bytes)?;
            stdout.flush()?;

            if pos == N {
                pos = 0;
                if grow {
                    sz += 1;

                    if sz == N - 1 {
                        grow = false;
                        sz -= 1;
                    }
                } else {
                    sz -= 1;

                    if sz == 0 {
                        sz = 1;
                        grow = true;
                    }
                }
            } else {
                pos += 1;
            }

            timer.recv();
        }
    } else if let Some(rainbow) = matches.subcommand_matches("rainbow") {
        let b: u8 = rainbow.value_of("brightness").unwrap().parse().unwrap();
        let fps: u8 = rainbow.value_of("fps").unwrap().parse().unwrap();

        let mut bytes = [0; 24 * N];

        let mut pos = 0;
        let timer = chan::tick_ms(1_000 / fps as u32);
        loop {
            for (i, led) in bytes.chunks_mut(3).enumerate() {
                let color = match (i + pos) % 24 {
                    0...3 => [b, 0, 0],
                    4...7 => [b, b, 0],
                    8...11 => [0, b, 0],
                    12...15 => [0, b, b],
                    16...19 => [0, 0, b],
                    20...23 => [b, 0, b],
                    _ => unreachable!(),
                };
                led.copy_from_slice(&color);
            }

            pos = (pos + 1) % 24;

            stdout.write_all(&bytes)?;
            stdout.flush()?;

            timer.recv();
        }
    } else if let Some(random) = matches.subcommand_matches("random") {
        let mut rng = rng();

        let fps: u8 = random.value_of("fps").unwrap().parse().unwrap();
        let timer = chan::tick_ms(1_000 / fps as u32);

        let mut bytes = [0; 3 * N];
        loop {
            rng.fill_bytes(&mut bytes);

            stdout.write_all(&bytes)?;
            stdout.flush()?;

            timer.recv();
        }
    } else if let Some(roulette) = matches.subcommand_matches("roulette") {
        let color = parse(roulette.value_of("color").unwrap())?;

        let fps: u8 = roulette.value_of("fps").unwrap().parse().unwrap();
        let timer = chan::tick_ms(1_000 / fps as u32);

        let mut pos = 0;
        loop {
            let mut bytes = [0; 3 * N];

            bytes[3 * pos..3 * (pos + 1)].copy_from_slice(&color);
            pos = (pos + 1) % N;

            stdout.write_all(&bytes)?;
            stdout.flush()?;
            timer.recv();
        }
    } else if let Some(single) = matches.subcommand_matches("single") {
        let color = parse(single.value_of("color").unwrap())?;

        let mut bytes = [0; 3 * N];
        for chunk in bytes.chunks_mut(3) {
            chunk.copy_from_slice(&color);
        }

        stdout.write_all(&bytes)?;
        stdout.flush()?;
    }

    Ok(())
}

fn parse(color: &str) -> Result<[u8; 3]> {
    const MSG: &str = "Color string must have the format 'AB10FF'";

    if color.len() == 6 {
        u8::from_str_radix(&color[..2], 16)
            .and_then(|r| {
                u8::from_str_radix(&color[2..4], 16).and_then(
                    |g| u8::from_str_radix(&color[4..6], 16).map(|b| [r, g, b]),
                )
            })
            .chain_err(|| MSG)
    } else {
        bail!(MSG);
    }
}

fn rng() -> XorShiftRng {
    rand::thread_rng().gen()
}
