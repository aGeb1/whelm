fn main() {
    let whelm = 0;

    firstline(whelm);
    secondline(whelm);
    thirdline(whelm);
    firstline(whelm);
}

fn firstline(whelm: usize) {
    let spaces = 11 * whelm;
    println!("{:spaces$}{}", "", "XXXXXXXXXXX");
}

fn secondline(whelm: usize) {
    let mut fills = [" "; 5];
    fills[whelm] = "X";
    println!("{0} MINIMUM {0}{1}  UNDER  {1}{2}         {2}{3}  OVER   {3}{4} MAXIMUM {4}",
              fills[0], fills[1], fills[2], fills[3], fills[4]);
}

fn thirdline(whelm: usize) {
    let mut fills = [" "; 5];
    fills[whelm] = "X";
    println!("{0}  WHELM  {0}{1} WHELMED {1}{2} WHELMED {2}{3} WHELMED {3}{4}  WHELM  {4}",
              fills[0], fills[1], fills[2], fills[3], fills[4]);
}