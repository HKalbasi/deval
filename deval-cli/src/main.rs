use std::{
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
};

use ariadne::{Color, Config, Fmt, Label, Report, ReportKind, Source};
use deval_format_json::Json;
use deval_format_toml::Toml;
use deval_validator::{AnyValidator, ValidationError, Validator};

use deval_data_model::{Format, ParseError};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct DevalRule {
    filename: String,
    schema: PathBuf,
}

#[derive(Debug, Default, Deserialize)]
struct DevalConfig {
    rules: Vec<DevalRule>,
}
impl DevalConfig {
    fn find_schema_path(&self, file: &Path) -> Option<PathBuf> {
        let near = file.with_file_name({
            let mut changed_name = file.file_stem()?.to_owned();
            changed_name.push(".dvl");
            changed_name
        });
        if near.exists() {
            return Some(near);
        }
        Some(
            self.rules
                .iter()
                .find(|rule| {
                    file.file_name()
                        .is_some_and(|x| x.as_bytes() == rule.filename.as_bytes())
                })
                .cloned()?
                .schema,
        )
    }
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
    ConvertJsonSchema {
        file: PathBuf,
    },
    Check {
        #[arg(short, long)]
        schema: Option<PathBuf>,
        #[arg(short, long)]
        file: PathBuf,
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
    deval_serde::deserialize_from_annotated(&annotated.result.discard_annotation())
}

fn main() -> ExitCode {
    use clap::Parser;
    let args = Args::parse();

    match args {
        Args::ConvertJsonSchema { file } => {
            let text = std::fs::read_to_string(&file).unwrap();
            let result = deval_schema_from_json_schema::convert(&text);
            println!("{result}");
            ExitCode::SUCCESS
        }
        Args::Check { schema, file } => {
            let schema = match schema {
                Some(path) => path,
                None => {
                    let config = load_config();
                    dbg!(&config);
                    match config.find_schema_path(&file) {
                        Some(path) => path,
                        None => {
                            eprintln!("Unknown schema for {file:?}");
                            return ExitCode::FAILURE;
                        }
                    }
                }
            };
            let schema_source = std::fs::read_to_string(&schema).unwrap();
            let source = std::fs::read_to_string(&file).unwrap();
            let format: Box<dyn Format> = match file.extension().and_then(|x| x.to_str()) {
                Some("json") => Box::new(Json),
                Some("toml") => Box::new(Toml),
                Some(f) => panic!("Unknown format {f}"),
                None => panic!("Unknown format"),
            };
            match format.parse(
                &source,
                &file
                    .file_name()
                    .map(|x| x.to_string_lossy())
                    .unwrap_or_default(),
            ) {
                Ok(data) => {
                    let validator = match deval_schema::compile(&schema_source) {
                        Ok(v) => v,
                        Err(e) => {
                            display_errors(&schema_source, e);
                            return ExitCode::FAILURE;
                        }
                    };
                    let r = validator.validate(data);
                    report_validation_errors(&source, &r.errors);
                }
                Err(errors) => {
                    report_errors(&source, &errors);
                    return ExitCode::FAILURE;
                }
            }
            println!("Input matches the schema!");
            ExitCode::SUCCESS
        }
        Args::Lsp => {
            let config = load_config();

            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(async move {
                    deval_lsp::start_server(move |path| {
                        let format: Arc<dyn Format> =
                            match path.extension().and_then(|x| x.to_str()) {
                                Some("json") => Arc::new(Json),
                                Some("toml") => Arc::new(Toml),
                                Some(_) => return None,
                                None => return None,
                            };
                        let validator: Arc<dyn Validator> = 'b: {
                            let schema_file = match config.find_schema_path(&path) {
                                Some(path) => path,
                                None => {
                                    break 'b Arc::new(AnyValidator);
                                }
                            };
                            let schema_source = std::fs::read_to_string(&schema_file).unwrap();
                            match deval_schema::compile(&schema_source) {
                                Ok(v) => Arc::<dyn Validator>::from(v),
                                Err(_) => Arc::new(AnyValidator),
                            }
                        };
                        Some((format, validator))
                    })
                    .await;
                });
            ExitCode::SUCCESS
        }
    }
}
