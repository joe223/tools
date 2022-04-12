use rome_core::App;
use rome_formatter::{FormatUnstableFeatures, LineWidth};
use rome_fs::RomePath;
use rome_js_formatter::{format, FormatOptions, Formatted, IndentStyle, QuoteStyle};
use rome_js_parser::{parse, ModuleKind, SourceType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

mod check_reformat {
    include!("check_reformat.rs");
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum SerializableIndentStyle {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
}

impl From<SerializableIndentStyle> for IndentStyle {
    fn from(test: SerializableIndentStyle) -> Self {
        match test {
            SerializableIndentStyle::Tab => IndentStyle::Tab,
            SerializableIndentStyle::Space(spaces) => IndentStyle::Space(spaces),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum SerializableQuoteStyle {
    Double,
    Single,
}

impl From<SerializableQuoteStyle> for QuoteStyle {
    fn from(test: SerializableQuoteStyle) -> Self {
        match test {
            SerializableQuoteStyle::Double => QuoteStyle::Double,
            SerializableQuoteStyle::Single => QuoteStyle::Single,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct SerializableFormatOptions {
    /// The indent style.
    pub indent_style: Option<SerializableIndentStyle>,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: Option<u16>,

    /// The style for quotes. Defaults to double.
    pub quote_style: Option<SerializableQuoteStyle>,

    /// Enables the sorting of import statements
    pub unstable_sort_imports: Option<bool>,
}

impl From<SerializableFormatOptions> for FormatOptions {
    fn from(test: SerializableFormatOptions) -> Self {
        Self {
            indent_style: test
                .indent_style
                .map_or_else(|| IndentStyle::Tab, |value| value.into()),
            line_width: test
                .line_width
                .and_then(|width| LineWidth::try_from(width).ok())
                .unwrap_or_default(),
            quote_style: test
                .quote_style
                .map_or_else(|| QuoteStyle::Double, |value| value.into()),
        }
    }
}

impl From<SerializableFormatOptions> for FormatUnstableFeatures {
    fn from(test: SerializableFormatOptions) -> Self {
        Self {
            unstable_sort_imports: test.unstable_sort_imports.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TestOptions {
    cases: Vec<SerializableFormatOptions>,
}

#[derive(Debug, Default)]
struct SnapshotContent {
    input: String,
    output: Vec<(String, FormatOptions, FormatUnstableFeatures)>,
}

impl SnapshotContent {
    pub fn add_output(
        &mut self,
        formatted: Formatted,
        options: FormatOptions,
        unstable_features: FormatUnstableFeatures,
    ) {
        let mut output: String = formatted.as_code().into();
        if !formatted.verbatim().is_empty() {
            output.push_str("\n\n");
            output.push_str("## Unimplemented nodes/tokens");
            output.push_str("\n\n");
            for (text, range) in formatted.verbatim() {
                let string = format!("{:?} => {:?}\n", text, range);
                output.push_str(string.as_str());
            }
        }
        self.output.push((output, options, unstable_features));
    }

    pub fn set_input(&mut self, content: impl Into<String>) {
        self.input = content.into();
    }

    pub fn snap_content(&mut self) -> String {
        let mut output = String::new();
        output.push_str("# Input");
        output.push('\n');
        output.push_str(self.input.as_str());
        output.push_str("\n=============================\n");

        output.push_str("# Outputs\n");
        let iter = self.output.iter();
        for (index, (content, options, unstable_features)) in iter.enumerate() {
            let formal_index = index + 1;
            output.push_str(format!("## Output {formal_index}\n").as_str());
            output.push_str("-----\n");
            output.push_str(format!("{}", options).as_str());
            output.push_str(format!("{}", unstable_features).as_str());
            output.push_str("-----\n");
            output.push_str(content.as_str());
        }

        output
    }
}

/// [insta.rs](https://insta.rs/docs) snapshot testing
///
/// For better development workflow, run
/// `cargo watch -i '*.new' -x 'test -p rome_js_formatter formatter'`
///
/// To review and commit the snapshots, `cargo install cargo-insta`, and run
/// `cargo insta review` or `cargo insta accept`
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.json` and the output file name is `{spec_name}.json.snap`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `json/null` -> input: `tests/specs/json/null.json`, expected output: `tests/specs/json/null.json.snap`
/// * `null` -> input: `tests/specs/null.json`, expected output: `tests/specs/null.json.snap`
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, file_type: &str) {
    let app = App::from_env();

    let file_path = &spec_input_file;
    let spec_input_file = Path::new(spec_input_file);

    assert!(
        spec_input_file.is_file(),
        "The input '{}' must exist and be a file.",
        spec_input_file.display()
    );

    let mut rome_path = RomePath::new(file_path);
    if app.features.can_format(&rome_path) {
        let mut snapshot_content = SnapshotContent::default();
        let buffer = rome_path.get_buffer_from_file();
        let mut source_type: SourceType = rome_path.as_path().try_into().unwrap();
        if file_type != "module" {
            source_type = source_type.with_module_kind(ModuleKind::Script);
        }

        let input = fs::read_to_string(file_path).unwrap();
        snapshot_content.set_input(input.as_str());

        let parsed = parse(buffer.as_str(), 0, source_type.clone());
        let has_errors = parsed.has_errors();
        let root = parsed.syntax();

        let formatted_result = format(
            FormatOptions::default(),
            FormatUnstableFeatures::default(),
            &root,
        );

        let file_name = spec_input_file.file_name().unwrap().to_str().unwrap();
        // we ignore the error for now
        let result = formatted_result.unwrap();

        if !has_errors {
            check_reformat::check_reformat(check_reformat::CheckReformatParams {
                root: &root,
                text: result.as_code(),
                source_type: source_type.clone(),
                file_name,
                format_options: FormatOptions::default(),
                format_unstable_features: FormatUnstableFeatures::default(),
            });
        }

        snapshot_content.add_output(
            result,
            FormatOptions::default(),
            FormatUnstableFeatures::default(),
        );

        let test_directory = PathBuf::from(test_directory);
        let options_path = test_directory.join("options.json");
        if options_path.exists() {
            {
                let mut options_path = RomePath::new(options_path.display().to_string().as_str());
                // SAFETY: we checked its existence already, we assume we have rights to read it
                let options: TestOptions =
                    serde_json::from_str(options_path.get_buffer_from_file().as_str()).unwrap();

                for test_case in options.cases {
                    let format_options: FormatOptions = test_case.into();
                    let format_unstable_features: FormatUnstableFeatures = test_case.into();
                    let formatted_result =
                        format(format_options, format_unstable_features, &root).unwrap();

                    if !has_errors {
                        check_reformat::check_reformat(check_reformat::CheckReformatParams {
                            root: &root,
                            text: formatted_result.as_code(),
                            source_type: source_type.clone(),
                            file_name,
                            format_options,
                            format_unstable_features,
                        });
                    }

                    snapshot_content.add_output(
                        formatted_result,
                        format_options,
                        format_unstable_features,
                    );
                }
            }
        }

        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => spec_input_file.parent().unwrap(),
        }, {
            insta::assert_snapshot!(file_name, snapshot_content.snap_content(), file_name);
        });
    }
}
