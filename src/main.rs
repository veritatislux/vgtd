use clap::Parser;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args
{
    /// Enter interactive mode
    #[arg(short, long)]
    interactive: bool,
}


fn main()
{
    let args = Args::parse();

    if args.interactive
    {
        voltgtd::run();
    }
}
