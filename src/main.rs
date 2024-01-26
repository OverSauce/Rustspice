mod parser;
use parser::*;

fn main() {
    let ctx: SpiceContext = spice! {
        v1 1 0 dc 12.0
        r1 1 0 2.2e3
        end
    };

    println!("{:?}", ctx);
}

