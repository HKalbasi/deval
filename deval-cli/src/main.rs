use std::{path::PathBuf, sync::Arc};

use ariadne::{Color, Config, Fmt, Label, Report, ReportKind, Source};
use deval_format_json::Json;
use deval_format_toml::Toml;
use deval_validator::{AnyValidator, ValidationError, Validator};

use deval_data_model::{Format, ParseError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DevalRule {
    filename: String,
    schema: PathBuf,
}

#[derive(Debug, Default, Deserialize)]
struct DevalConfig {
    rules: Vec<DevalRule>,
}

fn report_validation_errors(source: &str, errors: &[ValidationError]) {
    for error in errors {
        let source = Source::from(source);
        // Create a simple error report pointing to the beginning of the file
        // In a real implementation, you'd want to map errors to specific positions
        let filename = &*error.span.filename;
        let span = error.span.start..error.span.end;
        Report::build(ReportKind::Error, (filename, span.clone()))
            .with_message(&error.text)
            .with_label(Label::new((filename, span.clone())).with_message("error occurred here"))
            .finish()
            .print((filename, source))
            .unwrap();
    }
}

fn report_errors(source: &str, errors: &[ParseError]) {
    for error in errors {
        let filename = &*error.span.filename;
        let span = error.span.start..error.span.end;
        let source = Source::from(source);
        // Create a simple error report pointing to the beginning of the file
        // In a real implementation, you'd want to map errors to specific positions
        Report::build(ReportKind::Error, (filename, span.clone()))
            .with_message(&error.message)
            .with_label(Label::new((filename, span.clone())).with_message("error occurred here"))
            .finish()
            .print((filename, source))
            .unwrap();
    }
}

// Enhanced error reporting with Ariadne
fn display_errors(src: &str, errors: Vec<deval_schema::Error<'_>>) {
    let source_id = "schema";
    let config = Config::default().with_color(true);

    for error in errors {
        let span = error.span();
        let reason = error.reason();
        let found = error
            .found()
            .map(|c| format!("'{}'", c))
            .unwrap_or_else(|| "end of input".to_string());
        let expected = error.expected().map(|s| s.to_string()).collect::<Vec<_>>();

        let mut report = Report::build(ReportKind::Error, (source_id, span.into_range()))
            .with_config(config.clone())
            .with_message(format!("{}: {}", reason, found.fg(Color::Red)))
            .with_label(
                Label::new((source_id, span.into_range()))
                    .with_message(reason)
                    .with_color(Color::Red),
            );

        if !expected.is_empty() {
            let expected_list = expected.join(", ");
            report = report.with_note(format!(
                "Expected one of: {}",
                expected_list.fg(Color::Green)
            ));
        }

        // if let Some(while_parsing) = error.context() {
        //     report = report.with_note(format!("While parsing: {}", while_parsing.fg(Color::Cyan)));
        // }

        report
            .finish()
            .eprint((source_id, Source::from(src)))
            .unwrap();
    }
}

#[derive(clap::Parser)]
enum Args {
    Check {
        #[arg(short, long)]
        schema: String,
        #[arg(short, long)]
        file: String,
    },
    Lsp,
}

fn load_config() -> DevalConfig {
    let Ok(text) = std::fs::read_to_string("/root/.config/deval/config.toml") else {
        return DevalConfig::default();
    };
    let spanned = Toml.parse(&text, "config.toml").unwrap_or_else(|e| {
        report_errors(&text, &e);
        panic!();
    });
    let annotated = AnyValidator.validate(spanned);
    deval_serde::deserialize_from_annotated(&annotated.result)
}

fn main() {
    use clap::Parser;
    let args = Args::parse();

    match args {
        Args::Check { schema, file } => {
            let config = load_config();
            dbg!(&config);
            let schema_source = std::fs::read_to_string(&schema).unwrap();
            let source = std::fs::read_to_string(&file).unwrap();
            let format: Box<dyn Format> = if file.ends_with(".json") {
                Box::new(Json)
            } else if file.ends_with(".toml") {
                Box::new(Toml)    
            } else {
                panic!("Unknown format");
            };
            match format.parse(&source, &file) {
                Ok(data) => {
                    let validator = match deval_schema::compile(&schema_source) {
                        Ok(v) => v,
                        Err(e) => {
                            display_errors(&schema_source, e);
                            return;
                        }
                    };
                    let r = validator.validate(data);
                    report_validation_errors(&source, &r.errors);
                }
                Err(errors) => {
                    report_errors(&source, &errors);
                }
            }
            println!("Input matches the schema!");
        }
        Args::Lsp => {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(async {
                    deval_lsp::start_server(
                        Arc::new(Toml),
                        Arc::new(AnyValidator),
                    ).await;
                });
        }
    }
}
