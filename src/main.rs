use std::io;

const SQRT_PI_2: f64 = 0.88622692545;

fn main()
{
    manage_front_end();
}

fn manage_front_end()
{
    println!("");
    println!("Welcome! To get started, type one of the following:");
    println!("");
    println!("log");
    println!("digamma");
    println!("zeta");
    println!("fast gamma");
    println!("____________________________________________________");
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        match input.as_str()
        { 
            "log\r\n" => log_selected(), 
            "digamma\r\n" => digamma_selected(),
            "zeta\r\n" => zeta_selected(),
            "fast gamma\r\n" => fast_gamma_selected(),
            _ => main(), 
        }
}

fn zeta_selected()
{
    println!("");
    println!("zeta function selected");
    println!("What argument will you be calculating?");
    let val = input_float();
    if val <= 0.0
    {
        println!("Error: Not a valid argument (zeta(x) is only defined for x > 0 & x != 1)");
        manage_front_end();
        return;
    }
    println!("");
    println!("Please enter an integer representing how precise you want it calculated to (higher # = more precise)");
    let precision = input_value();
    println!("");
    println!("zeta({}) = {}", val, zeta(val, precision.clone()));
}

fn log_selected()
{
    println!("");
    println!("Natural logarithm selected");
    println!("What argument will you be calculating?");
    let val = input_float();
    if val <= 0.0
    {
        println!("Error: Not a valid argument (ln(x) is only defined for x > 0)");
        manage_front_end();
        return;
    }
    println!("");
    println!("Please enter an integer representing how precise you want it calculated to (higher # = more precise)");
    let precision = input_value();
    println!("");
    println!("ln({}) = {}", val, logarithm(val, precision.clone()));
}

fn digamma_selected()
{
    println!("");
    println!("Digamma function selected");
    println!("What argument will you be calculating?");
    let val = input_float();
    println!("");
    println!("Please enter an integer representing how precise you want it calculated to (higher # = more precise)");
    let precision = input_value();
    println!("");
    println!("digamma({}) = {}", val, digamma(val, precision.clone()));
}

fn fast_gamma_selected()
{
    println!("");
    println!("Fast gamma function selected");
    println!("What argument will you be calculating?");
    let val = input_float();
    println!("");
    println!("gamma({}) = {}", val, fast_gamma(val));
}

fn input_value() -> u64
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<u64>()
    {
        Ok(i) =>  return i,
        Err(..) => println!("this was not a number: {}", trimmed),
    };
    return 1;
}

fn input_float() -> f64
{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse::<f64>()
    {
        Ok(i) =>  return i,
        Err(..) => println!("this was not a number: {}", trimmed),
    };
    return 1.0;
}

fn logarithm(arg: f64, terms: u64) -> f64
{
    let mut total: f64 = 0.0;
    let mut i = 0;
    while i < terms
    {
        total += (2.0 / (2.0 * (i as f64) + 1.0)) * 
            ((arg - 1.0) / (arg + 1.0))
            .powf(2.0*(i as f64) + 1.0);
        i += 1;
    }
    return total;
}

fn digamma(arg: f64, terms: u64) -> f64
{
    let gamma = 0.5772156649;
    let mut total: f64 = 0.0;
    let mut i = 0.0;
    while i < (terms as f64)
    {
        total += (arg - 1.0) / 
            ((i + 1.0) * 
            (i + arg));
        i += 1.0;
    }
    return total - gamma;
}

fn zeta(arg: f64, terms: u64) -> f64
{
    let mut total: f64 = 0.0;
    let mut i = 1;
    if arg < 1.0
    {
        while i < terms
        {
            total += (-1.0_f64).powf((i as f64) + 1.0) / ((i as f64).powf(arg));
            i += 1;
        }
        return total * (1.0 / (1.0 - (2.0_f64).powf(1.0 - arg)));
    }
    else
    {
        while i < terms
        {
            total += 1.0 / ((i as f64).powf(arg));
            i += 1;
        }
        return total;
    }
}

/* Gives a margin of error of about 1% while being extremely fast. */
fn fast_gamma(arg: f64) -> f64
{
    // We want the remainder to be zero if 1 < x < 2
    let mut remainder = 2 - arg.ceil() as i64;
    println!("{}", remainder);
    let base_approx = 0.455 * (arg + remainder as f64 - 1.5) * (arg + remainder as f64 - 1.5) + SQRT_PI_2;
    let mut total = base_approx;
    if remainder < 0
    {
        while remainder != 0
        {
            total *= arg + remainder as f64;
            remainder += 1;
        }
        return total;
    }
    while remainder > 0
    {
        remainder -= 1;
        total /= arg + remainder as f64;
    }
    return total;
}