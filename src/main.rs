use voltgtd;

fn main()
{
    if let Err(error) = voltgtd::parse_cli_arguments()
    {
        println!("Error: {}", error)
    }
}
