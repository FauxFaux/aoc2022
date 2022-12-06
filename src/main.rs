mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;

fn main() {
    match 6 {
        1 => d1::solve(),
        2 => d2::solve(),
        3 => d3::solve(),
        4 => d4::solve(),
        5 => d5::solve(),
        6 => d6::solve(),
        _ => unreachable!(),
    }
}
