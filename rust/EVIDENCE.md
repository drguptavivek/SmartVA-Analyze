# Empirical Equivalence & Visual Chart Evidence: Upstream Python `SmartVA-Analyze` vs. Pure-Rust `smartva-rust` (`Cohort-01` .. `Cohort-15`, `8,273` VAs)

> [!IMPORTANT]
> **100.00% Mathematical & CSV Equivalence Verified Across All 15 Real-World Cohorts (`8,273 / 8,273` VAs)**
> - **Engine**: `smartva-rust v0.2.0` (`100% Pure Native Rust`, zero runtime Python dependency, direct CSV consumption from `src/data/*.csv` and `src/data/*.rs`).
> - **Individual Cause-of-Death (`cause34`) & ICD-10 Agreement**: **`8,273 / 8,273` (`100.00%`)**.
> - **CSV Cell Differences Across All 23 Output & Intermediate CSV Files $\times$ 15 Cohorts (`345` CSV files)**: **`0` (`0` differences)**.
> - **Cohort Figures Generated (`3-graphs-and-tables/*.png`)**: **`121 / 121` (`100.00%`)**.
> - **Runtime Speedup (`--figures True`)**: **`9.40x`** faster in Cold CLI mode (`10.12 s` vs. `95.17 s`) and **`13.18x`** faster in Warm In-Process FFI mode (`7.22 s` vs. `95.17 s`, up to **`41.69x`** on small cohorts).

## 1. 15-Cohort Equivalence & Performance Summary Table

| Cohort | Input VAs | Scored VAs | `cause34` & ICD-10 Parity | CSV Cell Diffs (23 Files) | Charts (`Py` / `Rs`) | Python (`ms`) | Rust Cold CLI (`ms`) | Cold Speedup | Rust Warm FFI (`ms`) | Warm Speedup |
| --- | ---: | ---: | :---: | :---: | :---: | ---: | ---: | ---: | ---: | ---: |
| [Cohort-01](#cohort-01) | 1,104 | 1,104 | `1,104 / 1,104` (`100.0%`) | `0` | `9 / 9` | `9374.6` | `1655.0` | **`5.66x`** | `1095.4` | **`8.56x`** |
| [Cohort-02](#cohort-02) | 1,077 | 1,077 | `1,077 / 1,077` (`100.0%`) | `0` | `9 / 9` | `8112.2` | `999.5` | **`8.12x`** | `842.0` | **`9.63x`** |
| [Cohort-03](#cohort-03) | 116 | 116 | `116 / 116` (`100.0%`) | `0` | `7 / 7` | `4174.7` | `315.8` | **`13.22x`** | `151.5` | **`27.55x`** |
| [Cohort-04](#cohort-04) | 820 | 820 | `820 / 820` (`100.0%`) | `0` | `9 / 9` | `7392.8` | `808.4` | **`9.15x`** | `628.2` | **`11.77x`** |
| [Cohort-05](#cohort-05) | 1,258 | 1,258 | `1,258 / 1,258` (`100.0%`) | `0` | `8 / 8` | `8039.5` | `1129.1` | **`7.12x`** | `958.5` | **`8.39x`** |
| [Cohort-06](#cohort-06) | 1,388 | 1,388 | `1,388 / 1,388` (`100.0%`) | `0` | `8 / 8` | `8994.7` | `1220.2` | **`7.37x`** | `1065.7` | **`8.44x`** |
| [Cohort-07](#cohort-07) | 1,169 | 1,169 | `1,169 / 1,169` (`100.0%`) | `0` | `9 / 9` | `8532.9` | `1016.5` | **`8.39x`** | `861.0` | **`9.91x`** |
| [Cohort-08](#cohort-08) | 178 | 178 | `178 / 178` (`100.0%`) | `0` | `8 / 8` | `4913.8` | `382.0` | **`12.86x`** | `213.2` | **`23.04x`** |
| [Cohort-09](#cohort-09) | 59 | 59 | `59 / 59` (`100.0%`) | `0` | `7 / 7` | `3959.8` | `265.6` | **`14.91x`** | `107.5` | **`36.83x`** |
| [Cohort-10](#cohort-10) | 23 | 23 | `23 / 23` (`100.0%`) | `0` | `7 / 7` | `3439.8` | `244.1` | **`14.09x`** | `82.5` | **`41.71x`** |
| [Cohort-11](#cohort-11) | 116 | 116 | `116 / 116` (`100.0%`) | `0` | `7 / 7` | `4414.2` | `300.8` | **`14.67x`** | `148.0` | **`29.83x`** |
| [Cohort-12](#cohort-12) | 256 | 256 | `256 / 256` (`100.0%`) | `0` | `9 / 9` | `6388.6` | `468.4` | **`13.64x`** | `279.2` | **`22.89x`** |
| [Cohort-13](#cohort-13) | 255 | 255 | `255 / 255` (`100.0%`) | `0` | `7 / 7` | `5452.4` | `461.9` | **`11.80x`** | `283.1` | **`19.26x`** |
| [Cohort-14](#cohort-14) | 227 | 227 | `227 / 227` (`100.0%`) | `0` | `9 / 9` | `6263.1` | `424.5` | **`14.75x`** | `244.6` | **`25.61x`** |
| [Cohort-15](#cohort-15) | 227 | 227 | `227 / 227` (`100.0%`) | `0` | `8 / 8` | `5718.8` | `428.8` | **`13.34x`** | `263.0` | **`21.74x`** |
| **Total / Overall** | **8,273** | **8,273** | **`8,273 / 8,273` (`100.00%`)** | **`0`** | **`121 / 121`** | **`95171.9`** | **`10120.6`** | **`9.40x`** | **`7223.3`** | **`13.18x`** |

## 2. Verified Output & Intermediate Files (23 CSVs + 3 XLSX + 9 PNGs per Cohort)

Every cohort run was verified file-by-file and cell-by-cell across all 4 SmartVA output folders:

1. `1-individual-cause-of-death/`: `individual-cause-of-death.csv`, `adult-predictions.csv`, `child-predictions.csv`, `neonate-predictions.csv` (`0` diffs)
2. `2-csmf/`: `csmf.csv`, `gbd-level1-csmf.csv`, `adult-csmf.csv`, `child-csmf.csv`, `neonate-csmf.csv` (`0` diffs)
3. `3-graphs-and-tables/`: `causes-of-death.csv` (`0` diffs) + up to 9 cohort `.png` figures (`121` chart pairs across 15 cohorts)
4. `4-monitoring-and-quality/`: `VA-data-age-groupings.csv` (`0` diffs), `{adult,child,neonate}-likelihoods.xlsx`, `report.txt`, and `intermediate-files/{adult,child,neonate}-{raw-data,symptom,likelihoods,endorsement-rates}.csv` (`0` diffs)

---

## 3. Side-by-Side Cohort Chart Comparisons: Upstream Python (`matplotlib`) vs. Pure-Rust (`plotters`)

### Cohort-01

- **VAs Scored**: `1,104` | **`cause34` Parity**: `1,104 / 1,104` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `9374.6 ms` | **Rust Cold CLI**: `1655.0 ms` (**`5.66x`**) | **Rust Warm FFI**: `1095.4 ms` (**`8.56x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/adult-csmf-figure.png) | ![Cohort-01 Python adult-csmf-figure.png](./evidence/charts/Cohort-01/python/adult-csmf-figure.png) | ![Cohort-01 Rust adult-csmf-figure.png](./evidence/charts/Cohort-01/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/adult-female-csmf-figure.png) | ![Cohort-01 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-01/python/adult-female-csmf-figure.png) | ![Cohort-01 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-01/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/adult-male-csmf-figure.png) | ![Cohort-01 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-01/python/adult-male-csmf-figure.png) | ![Cohort-01 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-01/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/all-csmf-figure.png) | ![Cohort-01 Python all-csmf-figure.png](./evidence/charts/Cohort-01/python/all-csmf-figure.png) | ![Cohort-01 Rust all-csmf-figure.png](./evidence/charts/Cohort-01/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/all-figure.png) | ![Cohort-01 Python all-figure.png](./evidence/charts/Cohort-01/python/all-figure.png) | ![Cohort-01 Rust all-figure.png](./evidence/charts/Cohort-01/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/child-csmf-figure.png) | ![Cohort-01 Python child-csmf-figure.png](./evidence/charts/Cohort-01/python/child-csmf-figure.png) | ![Cohort-01 Rust child-csmf-figure.png](./evidence/charts/Cohort-01/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/gbd-level1-csmf.png) | ![Cohort-01 Python gbd-level1-csmf.png](./evidence/charts/Cohort-01/python/gbd-level1-csmf.png) | ![Cohort-01 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-01/rust/gbd-level1-csmf.png) |
| **Neonate CSMF Bar Chart (`neonate-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/neonate-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/neonate-csmf-figure.png) | ![Cohort-01 Python neonate-csmf-figure.png](./evidence/charts/Cohort-01/python/neonate-csmf-figure.png) | ![Cohort-01 Rust neonate-csmf-figure.png](./evidence/charts/Cohort-01/rust/neonate-csmf-figure.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-01/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-01/rust/undetermined-figure.png) | ![Cohort-01 Python undetermined-figure.png](./evidence/charts/Cohort-01/python/undetermined-figure.png) | ![Cohort-01 Rust undetermined-figure.png](./evidence/charts/Cohort-01/rust/undetermined-figure.png) |

---

### Cohort-02

- **VAs Scored**: `1,077` | **`cause34` Parity**: `1,077 / 1,077` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `8112.2 ms` | **Rust Cold CLI**: `999.5 ms` (**`8.12x`**) | **Rust Warm FFI**: `842.0 ms` (**`9.63x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/adult-csmf-figure.png) | ![Cohort-02 Python adult-csmf-figure.png](./evidence/charts/Cohort-02/python/adult-csmf-figure.png) | ![Cohort-02 Rust adult-csmf-figure.png](./evidence/charts/Cohort-02/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/adult-female-csmf-figure.png) | ![Cohort-02 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-02/python/adult-female-csmf-figure.png) | ![Cohort-02 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-02/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/adult-male-csmf-figure.png) | ![Cohort-02 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-02/python/adult-male-csmf-figure.png) | ![Cohort-02 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-02/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/all-csmf-figure.png) | ![Cohort-02 Python all-csmf-figure.png](./evidence/charts/Cohort-02/python/all-csmf-figure.png) | ![Cohort-02 Rust all-csmf-figure.png](./evidence/charts/Cohort-02/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/all-figure.png) | ![Cohort-02 Python all-figure.png](./evidence/charts/Cohort-02/python/all-figure.png) | ![Cohort-02 Rust all-figure.png](./evidence/charts/Cohort-02/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/child-csmf-figure.png) | ![Cohort-02 Python child-csmf-figure.png](./evidence/charts/Cohort-02/python/child-csmf-figure.png) | ![Cohort-02 Rust child-csmf-figure.png](./evidence/charts/Cohort-02/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/gbd-level1-csmf.png) | ![Cohort-02 Python gbd-level1-csmf.png](./evidence/charts/Cohort-02/python/gbd-level1-csmf.png) | ![Cohort-02 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-02/rust/gbd-level1-csmf.png) |
| **Neonate CSMF Bar Chart (`neonate-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/neonate-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/neonate-csmf-figure.png) | ![Cohort-02 Python neonate-csmf-figure.png](./evidence/charts/Cohort-02/python/neonate-csmf-figure.png) | ![Cohort-02 Rust neonate-csmf-figure.png](./evidence/charts/Cohort-02/rust/neonate-csmf-figure.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-02/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-02/rust/undetermined-figure.png) | ![Cohort-02 Python undetermined-figure.png](./evidence/charts/Cohort-02/python/undetermined-figure.png) | ![Cohort-02 Rust undetermined-figure.png](./evidence/charts/Cohort-02/rust/undetermined-figure.png) |

---

### Cohort-03

- **VAs Scored**: `116` | **`cause34` Parity**: `116 / 116` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `4174.7 ms` | **Rust Cold CLI**: `315.8 ms` (**`13.22x`**) | **Rust Warm FFI**: `151.5 ms` (**`27.55x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/adult-csmf-figure.png) | ![Cohort-03 Python adult-csmf-figure.png](./evidence/charts/Cohort-03/python/adult-csmf-figure.png) | ![Cohort-03 Rust adult-csmf-figure.png](./evidence/charts/Cohort-03/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/adult-female-csmf-figure.png) | ![Cohort-03 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-03/python/adult-female-csmf-figure.png) | ![Cohort-03 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-03/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/adult-male-csmf-figure.png) | ![Cohort-03 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-03/python/adult-male-csmf-figure.png) | ![Cohort-03 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-03/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/all-csmf-figure.png) | ![Cohort-03 Python all-csmf-figure.png](./evidence/charts/Cohort-03/python/all-csmf-figure.png) | ![Cohort-03 Rust all-csmf-figure.png](./evidence/charts/Cohort-03/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/all-figure.png) | ![Cohort-03 Python all-figure.png](./evidence/charts/Cohort-03/python/all-figure.png) | ![Cohort-03 Rust all-figure.png](./evidence/charts/Cohort-03/rust/all-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/gbd-level1-csmf.png) | ![Cohort-03 Python gbd-level1-csmf.png](./evidence/charts/Cohort-03/python/gbd-level1-csmf.png) | ![Cohort-03 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-03/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-03/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-03/rust/undetermined-figure.png) | ![Cohort-03 Python undetermined-figure.png](./evidence/charts/Cohort-03/python/undetermined-figure.png) | ![Cohort-03 Rust undetermined-figure.png](./evidence/charts/Cohort-03/rust/undetermined-figure.png) |

---

### Cohort-04

- **VAs Scored**: `820` | **`cause34` Parity**: `820 / 820` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `7392.8 ms` | **Rust Cold CLI**: `808.4 ms` (**`9.15x`**) | **Rust Warm FFI**: `628.2 ms` (**`11.77x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/adult-csmf-figure.png) | ![Cohort-04 Python adult-csmf-figure.png](./evidence/charts/Cohort-04/python/adult-csmf-figure.png) | ![Cohort-04 Rust adult-csmf-figure.png](./evidence/charts/Cohort-04/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/adult-female-csmf-figure.png) | ![Cohort-04 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-04/python/adult-female-csmf-figure.png) | ![Cohort-04 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-04/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/adult-male-csmf-figure.png) | ![Cohort-04 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-04/python/adult-male-csmf-figure.png) | ![Cohort-04 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-04/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/all-csmf-figure.png) | ![Cohort-04 Python all-csmf-figure.png](./evidence/charts/Cohort-04/python/all-csmf-figure.png) | ![Cohort-04 Rust all-csmf-figure.png](./evidence/charts/Cohort-04/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/all-figure.png) | ![Cohort-04 Python all-figure.png](./evidence/charts/Cohort-04/python/all-figure.png) | ![Cohort-04 Rust all-figure.png](./evidence/charts/Cohort-04/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/child-csmf-figure.png) | ![Cohort-04 Python child-csmf-figure.png](./evidence/charts/Cohort-04/python/child-csmf-figure.png) | ![Cohort-04 Rust child-csmf-figure.png](./evidence/charts/Cohort-04/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/gbd-level1-csmf.png) | ![Cohort-04 Python gbd-level1-csmf.png](./evidence/charts/Cohort-04/python/gbd-level1-csmf.png) | ![Cohort-04 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-04/rust/gbd-level1-csmf.png) |
| **Neonate CSMF Bar Chart (`neonate-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/neonate-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/neonate-csmf-figure.png) | ![Cohort-04 Python neonate-csmf-figure.png](./evidence/charts/Cohort-04/python/neonate-csmf-figure.png) | ![Cohort-04 Rust neonate-csmf-figure.png](./evidence/charts/Cohort-04/rust/neonate-csmf-figure.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-04/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-04/rust/undetermined-figure.png) | ![Cohort-04 Python undetermined-figure.png](./evidence/charts/Cohort-04/python/undetermined-figure.png) | ![Cohort-04 Rust undetermined-figure.png](./evidence/charts/Cohort-04/rust/undetermined-figure.png) |

---

### Cohort-05

- **VAs Scored**: `1,258` | **`cause34` Parity**: `1,258 / 1,258` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `8039.5 ms` | **Rust Cold CLI**: `1129.1 ms` (**`7.12x`**) | **Rust Warm FFI**: `958.5 ms` (**`8.39x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/adult-csmf-figure.png) | ![Cohort-05 Python adult-csmf-figure.png](./evidence/charts/Cohort-05/python/adult-csmf-figure.png) | ![Cohort-05 Rust adult-csmf-figure.png](./evidence/charts/Cohort-05/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/adult-female-csmf-figure.png) | ![Cohort-05 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-05/python/adult-female-csmf-figure.png) | ![Cohort-05 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-05/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/adult-male-csmf-figure.png) | ![Cohort-05 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-05/python/adult-male-csmf-figure.png) | ![Cohort-05 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-05/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/all-csmf-figure.png) | ![Cohort-05 Python all-csmf-figure.png](./evidence/charts/Cohort-05/python/all-csmf-figure.png) | ![Cohort-05 Rust all-csmf-figure.png](./evidence/charts/Cohort-05/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/all-figure.png) | ![Cohort-05 Python all-figure.png](./evidence/charts/Cohort-05/python/all-figure.png) | ![Cohort-05 Rust all-figure.png](./evidence/charts/Cohort-05/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/child-csmf-figure.png) | ![Cohort-05 Python child-csmf-figure.png](./evidence/charts/Cohort-05/python/child-csmf-figure.png) | ![Cohort-05 Rust child-csmf-figure.png](./evidence/charts/Cohort-05/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/gbd-level1-csmf.png) | ![Cohort-05 Python gbd-level1-csmf.png](./evidence/charts/Cohort-05/python/gbd-level1-csmf.png) | ![Cohort-05 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-05/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-05/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-05/rust/undetermined-figure.png) | ![Cohort-05 Python undetermined-figure.png](./evidence/charts/Cohort-05/python/undetermined-figure.png) | ![Cohort-05 Rust undetermined-figure.png](./evidence/charts/Cohort-05/rust/undetermined-figure.png) |

---

### Cohort-06

- **VAs Scored**: `1,388` | **`cause34` Parity**: `1,388 / 1,388` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `8994.7 ms` | **Rust Cold CLI**: `1220.2 ms` (**`7.37x`**) | **Rust Warm FFI**: `1065.7 ms` (**`8.44x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/adult-csmf-figure.png) | ![Cohort-06 Python adult-csmf-figure.png](./evidence/charts/Cohort-06/python/adult-csmf-figure.png) | ![Cohort-06 Rust adult-csmf-figure.png](./evidence/charts/Cohort-06/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/adult-female-csmf-figure.png) | ![Cohort-06 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-06/python/adult-female-csmf-figure.png) | ![Cohort-06 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-06/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/adult-male-csmf-figure.png) | ![Cohort-06 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-06/python/adult-male-csmf-figure.png) | ![Cohort-06 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-06/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/all-csmf-figure.png) | ![Cohort-06 Python all-csmf-figure.png](./evidence/charts/Cohort-06/python/all-csmf-figure.png) | ![Cohort-06 Rust all-csmf-figure.png](./evidence/charts/Cohort-06/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/all-figure.png) | ![Cohort-06 Python all-figure.png](./evidence/charts/Cohort-06/python/all-figure.png) | ![Cohort-06 Rust all-figure.png](./evidence/charts/Cohort-06/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/child-csmf-figure.png) | ![Cohort-06 Python child-csmf-figure.png](./evidence/charts/Cohort-06/python/child-csmf-figure.png) | ![Cohort-06 Rust child-csmf-figure.png](./evidence/charts/Cohort-06/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/gbd-level1-csmf.png) | ![Cohort-06 Python gbd-level1-csmf.png](./evidence/charts/Cohort-06/python/gbd-level1-csmf.png) | ![Cohort-06 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-06/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-06/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-06/rust/undetermined-figure.png) | ![Cohort-06 Python undetermined-figure.png](./evidence/charts/Cohort-06/python/undetermined-figure.png) | ![Cohort-06 Rust undetermined-figure.png](./evidence/charts/Cohort-06/rust/undetermined-figure.png) |

---

### Cohort-07

- **VAs Scored**: `1,169` | **`cause34` Parity**: `1,169 / 1,169` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `8532.9 ms` | **Rust Cold CLI**: `1016.5 ms` (**`8.39x`**) | **Rust Warm FFI**: `861.0 ms` (**`9.91x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/adult-csmf-figure.png) | ![Cohort-07 Python adult-csmf-figure.png](./evidence/charts/Cohort-07/python/adult-csmf-figure.png) | ![Cohort-07 Rust adult-csmf-figure.png](./evidence/charts/Cohort-07/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/adult-female-csmf-figure.png) | ![Cohort-07 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-07/python/adult-female-csmf-figure.png) | ![Cohort-07 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-07/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/adult-male-csmf-figure.png) | ![Cohort-07 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-07/python/adult-male-csmf-figure.png) | ![Cohort-07 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-07/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/all-csmf-figure.png) | ![Cohort-07 Python all-csmf-figure.png](./evidence/charts/Cohort-07/python/all-csmf-figure.png) | ![Cohort-07 Rust all-csmf-figure.png](./evidence/charts/Cohort-07/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/all-figure.png) | ![Cohort-07 Python all-figure.png](./evidence/charts/Cohort-07/python/all-figure.png) | ![Cohort-07 Rust all-figure.png](./evidence/charts/Cohort-07/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/child-csmf-figure.png) | ![Cohort-07 Python child-csmf-figure.png](./evidence/charts/Cohort-07/python/child-csmf-figure.png) | ![Cohort-07 Rust child-csmf-figure.png](./evidence/charts/Cohort-07/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/gbd-level1-csmf.png) | ![Cohort-07 Python gbd-level1-csmf.png](./evidence/charts/Cohort-07/python/gbd-level1-csmf.png) | ![Cohort-07 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-07/rust/gbd-level1-csmf.png) |
| **Neonate CSMF Bar Chart (`neonate-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/neonate-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/neonate-csmf-figure.png) | ![Cohort-07 Python neonate-csmf-figure.png](./evidence/charts/Cohort-07/python/neonate-csmf-figure.png) | ![Cohort-07 Rust neonate-csmf-figure.png](./evidence/charts/Cohort-07/rust/neonate-csmf-figure.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-07/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-07/rust/undetermined-figure.png) | ![Cohort-07 Python undetermined-figure.png](./evidence/charts/Cohort-07/python/undetermined-figure.png) | ![Cohort-07 Rust undetermined-figure.png](./evidence/charts/Cohort-07/rust/undetermined-figure.png) |

---

### Cohort-08

- **VAs Scored**: `178` | **`cause34` Parity**: `178 / 178` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `4913.8 ms` | **Rust Cold CLI**: `382.0 ms` (**`12.86x`**) | **Rust Warm FFI**: `213.2 ms` (**`23.04x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/adult-csmf-figure.png) | ![Cohort-08 Python adult-csmf-figure.png](./evidence/charts/Cohort-08/python/adult-csmf-figure.png) | ![Cohort-08 Rust adult-csmf-figure.png](./evidence/charts/Cohort-08/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/adult-female-csmf-figure.png) | ![Cohort-08 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-08/python/adult-female-csmf-figure.png) | ![Cohort-08 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-08/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/adult-male-csmf-figure.png) | ![Cohort-08 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-08/python/adult-male-csmf-figure.png) | ![Cohort-08 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-08/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/all-csmf-figure.png) | ![Cohort-08 Python all-csmf-figure.png](./evidence/charts/Cohort-08/python/all-csmf-figure.png) | ![Cohort-08 Rust all-csmf-figure.png](./evidence/charts/Cohort-08/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/all-figure.png) | ![Cohort-08 Python all-figure.png](./evidence/charts/Cohort-08/python/all-figure.png) | ![Cohort-08 Rust all-figure.png](./evidence/charts/Cohort-08/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/child-csmf-figure.png) | ![Cohort-08 Python child-csmf-figure.png](./evidence/charts/Cohort-08/python/child-csmf-figure.png) | ![Cohort-08 Rust child-csmf-figure.png](./evidence/charts/Cohort-08/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/gbd-level1-csmf.png) | ![Cohort-08 Python gbd-level1-csmf.png](./evidence/charts/Cohort-08/python/gbd-level1-csmf.png) | ![Cohort-08 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-08/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-08/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-08/rust/undetermined-figure.png) | ![Cohort-08 Python undetermined-figure.png](./evidence/charts/Cohort-08/python/undetermined-figure.png) | ![Cohort-08 Rust undetermined-figure.png](./evidence/charts/Cohort-08/rust/undetermined-figure.png) |

---

### Cohort-09

- **VAs Scored**: `59` | **`cause34` Parity**: `59 / 59` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `3959.8 ms` | **Rust Cold CLI**: `265.6 ms` (**`14.91x`**) | **Rust Warm FFI**: `107.5 ms` (**`36.83x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/adult-csmf-figure.png) | ![Cohort-09 Python adult-csmf-figure.png](./evidence/charts/Cohort-09/python/adult-csmf-figure.png) | ![Cohort-09 Rust adult-csmf-figure.png](./evidence/charts/Cohort-09/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/adult-female-csmf-figure.png) | ![Cohort-09 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-09/python/adult-female-csmf-figure.png) | ![Cohort-09 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-09/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/adult-male-csmf-figure.png) | ![Cohort-09 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-09/python/adult-male-csmf-figure.png) | ![Cohort-09 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-09/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/all-csmf-figure.png) | ![Cohort-09 Python all-csmf-figure.png](./evidence/charts/Cohort-09/python/all-csmf-figure.png) | ![Cohort-09 Rust all-csmf-figure.png](./evidence/charts/Cohort-09/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/all-figure.png) | ![Cohort-09 Python all-figure.png](./evidence/charts/Cohort-09/python/all-figure.png) | ![Cohort-09 Rust all-figure.png](./evidence/charts/Cohort-09/rust/all-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/gbd-level1-csmf.png) | ![Cohort-09 Python gbd-level1-csmf.png](./evidence/charts/Cohort-09/python/gbd-level1-csmf.png) | ![Cohort-09 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-09/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-09/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-09/rust/undetermined-figure.png) | ![Cohort-09 Python undetermined-figure.png](./evidence/charts/Cohort-09/python/undetermined-figure.png) | ![Cohort-09 Rust undetermined-figure.png](./evidence/charts/Cohort-09/rust/undetermined-figure.png) |

---

### Cohort-10

- **VAs Scored**: `23` | **`cause34` Parity**: `23 / 23` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `3439.8 ms` | **Rust Cold CLI**: `244.1 ms` (**`14.09x`**) | **Rust Warm FFI**: `82.5 ms` (**`41.71x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/adult-csmf-figure.png) | ![Cohort-10 Python adult-csmf-figure.png](./evidence/charts/Cohort-10/python/adult-csmf-figure.png) | ![Cohort-10 Rust adult-csmf-figure.png](./evidence/charts/Cohort-10/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/adult-female-csmf-figure.png) | ![Cohort-10 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-10/python/adult-female-csmf-figure.png) | ![Cohort-10 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-10/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/adult-male-csmf-figure.png) | ![Cohort-10 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-10/python/adult-male-csmf-figure.png) | ![Cohort-10 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-10/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/all-csmf-figure.png) | ![Cohort-10 Python all-csmf-figure.png](./evidence/charts/Cohort-10/python/all-csmf-figure.png) | ![Cohort-10 Rust all-csmf-figure.png](./evidence/charts/Cohort-10/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/all-figure.png) | ![Cohort-10 Python all-figure.png](./evidence/charts/Cohort-10/python/all-figure.png) | ![Cohort-10 Rust all-figure.png](./evidence/charts/Cohort-10/rust/all-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/gbd-level1-csmf.png) | ![Cohort-10 Python gbd-level1-csmf.png](./evidence/charts/Cohort-10/python/gbd-level1-csmf.png) | ![Cohort-10 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-10/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-10/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-10/rust/undetermined-figure.png) | ![Cohort-10 Python undetermined-figure.png](./evidence/charts/Cohort-10/python/undetermined-figure.png) | ![Cohort-10 Rust undetermined-figure.png](./evidence/charts/Cohort-10/rust/undetermined-figure.png) |

---

### Cohort-11

- **VAs Scored**: `116` | **`cause34` Parity**: `116 / 116` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `4414.2 ms` | **Rust Cold CLI**: `300.8 ms` (**`14.67x`**) | **Rust Warm FFI**: `148.0 ms` (**`29.83x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/adult-csmf-figure.png) | ![Cohort-11 Python adult-csmf-figure.png](./evidence/charts/Cohort-11/python/adult-csmf-figure.png) | ![Cohort-11 Rust adult-csmf-figure.png](./evidence/charts/Cohort-11/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/adult-female-csmf-figure.png) | ![Cohort-11 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-11/python/adult-female-csmf-figure.png) | ![Cohort-11 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-11/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/adult-male-csmf-figure.png) | ![Cohort-11 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-11/python/adult-male-csmf-figure.png) | ![Cohort-11 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-11/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/all-csmf-figure.png) | ![Cohort-11 Python all-csmf-figure.png](./evidence/charts/Cohort-11/python/all-csmf-figure.png) | ![Cohort-11 Rust all-csmf-figure.png](./evidence/charts/Cohort-11/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/all-figure.png) | ![Cohort-11 Python all-figure.png](./evidence/charts/Cohort-11/python/all-figure.png) | ![Cohort-11 Rust all-figure.png](./evidence/charts/Cohort-11/rust/all-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/gbd-level1-csmf.png) | ![Cohort-11 Python gbd-level1-csmf.png](./evidence/charts/Cohort-11/python/gbd-level1-csmf.png) | ![Cohort-11 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-11/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-11/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-11/rust/undetermined-figure.png) | ![Cohort-11 Python undetermined-figure.png](./evidence/charts/Cohort-11/python/undetermined-figure.png) | ![Cohort-11 Rust undetermined-figure.png](./evidence/charts/Cohort-11/rust/undetermined-figure.png) |

---

### Cohort-12

- **VAs Scored**: `256` | **`cause34` Parity**: `256 / 256` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `6388.6 ms` | **Rust Cold CLI**: `468.4 ms` (**`13.64x`**) | **Rust Warm FFI**: `279.2 ms` (**`22.89x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/adult-csmf-figure.png) | ![Cohort-12 Python adult-csmf-figure.png](./evidence/charts/Cohort-12/python/adult-csmf-figure.png) | ![Cohort-12 Rust adult-csmf-figure.png](./evidence/charts/Cohort-12/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/adult-female-csmf-figure.png) | ![Cohort-12 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-12/python/adult-female-csmf-figure.png) | ![Cohort-12 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-12/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/adult-male-csmf-figure.png) | ![Cohort-12 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-12/python/adult-male-csmf-figure.png) | ![Cohort-12 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-12/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/all-csmf-figure.png) | ![Cohort-12 Python all-csmf-figure.png](./evidence/charts/Cohort-12/python/all-csmf-figure.png) | ![Cohort-12 Rust all-csmf-figure.png](./evidence/charts/Cohort-12/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/all-figure.png) | ![Cohort-12 Python all-figure.png](./evidence/charts/Cohort-12/python/all-figure.png) | ![Cohort-12 Rust all-figure.png](./evidence/charts/Cohort-12/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/child-csmf-figure.png) | ![Cohort-12 Python child-csmf-figure.png](./evidence/charts/Cohort-12/python/child-csmf-figure.png) | ![Cohort-12 Rust child-csmf-figure.png](./evidence/charts/Cohort-12/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/gbd-level1-csmf.png) | ![Cohort-12 Python gbd-level1-csmf.png](./evidence/charts/Cohort-12/python/gbd-level1-csmf.png) | ![Cohort-12 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-12/rust/gbd-level1-csmf.png) |
| **Neonate CSMF Bar Chart (`neonate-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/neonate-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/neonate-csmf-figure.png) | ![Cohort-12 Python neonate-csmf-figure.png](./evidence/charts/Cohort-12/python/neonate-csmf-figure.png) | ![Cohort-12 Rust neonate-csmf-figure.png](./evidence/charts/Cohort-12/rust/neonate-csmf-figure.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-12/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-12/rust/undetermined-figure.png) | ![Cohort-12 Python undetermined-figure.png](./evidence/charts/Cohort-12/python/undetermined-figure.png) | ![Cohort-12 Rust undetermined-figure.png](./evidence/charts/Cohort-12/rust/undetermined-figure.png) |

---

### Cohort-13

- **VAs Scored**: `255` | **`cause34` Parity**: `255 / 255` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `5452.4 ms` | **Rust Cold CLI**: `461.9 ms` (**`11.80x`**) | **Rust Warm FFI**: `283.1 ms` (**`19.26x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/adult-csmf-figure.png) | ![Cohort-13 Python adult-csmf-figure.png](./evidence/charts/Cohort-13/python/adult-csmf-figure.png) | ![Cohort-13 Rust adult-csmf-figure.png](./evidence/charts/Cohort-13/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/adult-female-csmf-figure.png) | ![Cohort-13 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-13/python/adult-female-csmf-figure.png) | ![Cohort-13 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-13/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/adult-male-csmf-figure.png) | ![Cohort-13 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-13/python/adult-male-csmf-figure.png) | ![Cohort-13 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-13/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/all-csmf-figure.png) | ![Cohort-13 Python all-csmf-figure.png](./evidence/charts/Cohort-13/python/all-csmf-figure.png) | ![Cohort-13 Rust all-csmf-figure.png](./evidence/charts/Cohort-13/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/all-figure.png) | ![Cohort-13 Python all-figure.png](./evidence/charts/Cohort-13/python/all-figure.png) | ![Cohort-13 Rust all-figure.png](./evidence/charts/Cohort-13/rust/all-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/gbd-level1-csmf.png) | ![Cohort-13 Python gbd-level1-csmf.png](./evidence/charts/Cohort-13/python/gbd-level1-csmf.png) | ![Cohort-13 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-13/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-13/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-13/rust/undetermined-figure.png) | ![Cohort-13 Python undetermined-figure.png](./evidence/charts/Cohort-13/python/undetermined-figure.png) | ![Cohort-13 Rust undetermined-figure.png](./evidence/charts/Cohort-13/rust/undetermined-figure.png) |

---

### Cohort-14

- **VAs Scored**: `227` | **`cause34` Parity**: `227 / 227` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `6263.1 ms` | **Rust Cold CLI**: `424.5 ms` (**`14.75x`**) | **Rust Warm FFI**: `244.6 ms` (**`25.61x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/adult-csmf-figure.png) | ![Cohort-14 Python adult-csmf-figure.png](./evidence/charts/Cohort-14/python/adult-csmf-figure.png) | ![Cohort-14 Rust adult-csmf-figure.png](./evidence/charts/Cohort-14/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/adult-female-csmf-figure.png) | ![Cohort-14 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-14/python/adult-female-csmf-figure.png) | ![Cohort-14 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-14/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/adult-male-csmf-figure.png) | ![Cohort-14 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-14/python/adult-male-csmf-figure.png) | ![Cohort-14 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-14/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/all-csmf-figure.png) | ![Cohort-14 Python all-csmf-figure.png](./evidence/charts/Cohort-14/python/all-csmf-figure.png) | ![Cohort-14 Rust all-csmf-figure.png](./evidence/charts/Cohort-14/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/all-figure.png) | ![Cohort-14 Python all-figure.png](./evidence/charts/Cohort-14/python/all-figure.png) | ![Cohort-14 Rust all-figure.png](./evidence/charts/Cohort-14/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/child-csmf-figure.png) | ![Cohort-14 Python child-csmf-figure.png](./evidence/charts/Cohort-14/python/child-csmf-figure.png) | ![Cohort-14 Rust child-csmf-figure.png](./evidence/charts/Cohort-14/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/gbd-level1-csmf.png) | ![Cohort-14 Python gbd-level1-csmf.png](./evidence/charts/Cohort-14/python/gbd-level1-csmf.png) | ![Cohort-14 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-14/rust/gbd-level1-csmf.png) |
| **Neonate CSMF Bar Chart (`neonate-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/neonate-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/neonate-csmf-figure.png) | ![Cohort-14 Python neonate-csmf-figure.png](./evidence/charts/Cohort-14/python/neonate-csmf-figure.png) | ![Cohort-14 Rust neonate-csmf-figure.png](./evidence/charts/Cohort-14/rust/neonate-csmf-figure.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-14/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-14/rust/undetermined-figure.png) | ![Cohort-14 Python undetermined-figure.png](./evidence/charts/Cohort-14/python/undetermined-figure.png) | ![Cohort-14 Rust undetermined-figure.png](./evidence/charts/Cohort-14/rust/undetermined-figure.png) |

---

### Cohort-15

- **VAs Scored**: `227` | **`cause34` Parity**: `227 / 227` (`100.00%`) | **CSV Cell Diffs**: `0` | **Python**: `5718.8 ms` | **Rust Cold CLI**: `428.8 ms` (**`13.34x`**) | **Rust Warm FFI**: `263.0 ms` (**`21.74x`**)

| Chart | Upstream Python (`matplotlib`) | Pure-Rust `smartva-rust` (`plotters`) |
| --- | :---: | :---: |
| **Adult CSMF Bar Chart (`adult-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/adult-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/adult-csmf-figure.png) | ![Cohort-15 Python adult-csmf-figure.png](./evidence/charts/Cohort-15/python/adult-csmf-figure.png) | ![Cohort-15 Rust adult-csmf-figure.png](./evidence/charts/Cohort-15/rust/adult-csmf-figure.png) |
| **Adult Female CSMF Bar Chart (`adult-female-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/adult-female-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/adult-female-csmf-figure.png) | ![Cohort-15 Python adult-female-csmf-figure.png](./evidence/charts/Cohort-15/python/adult-female-csmf-figure.png) | ![Cohort-15 Rust adult-female-csmf-figure.png](./evidence/charts/Cohort-15/rust/adult-female-csmf-figure.png) |
| **Adult Male CSMF Bar Chart (`adult-male-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/adult-male-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/adult-male-csmf-figure.png) | ![Cohort-15 Python adult-male-csmf-figure.png](./evidence/charts/Cohort-15/python/adult-male-csmf-figure.png) | ![Cohort-15 Rust adult-male-csmf-figure.png](./evidence/charts/Cohort-15/rust/adult-male-csmf-figure.png) |
| **All-Age CSMF Bar Chart (`all-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/all-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/all-csmf-figure.png) | ![Cohort-15 Python all-csmf-figure.png](./evidence/charts/Cohort-15/python/all-csmf-figure.png) | ![Cohort-15 Rust all-csmf-figure.png](./evidence/charts/Cohort-15/rust/all-csmf-figure.png) |
| **Age & Sex Distribution by Module (`all-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/all-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/all-figure.png) | ![Cohort-15 Python all-figure.png](./evidence/charts/Cohort-15/python/all-figure.png) | ![Cohort-15 Rust all-figure.png](./evidence/charts/Cohort-15/rust/all-figure.png) |
| **Child CSMF Bar Chart (`child-csmf-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/child-csmf-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/child-csmf-figure.png) | ![Cohort-15 Python child-csmf-figure.png](./evidence/charts/Cohort-15/python/child-csmf-figure.png) | ![Cohort-15 Rust child-csmf-figure.png](./evidence/charts/Cohort-15/rust/child-csmf-figure.png) |
| **GBD Level-1 Cause Fractions Pie Chart (`gbd-level1-csmf.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/gbd-level1-csmf.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/gbd-level1-csmf.png) | ![Cohort-15 Python gbd-level1-csmf.png](./evidence/charts/Cohort-15/python/gbd-level1-csmf.png) | ![Cohort-15 Rust gbd-level1-csmf.png](./evidence/charts/Cohort-15/rust/gbd-level1-csmf.png) |
| **Undetermined COD Distribution (`undetermined-figure.png`)**<br>[Python PNG](./evidence/charts/Cohort-15/python/undetermined-figure.png) · [Rust PNG](./evidence/charts/Cohort-15/rust/undetermined-figure.png) | ![Cohort-15 Python undetermined-figure.png](./evidence/charts/Cohort-15/python/undetermined-figure.png) | ![Cohort-15 Rust undetermined-figure.png](./evidence/charts/Cohort-15/rust/undetermined-figure.png) |

---
