use niho::{converter::Converter, tokenizer::Tokenizer};
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

    let tokenize = noargs::flag("tokenize")
        .short('t')
        .take(&mut args)
        .is_present();
    if let Some(help) = args.finish()? {
        print!("{help}");
        return Ok(());
    }

    let input = std::io::read_to_string(std::io::stdin().lock()).or_fail()?;
    let tokens = Tokenizer::new(&input);
    if tokenize {
        for token in tokens {
            println!("{}", nojson::Json(&token));
        }
        return Ok(());
    }

    let converter = Converter::new();
    let tokens = Tokenizer::new(&input);
    let result = converter.convert_tokens(tokens);
    print!("{}", result);

    Ok(())
}
