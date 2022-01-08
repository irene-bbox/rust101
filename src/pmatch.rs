fn how_many(x:i8) -> &'static str
{
    match x
    {
    0 => "no stuffies",
    1|2|3|4 => "few stuffies",
    12 => "a dozen",
    z @ 6..=8 => "lots",  // z @ gives a name to the range
    _ if (x % 2 == 0) => "even #",
    _ => "many Yodas = happy Irene",
    }
}


pub fn pattern_matching()
{   // pattern match your count of Baby-Yodas
    for i in 0..15
    {
        println!("{}: I have {} many Baby-Yoda stuffies", i, how_many(i));
    };

    // pattern match a point to its coordinates
    let point = (0,3);
    match point {
        (0,0) => println!("origin"),
        (x,0) => println!("lies on y-axix"),
        (0,y) => println!("lies on x-axis"),
        (_,_) => println!("elsewhere")
    };
}