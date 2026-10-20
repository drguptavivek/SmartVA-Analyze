# `smartva-rust` (`smartva-rs`) — 100% Pure Native Rust SmartVA-Analyze Engine

`smartva-rust` is a **100% pure native Rust** implementation of the [IHME SmartVA-Analyze (Tariff 2.0)](https://github.com/ihmeuw/SmartVA-Analyze) Verbal Autopsy cause-of-death assignment pipeline (`v3.0.0`).

It provides:
1. **Direct Native Rust Library API (`smartva_rust::SmartVaEngine`)** — score CSV files or in-memory rows directly in Rust with **zero Python dependency**.
2. **Drop-In Native CLI (`smartva-rs`)** — 100% CLI-flag compatible with `smartva` (`va_cli.py`), writing the identical 4-folder output hierarchy (`1-individual-cause-of-death/`, `2-csmf/`, `3-graphs-and-tables/`, `4-monitoring-and-quality/`).
3. **C-ABI Shared Library (`libsmartva_rust.dylib` / `.so` / `.dll`)** — callable from C, C++, R, Swift, Go, or Python (`ctypes` / `cffi`) with in-process cached Tariff 2.0 validation matrices (`smartva_warm_cache`).

---

## Architectural Highlights

- **Zero Python Dependency at Runtime**: Neither `pyo3` nor a Python interpreter is linked or required.
- **Zero JSON Conversion of SmartVA Data**: Source data is managed in Rust in the exact same structure as upstream Python:
  - All 19 Python domain modules in `src/smartva/data/*.py` (`who_data.py`, `common_data.py`, `adult_pre_symptom_data.py`, `child_pre_symptom_data.py`, `neonate_pre_symptom_data.py`, `adult_symptom_data.py`, `child_symptom_data.py`, `neonate_symptom_data.py`, `adult_tariff_data.py`, `child_tariff_data.py`, `neonate_tariff_data.py`, `codebook.py`, `answer_ranges.py`, `word_conversions.py`, `default_fill_data.py`, `icds.py`, `gbd_causes.py`, `countries.py`, `constants.py`) are replaced 1-for-1 with compiled native Rust modules in [`src/data/`](./src/data/).
  - All 9 SmartVA reference CSV files (`tariffs-{adult,child,neonate}.csv`, `validated-{adult,child,neonate}.csv`, `{adult,child,neonate}_undetermined_weights.csv`) sit in [`src/data/`](./src/data/) and are consumed directly in Rust via the [`csv`](https://crates.io/crates/csv) crate.
- **Native `.xlsx` & `.png` Generation**:
  - Generates `4-monitoring-and-quality/{adult,child,neonate}-likelihoods.xlsx` natively via [`rust_xlsxwriter`](https://crates.io/crates/rust_xlsxwriter) ([`src/xlsx.rs`](./src/xlsx.rs)).
  - Generates all 9 cohort figures in `3-graphs-and-tables/*.png` (`all-figure.png`, `undetermined-figure.png`, `adult-female-csmf-figure.png`, `adult-male-csmf-figure.png`, `{adult,child,neonate}-csmf-figure.png`, `all-csmf-figure.png`, `gbd-level1-csmf.png`) natively via [`plotters`](https://crates.io/crates/plotters) ([`src/charts.rs`](./src/charts.rs)).
- **100.00% Empirical Equivalence**: Validated across **15 real-world WHO Verbal Autopsy cohorts (`Cohort-01` .. `Cohort-15`, `8,273` VAs total)** with **100.00% (`8,273 / 8,273`) cause-of-death & ICD-10 agreement** and **0 cell differences across all 23 output and intermediate CSV files per cohort (`345` CSV files total)**. See [`EVIDENCE.md`](./EVIDENCE.md) and [`WHITEPAPER.md`](./WHITEPAPER.md).

---

## Directory Layout

```text
rust/
├── Cargo.toml                  # Pure-Rust crate manifest (no pyo3, no serde_json)
├── README.md                   # Usage guide (Rust API, CLI, C-ABI FFI, Python example)
├── WHITEPAPER.md               # Technical whitepaper & 15-cohort benchmark analysis
├── OPTIMIZATIONS.md            # Detailed technical catalog of engine optimizations
├── EVIDENCE.md                 # 15-cohort equivalence report & side-by-side Python vs Rust charts
├── examples/
│   └── call_smartva_rs.py      # Example Python script invoking smartva-rs CLI & C-ABI FFI
├── evidence/
│   ├── metrics/
│   │   └── upstream_vs_smartva_rs_15cohorts.json
│   └── charts/
│       └── Cohort-01..15/{python,rust}/*.png
└── src/
    ├── lib.rs                  # Public Rust API (SmartVaEngine) & C-ABI FFI exports
    ├── engine.rs               # Full pipeline (WHOPrep -> CommonPrep -> PreSymptomPrep -> RulesPrep -> SymptomPrep -> TariffPrep -> OutputPrep)
    ├── ldap.rs                 # Pure-Rust LdapNotationParser & 20 deterministic clinical rules
    ├── stemmer.rs              # Pure-Rust Porter2 English stemmer (free-text symptom extraction)
    ├── xlsx.rs                 # Native rust_xlsxwriter .xlsx likelihood workbook generator
    ├── charts.rs               # Native plotters .png CSMF & age-distribution chart generator
    ├── bin/
    │   └── smartva_rs.rs       # Drop-in CLI binary matching va_cli.py flags
    └── data/
        ├── mod.rs              # 1-for-1 Rust modules replacing smartva/data/*.py
        ├── who_data.rs
        ├── common_data.rs
        ├── adult_pre_symptom_data.rs
        ├── child_pre_symptom_data.rs
        ├── neonate_pre_symptom_data.rs
        ├── adult_symptom_data.rs
        ├── child_symptom_data.rs
        ├── neonate_symptom_data.rs
        ├── adult_tariff_data.rs
        ├── child_tariff_data.rs
        ├── neonate_tariff_data.rs
        ├── answer_ranges.rs
        ├── codebook.rs
        ├── countries.rs
        ├── default_fill_data.rs
        ├── gbd_causes.rs
        ├── icds.rs
        ├── word_conversions.rs
        ├── constants.rs
        ├── tariffs-{adult,child,neonate}.csv
        ├── validated-{adult,child,neonate}.csv
        └── {adult,child,neonate}_undetermined_weights.csv
```

---

## Building & Testing

```bash
cd rust
cargo test --release
cargo build --release
```

Build outputs in `target/release/`:
- `target/release/smartva-rs` — standalone native CLI binary
- `target/release/libsmartva_rust.rlib` — Rust static library
- `target/release/libsmartva_rust.dylib` (macOS) / `.so` (Linux) / `.dll` (Windows) — C-ABI dynamic library

---

## 1. Direct Rust API Usage (No Python Required)

Add `smartva-rust` to your `Cargo.toml`:

```toml
[dependencies]
smartva-rust = { path = "../smartva-analyze/rust" }
```

Use `SmartVaEngine` directly in Rust:

```rust
use std::path::Path;
use smartva_rust::{SmartVaEngine, SmartVaRunOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = SmartVaEngine::new();

    // Optional: pre-warm the Adult, Child, and Neonate Tariff 2.0 validation matrices (~167 ms once)
    engine.warm_cache()?;

    let options = SmartVaRunOptions {
        country: "IND".to_string(),
        hiv: true,
        malaria: true,
        hce: true,
        freetext: true,
        figures: true,
        write_csv: true,
        extended: false,
        language: "english".to_string(),
        legacy_format: false,
    };

    // Option A: Full 4-folder directory output
    let result = engine.run_cli_directory(
        Path::new("input_va.csv"),
        Path::new("smartva_output"),
        &options,
    )?;

    // Option B: Score JSON payload in memory (converts input JSON -> in-memory CSV -> SmartVA pipeline -> in-memory CSV tables -> JSON / Protobuf)
    let json_out = engine.score_json(
        r#"{"sid":"VA-001","Id10013":"yes","Id10019":"male","ageInYears":62,"Id10147":"yes"}"#,
        &options,
    )?;
    println!("JSON output: {}", json_out);

    println!("Scored {} VAs", result.total_vas);
    for pred in result.predictions.iter().take(5) {
        println!("SID={} -> {} (ICD-10: {})", pred.sid, pred.cause34, pred.icd10);
    }
    Ok(())
}
```

---

## 2. Drop-In CLI Usage (`smartva-rs`)

`smartva-rs` accepts all upstream `smartva` (`va_cli.py`) flags plus `--csv`, `--charts`, `--json`, and `--format`:

```bash
# 1. Full 4-folder CSV + .xlsx + .png directory output (default when OUTPUT_DIR is supplied)
./target/release/smartva-rs \
  --country IND \
  --hiv True \
  --malaria True \
  --hce True \
  --freetext True \
  --csv True \
  --charts True \
  /path/to/input.csv \
  /path/to/output_dir

# 2. Score a single VA or batch from JSON (or CSV) in memory and return standard individual-level JSON
#    (includes individual-cause-of-death.csv, {module}-predictions.csv, {module}-likelihoods.csv,
#     {module}-tariff-scores.csv, {module}-tariff-ranks.csv, {module}-external-ranks.csv; omits CSMF)
./target/release/smartva-rs \
  --country IND \
  --json '{"sid":"VA-001","Id10013":"yes","Id10019":"male","ageInYears":62,"Id10147":"yes"}' \
  --format json

# 3. Return extended cohort-level JSON (adds csmf.csv, {module}-csmf.csv, gbd-level1-csmf.csv,
#    causes-of-death.csv, VA-data-age-groupings.csv, {module}-endorsement-rates.csv, and report.txt)
./target/release/smartva-rs \
  --country IND \
  --format json-extended \
  /path/to/input.csv

# 4. Return Protocol Buffers (proto3 wire format matching proto/smartva.proto)
./target/release/smartva-rs \
  --country IND \
  --format protobuf \
  --output-file result.pb \
  /path/to/input.csv
```

List all 188 supported ISO-3 country codes:

```bash
./target/release/smartva-rs --list-countries
```

---

## 3. Calling from Python (Optional Example Script)

While `smartva-rust` has zero Python dependency, external Python pipelines can invoke either the `smartva-rs` CLI binary or the in-process C-ABI shared library (`libsmartva_rust`). See [`examples/call_smartva_rs.py`](./examples/call_smartva_rs.py):

```bash
python3 examples/call_smartva_rs.py /path/to/input.csv /path/to/output_dir IND
```
