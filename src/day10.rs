
//      North
// West         Each
//      South

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    NorthSouth,     // |
    EastWest,       // -
    NorthEast,      // L
    NorthWest,      // J
    SouthWest,      // 7
    SouthEast,      // F
    Ground,         // .
    StartingPoint,  // S
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::StartingPoint,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = ".....
                 .S-7.
                 .|.|.
                 .L-J.
                 .....";

    let mut tiles: Vec<Tile> = vec![];
    for line in input.lines() {
        for ch in line.trim().chars() {
            tiles.push(ch.into());
        }
    }

    let start = tiles.iter().find(|tile| **tile == Tile::StartingPoint).unwrap();

    println!("{tiles:?}");
}
