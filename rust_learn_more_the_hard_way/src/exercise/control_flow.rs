pub fn labels() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    println!("{:#?}", s);

    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        println!("i: {}", i);
        for j in 0..=2 {
            println!("{} {}: {}", i, j, s[i][j]);
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}