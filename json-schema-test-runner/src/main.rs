use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;

#[derive(Debug, Parser)]
#[clap(name = "json-schema-test-runner", version = "0.1.0")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run comprehensive analysis on JSON Schema test suite
    Analyze {
        /// Specific test files to analyze (default: all)
        #[clap(short, long, value_delimiter = ',')]
        files: Option<Vec<String>>,
        /// Show detailed output for each test
        #[clap(short, long)]
        verbose: bool,
    },
    /// Debug a specific test case
    Debug {
        /// Test file to debug
        #[clap(short, long)]
        file: String,
        /// Test case index
        #[clap(short, long)]
        case: usize,
        /// Test index within the test case
        #[clap(short, long)]
        test: usize,
    },
}

#[derive(Debug, Deserialize)]
struct TestCase {
    description: String,
    schema: serde_json::Value,
    tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
struct Test {
    description: String,
    data: serde_json::Value,
    valid: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Build the deval-cli binary once
    println!("Building deval-cli...");
    let output = Command::new("cargo")
        .args(&["build", "--bin", "deval-cli"])
        .current_dir("..")
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Failed to build deval-cli: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Ok(());
    }

    let deval_cli_path = "../target/debug/deval-cli";

    match cli.command {
        Commands::Analyze { files, verbose } => {
            run_analysis(deval_cli_path, files, verbose)?;
        }
        Commands::Debug { file, case, test } => {
            run_debug(deval_cli_path, &file, case, test)?;
        }
    }

    Ok(())
}

fn run_analysis(
    deval_cli_path: &str,
    files: Option<Vec<String>>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running comprehensive test suite analysis");

    // Create temp directory
    let temp_dir = "/tmp/json-schema-test-runner";
    fs::create_dir_all(temp_dir)?;

    let test_files = if let Some(files) = files {
        files
    } else {
        vec![
            "type.json".to_string(),
            "properties.json".to_string(),
            "required.json".to_string(),
        ]
    };

    for test_file in test_files {
        println!("\n=== Testing {} ===", test_file);
        test_file_coverage(deval_cli_path, temp_dir, &test_file, verbose)?;
    }

    // Clean up temp directory
    let _ = fs::remove_dir_all(temp_dir);

    Ok(())
}

fn test_file_coverage(
    deval_cli_path: &str,
    temp_dir: &str,
    filename: &str,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let filepath = format!(
        "{}/../JSON-Schema-Test-Suite/tests/draft4/{}",
        current_dir.display(),
        filename
    );
    let content = fs::read_to_string(&filepath)?;
    let test_cases: Vec<TestCase> = serde_json::from_str(&content)?;

    let mut total_tests = 0;
    let mut passed_tests = 0;

    for (i, test_case) in test_cases.iter().enumerate() {
        // Convert the schema to deval format
        let schema_json = serde_json::to_string(&test_case.schema)?;
        let schema_path = format!("{}/temp_schema.json", temp_dir);
        fs::write(&schema_path, &schema_json)?;

        // Convert using our tool
        let output = Command::new(deval_cli_path)
            .args(&["convert-json-schema", &schema_path])
            .output()?;

        if !output.status.success() {
            if verbose {
                println!("  Test case {}: Conversion failed", i);
            }
            total_tests += test_case.tests.len();
            continue;
        }

        let deval_schema = String::from_utf8(output.stdout)?;
        let dvl_path = format!("{}/temp_schema.dvl", temp_dir);
        fs::write(&dvl_path, &deval_schema)?;

        // Run each test in this test case
        for (j, test) in test_case.tests.iter().enumerate() {
            total_tests += 1;

            let result = run_single_test(deval_cli_path, temp_dir, test, &dvl_path)?;

            // Check if result matches expectation
            if result.success == test.valid {
                passed_tests += 1;
                if verbose {
                    println!("  Test case {} test {}: PASS", i, j);
                }
            } else {
                if verbose {
                    println!(
                        "  Test case {} test {}: FAIL (expected {}, got {})",
                        i, j, test.valid, result.success
                    );
                    println!("    Schema: {}", serde_json::to_string(&test_case.schema)?);
                    println!("    Data: {}", serde_json::to_string(&test.data)?);
                }
            }
        }
    }

    let coverage = if total_tests > 0 {
        (passed_tests as f64 / total_tests as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "  Coverage: {:.2}% ({}/{})",
        coverage, passed_tests, total_tests
    );

    Ok(())
}

fn run_debug(
    deval_cli_path: &str,
    filename: &str,
    case_index: usize,
    test_index: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Debugging {} test case {} test {}",
        filename, case_index, test_index
    );

    // Create temp directory
    let temp_dir = "/tmp/json-schema-test-runner-debug";
    fs::create_dir_all(temp_dir)?;

    let current_dir = env::current_dir()?;
    let filepath = format!(
        "{}/../JSON-Schema-Test-Suite/tests/draft4/{}",
        current_dir.display(),
        filename
    );
    let content = fs::read_to_string(&filepath)?;
    let test_cases: Vec<TestCase> = serde_json::from_str(&content)?;

    if case_index >= test_cases.len() {
        eprintln!(
            "Test case index {} out of range (0-{})",
            case_index,
            test_cases.len() - 1
        );
        return Ok(());
    }

    let test_case = &test_cases[case_index];
    println!("Test case: {}", test_case.description);
    println!(
        "Schema: {}",
        serde_json::to_string_pretty(&test_case.schema)?
    );

    // Convert the schema to deval format
    let schema_json = serde_json::to_string(&test_case.schema)?;
    let schema_path = format!("{}/temp_schema.json", temp_dir);
    fs::write(&schema_path, &schema_json)?;

    // Convert using our tool
    let output = Command::new(deval_cli_path)
        .args(&["convert-json-schema", &schema_path])
        .output()?;

    if !output.status.success() {
        println!(
            "Conversion failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let _ = fs::remove_dir_all(temp_dir);
        return Ok(());
    }

    let deval_schema = String::from_utf8(output.stdout)?;
    println!("Converted schema: {}", deval_schema);
    let dvl_path = format!("{}/temp_schema.dvl", temp_dir);
    fs::write(&dvl_path, &deval_schema)?;

    if test_index >= test_case.tests.len() {
        eprintln!(
            "Test index {} out of range (0-{})",
            test_index,
            test_case.tests.len() - 1
        );
        let _ = fs::remove_dir_all(temp_dir);
        return Ok(());
    }

    // Run the specified test
    let test = &test_case.tests[test_index];
    println!("Test: {}", test.description);
    println!("Data: {}", serde_json::to_string_pretty(&test.data)?);
    println!("Expected valid: {}", test.valid);

    let result = run_single_test(deval_cli_path, temp_dir, test, &dvl_path)?;

    println!("Actual valid: {}", result.success);
    if result.success == test.valid {
        println!("Result: PASS");
    } else {
        println!("Result: FAIL");
    }

    if !result.stdout.is_empty() {
        println!("Stdout: {}", result.stdout);
    }
    if !result.stderr.is_empty() {
        println!("Stderr: {}", result.stderr);
    }

    // Clean up temp directory
    let _ = fs::remove_dir_all(temp_dir);

    Ok(())
}

struct TestResult {
    success: bool,
    stdout: String,
    stderr: String,
}

fn run_single_test(
    deval_cli_path: &str,
    temp_dir: &str,
    test: &Test,
    dvl_path: &str,
) -> Result<TestResult, Box<dyn std::error::Error>> {
    // Write test data to temporary file
    let test_data = serde_json::to_string(&test.data)?;
    let data_path = format!("{}/temp_data.json", temp_dir);
    fs::write(&data_path, &test_data)?;

    // Validate using our tool
    let output = Command::new(deval_cli_path)
        .args(&["check", "--schema", dvl_path, "--file", &data_path])
        .output()?;

    Ok(TestResult {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}
