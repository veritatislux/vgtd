use vgtd;

fn main()
{
    if let Err(error) = vgtd::parse_cli_arguments()
    {
        vgtd::tos::send_error(&error.to_string())
    }
}
