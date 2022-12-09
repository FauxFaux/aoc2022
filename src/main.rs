mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

fn main() {
    match 9 {
        1 => d1::solve(),
        2 => d2::solve(),
        3 => d3::solve(),
        4 => d4::solve(),
        5 => d5::solve(),
        6 => d6::solve(),
        7 => d7::solve(),
        8 => d8::solve(),
        9 => d9::solve(),
        _ => unreachable!(),
    }
}
