use structopt::StructOpt;

/// Print amortization table
#[derive(StructOpt, Debug)]
struct Opt {
    /// Amount
    #[structopt(long = "amount")]
    amount: f64,

    /// Extra
    #[structopt(long = "extra", default_value = "0.0")]
    extra: f64,

    /// rate
    #[structopt(long = "rate")]
    rate: f64,

    /// Years
    #[structopt(long = "years")]
    years: i32,
}

fn main() {
    let opt = Opt::from_args();
    let mut amount = opt.amount;
    let extra = opt.extra;
    let rate = opt.rate / 100.0;
    let years = opt.years;
    let pi = monthly_pi(amount, rate, years);

    for i in 1.. {
        let interest = amount * rate / 12.0;
        let principal = pi - interest;
        amount = amount - principal - extra;
        if amount < 0.00 {
            break;
        }
        println!(
            "{:>3} {:.2} {:.2} {:.2} {:.2}",
            i, amount, principal, interest, pi
        );
    }
}

fn monthly_pi(amount: f64, rate: f64, years: i32) -> f64 {
    let n = years * 12;
    let r = rate / 12.0;
    (r * amount * (1.0 + r).powi(n)) / ((1.0 + r).powi(n) - 1.0)
}
