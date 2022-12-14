#![feature(array_windows)]
#![feature(array_chunks)]

extern crate core;

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

fn main() {
    match 22 {
        1 => d1::solve(),
        2 => d2::solve(),
        3 => d3::solve(),
        4 => d4::solve(),
        5 => d5::solve(),
        6 => d6::solve(),
        7 => d7::solve(),
        8 => d8::solve(),
        9 => d9::solve(),
        10 => d10::solve(),
        11 => d11::solve(),
        12 => d12::solve(),
        13 => d13::solve(),
        14 => d14::solve(),
        15 => d15::solve(),
        16 => d16::solve(),
        17 => d17::solve(),
        18 => d18::solve(),
        19 => d19::solve(),
        20 => d20::solve(),
        21 => d21::solve(),
        22 => d22::solve(),
        23 => d23::solve(),
        24 => d24::solve(),
        25 => d25::solve(),
        _ => unreachable!(),
    }
}
