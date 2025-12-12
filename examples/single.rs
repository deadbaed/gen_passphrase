fn main() {
    use gen_passphrase::dictionary::EFF_SHORT_2;
    use gen_passphrase::generate;

    let passphrase = generate(&[EFF_SHORT_2], 3, Some("-")).unwrap();
    println!("{passphrase}");
}
