mod d1;
mod d2;
mod d3;
mod d4;

fn main() {
    match 4 {
        1 => d1::solve(),
        2 => d2::solve(),
        3 => d3::solve(),
        4 => d4::solve(),
        _ => unreachable!(),
    }
}
