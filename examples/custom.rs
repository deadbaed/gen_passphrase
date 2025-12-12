fn main() {
    use gen_passphrase::dictionary::EFF_SHORT_1;
    use gen_passphrase::generate;

    let custom_dictionary: &[&str] = &["this", "is", "my", "custom", "dictionary"];

    let passphrase = generate(&[custom_dictionary, EFF_SHORT_1], 1, Some(" ")).unwrap();
    println!("{passphrase}");
}
