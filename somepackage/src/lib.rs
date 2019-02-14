#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn public_function() {
    println!("called somepackage's `public_function()`");
}

fn private_function() {
    println!("called somepackage's `private_function()`");
}

pub fn indirect_access() {
    print!("called somepackage's `indirect_access()`, that\n> ");

    private_function();
}

pub mod somemod {
    pub fn public_mod_function() {
    println!("called somepackage's `public_mod_function()`");
}
}