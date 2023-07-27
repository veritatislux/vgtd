use voltgtd;

fn main()
{
    if let Err(error) = voltgtd::parse_cli_arguments()
    {
        voltgtd::tos::send_error(&error.to_string())
    }
}
