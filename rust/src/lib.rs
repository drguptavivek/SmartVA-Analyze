//! 100% Pure Native Rust SmartVA-Analyze Engine (`smartva-rust` / `smartva-rs`).
//!
//! - **Zero runtime Python dependency** (`pyo3` and Python interpreter completely removed).
//! - **Zero JSON data conversion**: Static SmartVA domain modules (`smartva/data/*.py`) live
//!   1-for-1 as native Rust modules in [`data`] (`rust/src/data/*.rs`), and all SmartVA
//!   reference CSVs (`tariffs-{adult,child,neonate}.csv`, `validated-{adult,child,neonate}.csv`,
//!   `{adult,child,neonate}_undetermined_weights.csv`) and user input/output CSVs are read
//!   and written directly in Rust via the [`csv`] crate.
//! - Generates all `.xlsx` workbooks (`4-monitoring-and-quality/*-likelihoods.xlsx`) natively
//!   in Rust via [`rust_xlsxwriter`] ([`xlsx`]).
//! - Generates all `.png` figures (`3-graphs-and-tables/*.png`) natively in Rust via
//!   [`plotters`] ([`charts`]).

pub mod charts;
pub mod data;
pub mod engine;
pub mod ldap;
pub mod stemmer;
pub mod xlsx;

use std::collections::{BTreeMap, HashMap};
use std::ffi::{CStr, CString, c_char, c_int};
use std::fmt;
use std::path::Path;
use std::ptr;

#[derive(Debug, Clone)]
pub enum SmartVaError {
    IoError(String),
    CsvError(String),
    ExecutionError(String),
}

impl fmt::Display for SmartVaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartVaError::IoError(msg) => write!(f, "IO error: {}", msg),
            SmartVaError::CsvError(msg) => write!(f, "CSV error: {}", msg),
            SmartVaError::ExecutionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for SmartVaError {}

#[derive(Debug, Clone)]
pub struct SmartVaRunOptions {
    pub country: String,
    pub hiv: bool,
    pub malaria: bool,
    pub hce: bool,
    pub freetext: bool,
    pub figures: bool,
    pub language: String,
    pub legacy_format: bool,
}

impl Default for SmartVaRunOptions {
    fn default() -> Self {
        Self {
            country: "IND".to_string(),
            hiv: true,
            malaria: true,
            hce: true,
            freetext: true,
            figures: true,
            language: "english".to_string(),
            legacy_format: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmartVaPrediction {
    pub sid: String,
    pub cause34: String,
    pub icd10: String,
    pub age: Option<f64>,
    pub sex: Option<i32>,
    pub module: Option<String>,
    pub cause1: Option<String>,
    pub likelihood1: Option<String>,
    pub key_symptom1: Option<String>,
    pub cause2: Option<String>,
    pub likelihood2: Option<String>,
    pub key_symptom2: Option<String>,
    pub cause3: Option<String>,
    pub likelihood3: Option<String>,
    pub key_symptom3: Option<String>,
    pub all_symptoms: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SmartVaCliDirectoryResult {
    pub total_vas: usize,
    pub predictions: Vec<SmartVaPrediction>,
    pub csmf_all: BTreeMap<String, f64>,
    pub csmf_adult: BTreeMap<String, f64>,
    pub csmf_child: BTreeMap<String, f64>,
    pub csmf_neonate: BTreeMap<String, f64>,
    pub xlsx_files_written: usize,
    pub png_charts_written: usize,
}

/// Pure-Rust SmartVA Engine with in-process cached Tariff 2.0 validation matrices.
#[derive(Debug, Clone, Default)]
pub struct SmartVaEngine;

impl SmartVaEngine {
    /// Create a new pure-Rust `SmartVaEngine` instance.
    pub fn new() -> Self {
        Self
    }

    /// Pre-warm the Adult, Child, and Neonate `TariffPrep` validation models in memory.
    pub fn warm_cache(&self) -> Result<(), SmartVaError> {
        engine::warm_all_models()
    }

    /// Return all 188 valid SmartVA country strings (`"Afghanistan (AFG)"`, etc.).
    pub fn list_countries(&self) -> Vec<&'static str> {
        data::countries::COUNTRIES.to_vec()
    }

    /// Execute the full SmartVA-Analyze directory pipeline on `input_csv` and write all
    /// 4 output folders (`1-individual-cause-of-death/`, `2-csmf/`, `3-graphs-and-tables/`,
    /// `4-monitoring-and-quality/`) into `output_dir`.
    pub fn run_cli_directory(
        &self,
        input_csv: &Path,
        output_dir: &Path,
        options: &SmartVaRunOptions,
    ) -> Result<SmartVaCliDirectoryResult, SmartVaError> {
        engine::run_smartva_pipeline_directory(input_csv, output_dir, options)
    }

    /// Score an input CSV file directly and return the structured [`SmartVaCliDirectoryResult`]
    /// using an ephemeral workspace directory.
    pub fn score_csv(
        &self,
        input_csv: &Path,
        options: &SmartVaRunOptions,
    ) -> Result<SmartVaCliDirectoryResult, SmartVaError> {
        let tmp_dir = tempfile::tempdir().map_err(|e| SmartVaError::IoError(e.to_string()))?;
        let mut opts = options.clone();
        opts.figures = false;
        engine::run_smartva_pipeline_directory(input_csv, tmp_dir.path(), &opts)
    }

    /// Score in-memory CSV rows (`headers` and `rows` of string cells) directly in Rust.
    pub fn score_rows(
        &self,
        headers: &[String],
        rows: &[HashMap<String, String>],
        options: &SmartVaRunOptions,
    ) -> Result<SmartVaCliDirectoryResult, SmartVaError> {
        let tmp_dir = tempfile::tempdir().map_err(|e| SmartVaError::IoError(e.to_string()))?;
        let input_csv = tmp_dir.path().join("input.csv");
        let output_dir = tmp_dir.path().join("output");
        {
            let mut wtr = csv::WriterBuilder::new()
                .from_path(&input_csv)
                .map_err(|e| SmartVaError::CsvError(e.to_string()))?;
            wtr.write_record(headers)
                .map_err(|e| SmartVaError::CsvError(e.to_string()))?;
            for row in rows {
                let rec: Vec<&str> = headers
                    .iter()
                    .map(|h| row.get(h).map(|s| s.as_str()).unwrap_or(""))
                    .collect();
                wtr.write_record(&rec)
                    .map_err(|e| SmartVaError::CsvError(e.to_string()))?;
            }
            wtr.flush()
                .map_err(|e| SmartVaError::IoError(e.to_string()))?;
        }
        let mut opts = options.clone();
        opts.figures = false;
        engine::run_smartva_pipeline_directory(&input_csv, &output_dir, &opts)
    }
}

// ============================================================================
// C-ABI FFI Exports for external callers (C, C++, Swift, Python ctypes, R, etc.)
// ============================================================================

/// Pre-warm the in-process Tariff 2.0 validation matrices via C-ABI.
/// Returns `0` on success, `-1` on error.
#[unsafe(no_mangle)]
pub extern "C" fn smartva_warm_cache() -> c_int {
    match engine::warm_all_models() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Run the full SmartVA-Analyze pipeline on `input_csv_path` and write all output files
/// into `output_dir_path` via C-ABI.
///
/// Returns `0` on success, non-zero on error. If `error_out` is non-null, writes a newly
/// allocated error string on failure (free with [`smartva_free_string`]).
///
/// # Safety
/// `input_csv_path`, `output_dir_path`, and `country` must be valid null-terminated UTF-8 C strings.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartva_run_directory(
    input_csv_path: *const c_char,
    output_dir_path: *const c_char,
    country: *const c_char,
    hiv: c_int,
    malaria: c_int,
    hce: c_int,
    freetext: c_int,
    figures: c_int,
    error_out: *mut *mut c_char,
) -> c_int {
    if !error_out.is_null() {
        unsafe { *error_out = ptr::null_mut() };
    }
    let parse_str = |p: *const c_char| -> Result<String, String> {
        if p.is_null() {
            return Err("Null pointer argument".to_string());
        }
        unsafe { CStr::from_ptr(p) }
            .to_str()
            .map(|s| s.to_string())
            .map_err(|e| e.to_string())
    };
    let res = (|| -> Result<(), String> {
        let in_str = parse_str(input_csv_path)?;
        let out_str = parse_str(output_dir_path)?;
        let country_str = parse_str(country)?;
        let opts = SmartVaRunOptions {
            country: country_str,
            hiv: hiv != 0,
            malaria: malaria != 0,
            hce: hce != 0,
            freetext: freetext != 0,
            figures: figures != 0,
            language: "english".to_string(),
            legacy_format: false,
        };
        engine::run_smartva_pipeline_directory(Path::new(&in_str), Path::new(&out_str), &opts)
            .map(|_| ())
            .map_err(|e| e.to_string())
    })();

    match res {
        Ok(()) => 0,
        Err(msg) => {
            if !error_out.is_null()
                && let Ok(cstr) = CString::new(msg)
            {
                unsafe { *error_out = cstr.into_raw() };
            }
            2
        }
    }
}

/// Free a C string allocated by [`smartva_run_directory`].
///
/// # Safety
/// `ptr` must be null or a pointer returned by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn smartva_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_rust_country_list_and_warm_cache() {
        let engine = SmartVaEngine::new();
        let countries = engine.list_countries();
        assert_eq!(countries.len(), 188);
        assert!(countries.iter().any(|c| c.contains("(IND)")));
        engine.warm_cache().expect("Failed to warm pure-Rust models");
    }
}
