use quick_mathml::parse;

pub fn main() {
    let test = "<math><apply></apply></math>";
    let parsed = parse(test);
    println!("{:?}", parsed);
}
