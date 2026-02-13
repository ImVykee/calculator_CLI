use clap::Parser;

#[derive(Parser)]
struct Operation{
    number_1 : f64,
    operator : String,
    number_2 : Option<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operation: Operation = Operation::parse();
    let result: Result<f64, String>  = calculate(operation.operator, operation.number_1, operation.number_2);
    let content = match result{
        Ok(content) => content ,
        Err(error) => return Err(error.into())
    };
    println!("Résultat : {}", content);
    Ok(())
}

fn calculate(op: String, a: f64, b: Option<f64>) -> Result<f64, String> {
    let op = op.as_str();
    match op{
        "+" | "*" | "x" | "-" | "/" | "^" | "r" | "log" 
        => {
            let b_val = b.ok_or("this operator requires 2 numbers")?;
            classic_operators(op, a, b_val)
        }
        "!" => Ok(factorial(a)),
        "ln" => Ok(a.ln()),
        _ => Err(format!("invalid operator {}", op))
    }
}

fn classic_operators(op: &str, a: f64, b: f64) -> Result<f64, String>{
    match op {
        "+" => Ok(a + b),
        "*" | "x" => Ok(a * b),
        "-" => Ok(a - b),
        "/" => {
            if b == 0.0 {
                Err(String::from("dividing by 0"))
            }else{
                Ok(a / b)
            }
        },
        "^" => Ok(a.powf(b)),
        "r" => Ok(b.powf(1.0/a)),
        "log" => {
            if a <= 1.0 {
                Err(String::from("invalid logarithm base"))
            } else {
                Ok(b.ln() / a.ln())
            }
        },
        _ => unreachable!()
    }
}

fn factorial(n: f64) -> f64 {
    if n <= 1.0 {
        1.0
    } else {
        n * factorial(n - 1.0)
    }
}
