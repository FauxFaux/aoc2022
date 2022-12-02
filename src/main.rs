mod d1;
mod d2;

fn main() {
    match 2 {
        1 => d1::solve(),
        2 => d2::solve(),
        _ => unreachable!(),
    }
}
