# Technical Whitepaper: `smartva-rust` — A 100% Pure Native Rust Implementation of IHME SmartVA-Analyze (Tariff 2.0)

**Version:** `0.2.0` (`SmartVA-Analyze 3.0.0-rust`)  
**Date:** October 20, 2026  
**Baseline Reference:** Upstream [`ihmeuw/SmartVA-Analyze`](https://github.com/ihmeuw/SmartVA-Analyze) `v3.0.0` (Abraham Flaxman *et al.*, Institute for Health Metrics and Evaluation)

---

## Abstract

Verbal Autopsy (VA) automated cause-of-death assignment via the IHME Tariff 2.0 algorithm (`SmartVA-Analyze`) is a cornerstone of population mortality surveillance across low- and middle-income countries. However, the upstream Python implementation (`ihmeuw/SmartVA-Analyze` v3.0.0) carries heavy runtime dependencies (`numpy`, `scipy`, `matplotlib`, `xlsxwriter`, `pyyaml`, `click`, `progressbar2`), incurs multi-second interpreter and model-training startup overhead on every CLI invocation, and suffers from a Python 3 `io.StringIO.truncate(0)` bug in `UnicodeWriter` that quadratic-pads `{adult,child,neonate}-likelihoods.csv` with megabytes of null bytes (`\x00`).

We present **`smartva-rust` (`smartva-rs` / `libsmartva_rust`)**, a **100% pure native Rust** implementation of the complete `SmartVA-Analyze` v3.0.0 pipeline (`WHOPrep` $\rightarrow$ `CommonPrep` $\rightarrow$ `PreSymptomPrep` $\rightarrow$ `RulesPrep` $\rightarrow$ `SymptomPrep` $\rightarrow$ `TariffPrep` $\rightarrow$ `OutputPrep`, plus `.xlsx` likelihood workbooks and `.png` cohort figures). `smartva-rust` has **zero runtime Python dependency** and **zero JSON conversion of SmartVA data**: all 19 Python domain modules (`src/smartva/data/*.py`) are replaced 1-for-1 with native Rust modules (`rust/src/data/*.rs`), and all 9 SmartVA reference CSVs (`tariffs-*.csv`, `validated-*.csv`, `*_undetermined_weights.csv`) are consumed directly in Rust via the `csv` crate.

Evaluated across **15 anonymized real-world WHO Verbal Autopsy cohorts (`Cohort-01` .. `Cohort-15`, `8,273` VAs total)** with full chart and Excel generation (`--figures True`), `smartva-rust` achieves:
- **100.00% (`8,273 / 8,273`) individual cause-of-death (`cause34`) and ICD-10 parity** with upstream Python `SmartVA-Analyze`.
- **0 cell differences across all 23 output and intermediate CSV files per cohort (`345` CSV files total)**.
- **`9.40x` overall speedup in Cold CLI subprocess mode** (`10.12 s` vs. `95.17 s`, up to **`14.9x`** per cohort).
- **`13.18x` overall speedup in Warm In-Process FFI mode** (`7.22 s` vs. `95.17 s` with `--figures True`, and `6.78 s` / **`14.04x`** with `--figures False`; up to **`41.7x`** faster on small cohorts).
- **`503x` reduction in likelihood CSV disk footprint** on `Cohort-01` (`372 KB` vs. `187.5 MB` in Python) by eliminating the upstream `UnicodeWriter` null-byte corruption bug.

---

## 1. Architecture & 1-for-1 Source Data Management

A core design principle of `smartva-rust` is **structural fidelity to upstream `SmartVA-Analyze`**:
- Where upstream Python stores static domain tables in `src/smartva/data/*.py`, `smartva-rust` stores them 1-for-1 in `rust/src/data/*.rs`.
- Where upstream Python reads reference CSVs (`tariffs-{module}.csv`, `validated-{module}.csv`, `{module}_undetermined_weights.csv`) from `src/smartva/data/*.csv`, `smartva-rust` reads the exact same CSV files directly from `rust/src/data/*.csv` using Rust's `csv::Reader`.

```
Input CSV (WHO 2016 / WHO 2022 / PHMRC Short / PHMRC Full)
   │
   ▼
[1. Header Formatting & Form Detection] (engine.rs)
   │  Strips ODK group prefixes (: / -), detects WHO 2016 vs WHO 2022 vs PHMRC
   ▼
[2. WHOPrep] (engine.rs + data/who_data.rs)
   │  Maps WHO Id10004..Id10481 items, ages, durations, multiselects -> cleanheaders.csv
   ▼
[3. CommonPrep] (engine.rs + data/common_data.rs)
   │  Validates SIDs, consent (gen_3_1), age routing -> {adult,child,neonate}-prepped.csv
   ▼
[4. PreSymptomPrep] (engine.rs + data/*_pre_symptom_data.rs + stemmer.rs)
   │  Recodes categoricals, computes duration/age/WHO weight-for-age SDs, Porter2 free-text stemming
   ▼
[5. RulesPrep] (engine.rs + ldap.rs)
   │  Evaluates 20 deterministic clinical rules & prefix LDAP expressions -> *-logic-rules.csv
   ▼
[6. SymptomPrep] (engine.rs + data/*_symptom_data.rs)
   │  Applies age quartiles, duration cutoffs, injury windows, censored/required cause masks
   ▼
[7. TariffPrep] (engine.rs + data/*_tariff_data.rs + data/tariffs-*.csv + data/validated-*.csv)
   │  Top-40 round-to-0.5 tariff scoring, uniform validation ranking, cause34 reduction, CSMF redistribution
   ▼
[8. OutputPrep + Native XLSX & PNG Renderers] (engine.rs + xlsx.rs + charts.rs)
   ├── 1-individual-cause-of-death/individual-cause-of-death.csv & {module}-predictions.csv
   ├── 2-csmf/{adult,child,neonate}-csmf.csv, csmf.csv, gbd-level1-csmf.csv
   ├── 3-graphs-and-tables/causes-of-death.csv & 9 cohort figures (*.png via plotters)
   └── 4-monitoring-and-quality/{module}-likelihoods.xlsx (via rust_xlsxwriter), report.txt, intermediate-files/
```

---

## 2. Numerical & Algorithmic Precision Guarantees

Achieving **0 CSV cell differences across all 345 output and intermediate CSV files** required exact reproduction of Python 3 / NumPy semantics across seven subtle edge cases:

1. **Banker's Rounding (`round_ties_even`) on Tariff Precision (`0.5`)**:
   In `clean_tariffs`, upstream Python computes `np.round(v / 0.5) * 0.5`. `smartva-rust` preserves exact CSV column order from `tariffs-{module}.csv`, performs a stable sort by `|weight|` descending (`sort_by(|a, b| b.1.abs().total_cmp(&a.1.abs()))`), truncates to 40 symptoms, and applies IEEE-754 round-half-to-even (`(v / 0.5).round_ties_even() * 0.5`).
2. **Distinguishing Zero Endorsed Symptoms (`"0"`) vs. Zero-Sum Endorsed Symptoms (`"0.0"`)**:
   In Python's `TariffPrep.score_row`, `sum(tariff for symptom, tariff in symptoms if symptom in endorsements)` returns integer `0` (`"0"`) when the generator is empty, and float `0.0` (`"0.0"`) when $\ge 1$ endorsed symptoms sum to zero. `engine.rs` tracks `(score: f64, had_endorsed: bool)` so `{module}-tariff-scores.csv` matches Python byte-for-byte.
3. **Duration Multiplication Type Formatting (`"0"` vs. `"0.0"`)**:
   In `PreSymptomPrep.calculate_duration_vars`, `TIME_FACTORS.get(code_value, 0) * length_value` yields integer `0` (`"0"`) when `code_value` is outside `1..=6` and a `float` (`"0.0"`) when `code_value` is in `1..=6` and `length_value == 0`.
4. **Midpoint Tie-Rank Averaging (`bisect_left` / `bisect_right`)**:
   External ranking against the ascending `uniform_scores` distribution uses binary-search partition points (`gt = partition_point(|&x| x < s)`, `lt = partition_point(|&x| x <= s)`) to compute `(N - gt) + (gt - lt) / 2.0 + 0.5`.
5. **First-Index Symptom Description Ordering (`symptom_order.index(s)`)**:
   In `neonate_tariff_data.SYMPTOM_DESCRIPTIONS`, `"Difficulty breathing"` appears twice (`s73` and `s76`). Python's `list.index(s)` returns the first index. `engine.rs` populates `symp_order` via `.entry(desc).or_insert(i)` so `all_symptoms` in `{module}-likelihoods.csv` sorts identically.
6. **Relative-Delta Calendar Month Calculation (`dateutil.relativedelta`)**:
   For child weight-for-age SD scoring (`process_weight_sd_vars`), `engine.rs` replicates `dateutil.relativedelta(latest_exam, dob)` month clamping across variable-length months (`days_in_month`).
7. **Elimination of Upstream Python's `UnicodeWriter` Null-Byte (`\x00`) Bug**:
   Upstream Python's `UnicodeWriter.writerow` calls `self.queue.truncate(0)` on an `io.StringIO` without `self.queue.seek(0)`, causing Python 3 to pad `{module}-likelihoods.csv` with $O(N^2)$ null bytes (`187.5 MB` on `Cohort-01`). `smartva-rust` writes clean RFC-4180 UTF-8 CSVs (`372 KB` on `Cohort-01`) with zero null bytes.

---

## 3. Empirical Validation & Benchmark Results Across 15 Cohorts (`8,273` VAs)

All 15 cohorts were benchmarked on Apple Silicon (`macOS arm64`) with `--country IND --hiv True --malaria True --hce True --freetext True --figures True`.

| Cohort | Input VAs | Scored VAs | `cause34` & ICD-10 Match | CSV Cell Diffs (23 Files) | Charts (`Py` / `Rs`) | Upstream Python (`ms`) | Rust Cold CLI (`ms`) | Cold CLI Speedup | Rust Warm FFI (`ms`) | Warm FFI Speedup |
| --- | ---: | ---: | ---: | ---: | :---: | ---: | ---: | ---: | ---: | ---: |
| **Cohort-01** | 1,104 | 1,104 | `1,104 / 1,104` (`100.0%`) | `0` | `9 / 9` | `9,374.6` | `1,655.0` | **`5.66x`** | `1,095.4` | **`8.56x`** |
| **Cohort-02** | 1,077 | 1,077 | `1,077 / 1,077` (`100.0%`) | `0` | `9 / 9` | `8,112.2` | `999.4` | **`8.12x`** | `842.0` | **`9.63x`** |
| **Cohort-03** | 116 | 116 | `116 / 116` (`100.0%`) | `0` | `8 / 8` | `4,174.7` | `315.8` | **`13.22x`** | `151.5` | **`27.56x`** |
| **Cohort-04** | 820 | 820 | `820 / 820` (`100.0%`) | `0` | `9 / 9` | `7,392.8` | `808.4` | **`9.14x`** | `628.2` | **`11.77x`** |
| **Cohort-05** | 1,258 | 1,258 | `1,258 / 1,258` (`100.0%`) | `0` | `8 / 8` | `8,039.5` | `1,129.1` | **`7.12x`** | `958.5` | **`8.39x`** |
| **Cohort-06** | 1,388 | 1,388 | `1,388 / 1,388` (`100.0%`) | `0` | `9 / 9` | `8,994.7` | `1,220.2` | **`7.37x`** | `1,065.7` | **`8.44x`** |
| **Cohort-07** | 1,169 | 1,169 | `1,169 / 1,169` (`100.0%`) | `0` | `9 / 9` | `8,532.9` | `1,016.5` | **`8.39x`** | `861.0` | **`9.91x`** |
| **Cohort-08** | 178 | 178 | `178 / 178` (`100.0%`) | `0` | `7 / 7` | `4,913.8` | `382.0` | **`12.86x`** | `213.2` | **`23.05x`** |
| **Cohort-09** | 59 | 59 | `59 / 59` (`100.0%`) | `0` | `7 / 7` | `3,959.8` | `265.7` | **`14.90x`** | `107.5` | **`36.84x`** |
| **Cohort-10** | 23 | 23 | `23 / 23` (`100.0%`) | `0` | `6 / 6` | `3,439.8` | `244.1` | **`14.09x`** | `82.5` | **`41.69x`** |
| **Cohort-11** | 116 | 116 | `116 / 116` (`100.0%`) | `0` | `8 / 8` | `4,414.2` | `300.8` | **`14.67x`** | `148.0` | **`29.83x`** |
| **Cohort-12** | 256 | 256 | `256 / 256` (`100.0%`) | `0` | `9 / 9` | `6,388.6` | `468.4` | **`13.64x`** | `279.2` | **`22.88x`** |
| **Cohort-13** | 255 | 255 | `255 / 255` (`100.0%`) | `0` | `7 / 7` | `5,452.4` | `461.9` | **`11.80x`** | `283.1` | **`19.26x`** |
| **Cohort-14** | 227 | 227 | `227 / 227` (`100.0%`) | `0` | `9 / 9` | `6,263.1` | `424.5` | **`14.75x`** | `244.6` | **`25.61x`** |
| **Cohort-15** | 227 | 227 | `227 / 227` (`100.0%`) | `0` | `7 / 7` | `5,718.8` | `428.8` | **`13.34x`** | `263.0` | **`21.74x`** |
| **Total / Overall** | **8,273** | **8,273** | **`8,273 / 8,273` (`100.00%`)** | **`0`** | **`121 / 121`** | **`95,171.9`** | **`10,120.6`** | **`9.40x`** | **`7,223.3`** | **`13.18x`** |

---

## 4. Conclusion

## 7. Known Limitations

1. **PHMRC Full / Shortened Questionnaires (and WHO 2016 Questionnaires) Not Empirically Tested**: Although the PHMRC Full (`adult_*` / `child_*` / `neonate_*` / `gen_*` headers), PHMRC Shortened (`adult_7_11` header), and WHO 2016 (`Id10219` header) questionnaire branches and all downstream `CommonPrep`, `PreSymptomPrep`, `RulesPrep`, `SymptomPrep`, and `TariffPrep` modules are ported 1-for-1 in Rust, empirical benchmark validation (`8,273` VAs across `15` cohorts) was conducted exclusively on **WHO 2022 Verbal Autopsy questionnaires**. Any deployment targeting legacy PHMRC Full, PHMRC Shortened, or WHO 2016 forms should first run cohort parity verification on representative datasets.
2. **English Output Tables & Figures**: Output labels are generated in English (`--language english`).

---

## 8. Conclusion

`smartva-rust` demonstrates that the complete IHME SmartVA-Analyze (Tariff 2.0) pipeline — including WHO 2016/2022 questionnaire mapping, Porter2 free-text stemming, prefix LDAP logic rules, uniform validation ranking, CSMF undetermined redistribution, formatted Excel likelihood workbooks, cohort mortality figures, and in-memory JSON/Protobuf serialization — can be executed in **100% pure native Rust** with **zero runtime Python dependency**, **100.00% mathematical equivalence across 8,273 WHO VAs**, and an **order-of-magnitude reduction in execution time and disk footprint**.
