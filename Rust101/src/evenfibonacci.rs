#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

// Sum of the even fibonacci numbers below 4 millions
pub fn evenfib()
{
    let max = 4*i64::pow(10, 6);
    let mut fib = vec![1,2];
    let mut sum = fib[1];
    let mut indx = fib.len()-1;

    while fib[indx] < max
    {
        let i = fib.len()-1;
        let j = fib.len()-2;
        indx = i;
        let c = fib[i]+fib[j];
        fib.push(c); //check whether the push was successful
        let tail_fib = &fib[j..=i];
        //println!("{:?}", tail_fib);
        if c >= max { break; }
        if c%2 == 0 { sum += c} else { sum += 0}
    }
    println!("The sum of the even Fibonacci numbers smaller than 4 millions is = {}", sum);
}





