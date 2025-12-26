fn main() {
    let to_be_shadowed_var = 1;
    let to_be_shadowed_var = to_be_shadowed_var + 1;

    {
        let to_be_shadowed_var = to_be_shadowed_var + 1;
        println!("Scoped value: {to_be_shadowed_var}");
    }

    println!("Function scope value {to_be_shadowed_var}");
}
