use std::io;

fn main()
{
    manage_front_end();
}

fn manage_front_end()
{
    println!("");
    println!("Welcome! Do you want to calculate the digamma function or the natural logarithm of a number?");
    println!("For log, type: log");
    println!("For digamma, type: digamma");
    let mut function = String::new();
    io::stdin()
        .read_line(&mut function)
        .expect("Failed to read line");
        if function == ("log\r\n")
        {
            println!("");
            println!("Natural logarithm selected it is");
            println!("What argument will you be calculating?");
            let val = input_float();
            if (val <= 0.0)
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
        else if function == ("digamma\r\n")
        {
            println!("");
            println!("Digamma function selected it is");
            println!("What argument will you be calculating?");
            let val = input_float();
            println!("");
            println!("Please enter an integer representing how precise you want it calculated to (higher # = more precise)");
            let precision = input_value();
            println!("");
            println!("digamma({}) = {}", val, digamma(val, precision.clone()));
        }
        else
        {
            println!("Failed to read input. Please try again");
            manage_front_end();
        }
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