pub fn verse(n: i32) -> String {
    let no_bottles = String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    let single_bottle = String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    let two_bottles = String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");

    match n {
        0 => return no_bottles,
        1 => return single_bottle,
        2 => return two_bottles,
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::from("");
    let mut range_end: i32 = start.clone();
    while range_end >= end {
        if range_end != end {
            song = song + &verse(range_end) + "\n";
        } else {
            song = song + &verse(range_end);
        }
        range_end = range_end - 1;
    }
    song
}
