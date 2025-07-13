use std::{borrow::Cow, path::PathBuf};

use niho::{converter::Converter, dictionary::Dictionary, tokenizer::Tokenizer};
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
    let dictionary_file_path: Option<PathBuf> = noargs::opt("dictionary-file")
        .short('d')
        .ty("PATH")
        .env("NIHO_DICTIONARY_FILE")
        .take(&mut args)
        .present_and_then(|a| a.value().parse())?;
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

    let dic_text = if let Some(path) = dictionary_file_path {
        Cow::Owned(
            std::fs::read_to_string(&path)
                .or_fail_with(|e| format!("failed to read {}: {e}", path.display()))?,
        )
    } else {
        Cow::Borrowed(Dictionary::DEFAULT)
    };
    let dic = Dictionary::new(&dic_text);
    let converter = Converter::new(dic).or_fail()?;
    for token in tokens {
        converter.convert(std::io::stdout(), token).or_fail()?;
    }
    println!();

    Ok(())
}
