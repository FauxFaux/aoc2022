mod d1;
mod d2;
mod d3;

fn main() {
    match 3 {
        1 => d1::solve(),
        2 => d2::solve(),
        3 => d3::solve(),
        _ => unreachable!(),
    }
}
