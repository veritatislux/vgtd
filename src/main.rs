fn main()
{
    if let Err(message) = voltgtd::run()
    {
        eprintln!("(VoltGTD) Error: {message}.");
        std::process::exit(1);
    }
}
