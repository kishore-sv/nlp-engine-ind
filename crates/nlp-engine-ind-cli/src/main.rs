use clap::{Parser, Subcommand, ValueEnum};
use nlp_engine_ind_core::{
    ConversionRequest, Converter, EngineConfig, Language, Result, RomanizationStyle, Style,
};
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, ValueEnum)]
enum LangArg {
    Kn,
    Hi,
    Te,
    Ta,
    Ml,
}

impl LangArg {
    fn into_language(self) -> Language {
        match self {
            Self::Kn => Language::Kannada,
            Self::Hi => Language::Hindi,
            Self::Te => Language::Telugu,
            Self::Ta => Language::Tamil,
            Self::Ml => Language::Malayalam,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "nlp-engine-ind",
    about = "Offline Indian conversational NLP engine CLI",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Target language code (kn, hi, te, ta, ml)
    #[arg(short, long, default_value = "kn")]
    language: LangArg,

    /// Output JSON
    #[arg(long)]
    json: bool,

    /// Input text (when not using interactive mode)
    text: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Interactive REPL mode
    Interactive {
        #[arg(short, long, default_value = "kn")]
        language: LangArg,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let converter = Converter::new();

    match cli.command {
        Some(Commands::Interactive { language }) => {
            run_interactive(&converter, language.into_language())?
        }
        None => {
            if cli.text.is_empty() {
                eprintln!("Usage: nlp-engine-ind --language kn \"I'll call you later\"");
                eprintln!("       nlp-engine-ind interactive");
                std::process::exit(1);
            }
            let text = cli.text.join(" ");
            run_once(&converter, cli.language.into_language(), &text, cli.json)?
        }
    }
    Ok(())
}

fn run_once(converter: &Converter, language: Language, text: &str, json: bool) -> Result<()> {
    let result = converter.convert(ConversionRequest::new(text, language))?;
    if json {
        let payload = serde_json::json!({
            "language": result.language.display_name(),
            "language_code": result.language.code(),
            "input": result.input,
            "output": result.output,
            "confidence": result.confidence,
            "latency_us": result.latency_us,
            "intent": format!("{:?}", result.intent),
            "diagnostics": {
                "template_matched": result.diagnostics.template_matched,
                "conversion_ratio": result.diagnostics.conversion_ratio,
                "unknown_token_count": result.diagnostics.unknown_token_count,
                "protected_token_count": result.diagnostics.protected_token_count,
            }
        });
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    } else {
        println!("Language: {}", result.language);
        println!("Input: {}", result.input);
        println!("Output: {}", result.output);
        println!("Confidence: {:.2}", result.confidence);
        println!("Latency: {} µs", result.latency_us);
    }
    Ok(())
}

fn run_interactive(converter: &Converter, mut language: Language) -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    writeln!(stdout, "nlp-engine-ind interactive mode").ok();
    writeln!(stdout, "Language: {language}").ok();
    writeln!(stdout, "Commands: :lang kn|hi|te|ta|ml, :quit").ok();

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| nlp_engine_ind_core::EngineError::InvalidInput {
            reason: e.to_string(),
        })?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == ":quit" || trimmed == ":q" {
            break;
        }
        if let Some(code) = trimmed.strip_prefix(":lang ") {
            if let Some(lang) = Language::from_code(code.trim()) {
                language = lang;
                writeln!(stdout, "> Language: {language}").ok();
            } else {
                writeln!(stdout, "Unknown language: {code}").ok();
            }
            continue;
        }

        let config = EngineConfig {
            language,
            style: Style::Casual,
            code_mixing: nlp_engine_ind_core::CodeMixMode::Natural,
            romanization: RomanizationStyle::CasualChat,
            ..Default::default()
        };
        let result = converter.with_config(config).convert(trimmed)?;
        writeln!(stdout, "\nOutput:\n{}\n", result.output).ok();
        writeln!(stdout, "Confidence: {:.2}", result.confidence).ok();
    }
    Ok(())
}
