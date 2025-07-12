use niho::tokenizer::Tokenizer;
use orfail::OrFail;

fn main() -> noargs::Result<()> {
    let mut args = noargs::raw_args();
    args.metadata_mut().app_name = env!("CARGO_PKG_NAME");
    args.metadata_mut().app_description = env!("CARGO_PKG_DESCRIPTION");

    if noargs::VERSION_FLAG.take(&mut args).is_present() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    noargs::HELP_FLAG.take_help(&mut args);
    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let input = std::io::read_to_string(std::io::stdin().lock()).or_fail()?;
    for token in Tokenizer::new(&input) {
        println!("{}", nojson::Json(&token));
    }

    Ok(())
}
