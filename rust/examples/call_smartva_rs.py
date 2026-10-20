#!/usr/bin/env python3
"""Example Python script calling the Pure-Rust SmartVA-Analyze engine (`smartva-rs`).

Note: The Rust crate (`smartva-rust` / `smartva-rs`) is 100% pure native Rust
with ZERO runtime Python dependency. This script is provided purely as an example
of how external Python callers can invoke the compiled Rust CLI (`smartva-rs`) or
C-ABI shared library (`libsmartva_rust`).
"""

from __future__ import annotations

import csv
import ctypes
import subprocess
import sys
import tempfile
from pathlib import Path

RUST_DIR = Path(__file__).resolve().parents[1]
RELEASE_DIR = RUST_DIR / "target" / "release"


def run_via_cli(input_csv: Path, output_dir: Path, country: str = "IND") -> None:
    """Invoke the drop-in `smartva-rs` binary via subprocess (same flags as `va_cli.py`)."""
    bin_path = RELEASE_DIR / "smartva-rs"
    if not bin_path.exists():
        raise FileNotFoundError(
            f"smartva-rs binary not found at {bin_path}; build with `cargo build --release` first."
        )
    cmd = [
        str(bin_path),
        "--country",
        country,
        "--hiv",
        "True",
        "--malaria",
        "True",
        "--hce",
        "True",
        "--freetext",
        "True",
        "--figures",
        "True",
        str(input_csv),
        str(output_dir),
    ]
    subprocess.run(cmd, check=True)


def run_via_ffi(input_csv: Path, output_dir: Path, country: str = "IND") -> None:
    """Invoke `libsmartva_rust` in-process via C-ABI FFI (`ctypes`)."""
    for ext in ("dylib", "so", "dll"):
        candidate = RELEASE_DIR / f"libsmartva_rust.{ext}"
        if candidate.exists():
            lib_path = candidate
            break
    else:
        raise FileNotFoundError(
            f"Shared library libsmartva_rust not found in {RELEASE_DIR}; build with `cargo build --release`."
        )

    lib = ctypes.CDLL(str(lib_path))
    lib.smartva_warm_cache.argtypes = []
    lib.smartva_warm_cache.restype = ctypes.c_int

    lib.smartva_run_directory.argtypes = [
        ctypes.c_char_p,
        ctypes.c_char_p,
        ctypes.c_char_p,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.POINTER(ctypes.c_char_p),
    ]
    lib.smartva_run_directory.restype = ctypes.c_int

    lib.smartva_warm_cache()
    err_ptr = ctypes.c_char_p()
    rc = lib.smartva_run_directory(
        str(input_csv).encode("utf-8"),
        str(output_dir).encode("utf-8"),
        country.encode("utf-8"),
        1,  # hiv
        1,  # malaria
        1,  # hce
        1,  # freetext
        1,  # figures
        ctypes.byref(err_ptr),
    )
    if rc != 0:
        err_msg = err_ptr.value.decode("utf-8") if err_ptr.value else f"FFI exit code {rc}"
        raise RuntimeError(err_msg)


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <input.csv> [output_dir] [country]")
        sys.exit(1)

    in_csv = Path(sys.argv[1]).resolve()
    out_dir = (
        Path(sys.argv[2]).resolve()
        if len(sys.argv) >= 3
        else Path(tempfile.mkdtemp(prefix="smartva_rs_example_"))
    )
    iso3 = sys.argv[3] if len(sys.argv) >= 4 else "IND"

    print(f"Running Pure-Rust SmartVA via FFI on {in_csv} -> {out_dir} ...")
    run_via_ffi(in_csv, out_dir, country=iso3)

    pred_csv = out_dir / "1-individual-cause-of-death" / "individual-cause-of-death.csv"
    with pred_csv.open() as f:
        rows = list(csv.DictReader(f))
    print(f"Completed! Scored {len(rows)} VAs into {out_dir}")
