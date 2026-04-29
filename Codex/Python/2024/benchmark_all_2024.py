#!/usr/bin/env python3
from __future__ import annotations

import argparse
import math
import shutil
import statistics
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

RUNTIME_PREFIX = "Runtime:"
MEMORY_PREFIX = "__MAX_RSS_KB__="
YEAR = "2024"


def detect_time_binary() -> Path | None:
    for candidate in ("/usr/bin/time", shutil.which("gtime"), shutil.which("time")):
        if not candidate:
            continue
        path = Path(candidate)
        try:
            probe = subprocess.run(
                [str(path), "-f", MEMORY_PREFIX, "true"],
                capture_output=True,
                text=True,
                timeout=2.0,
                check=False,
            )
        except (OSError, subprocess.SubprocessError):
            continue
        if probe.returncode == 0 and MEMORY_PREFIX in probe.stderr:
            return path
    return None


TIME_BINARY = detect_time_binary()


def discover(base: Path) -> list[Path]:
    return sorted(base.glob(f"Day*b_Solution_{YEAR}.py"))


def clean(stdout: str) -> str:
    return "\n".join(
        line.rstrip() for line in stdout.splitlines() if not line.startswith(RUNTIME_PREFIX)
    ).strip()


@dataclass
class RunResult:
    stdout: str
    stderr: str
    returncode: int
    max_rss_kb: int | None


def time_command(solution: Path) -> list[str]:
    if TIME_BINARY is not None:
        return [str(TIME_BINARY), "-f", f"{MEMORY_PREFIX}%M", sys.executable, solution.name]
    return [sys.executable, solution.name]


def split_stderr_and_memory(stderr: str) -> tuple[str, int | None]:
    lines = stderr.splitlines()
    if TIME_BINARY is not None and lines:
        last_line = lines[-1].strip()
        if last_line.startswith(MEMORY_PREFIX):
            value = last_line.removeprefix(MEMORY_PREFIX)
            lines = lines[:-1]
            return "\n".join(lines).strip(), int(value) if value.isdigit() else None
    return stderr.strip(), None


def run_once(solution: Path, timeout: float) -> RunResult:
    completed = subprocess.run(
        time_command(solution),
        cwd=solution.parent,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    stderr, max_rss_kb = split_stderr_and_memory(completed.stderr)
    return RunResult(
        stdout=completed.stdout,
        stderr=stderr,
        returncode=completed.returncode,
        max_rss_kb=max_rss_kb,
    )


def memory_stats(samples: list[int | None]) -> str:
    measured = [sample for sample in samples if sample is not None]
    if not measured:
        return "memory= n/a (GNU time not available)"
    mean_kb = statistics.fmean(measured)
    return (
        f"memory_mean= {mean_kb / 1024.0:.3f} MiB "
        f"memory_min= {min(measured) / 1024.0:.3f} MiB "
        f"memory_max= {max(measured) / 1024.0:.3f} MiB"
    )


def bench(solution: Path, repeats: int, warmups: int, timeout: float) -> tuple[str, str]:
    try:
        baseline = run_once(solution, timeout)
    except subprocess.TimeoutExpired:
        return "TIMEOUT", f"initial run exceeded {timeout:.1f}s"
    if baseline.returncode != 0:
        msg = baseline.stderr.strip() or baseline.stdout.strip() or f"exit code {baseline.returncode}"
        return "FAILED", msg

    expected = clean(baseline.stdout)
    for _ in range(warmups):
        proc = run_once(solution, timeout)
        if proc.returncode != 0:
            msg = proc.stderr.strip() or proc.stdout.strip() or f"exit code {proc.returncode}"
            return "FAILED", f"warmup failed: {msg}"
        if clean(proc.stdout) != expected:
            return "MISMATCH", "warmup output differed from baseline"

    samples: list[float] = []
    memory_samples: list[int | None] = []
    for run in range(1, repeats + 1):
        start = time.perf_counter()
        proc = run_once(solution, timeout)
        elapsed = (time.perf_counter() - start) * 1000.0
        if proc.returncode != 0:
            msg = proc.stderr.strip() or proc.stdout.strip() or f"exit code {proc.returncode}"
            return "FAILED", f"run {run} failed: {msg}"
        if clean(proc.stdout) != expected:
            return "MISMATCH", f"run {run} output differed from baseline"
        samples.append(elapsed)
        memory_samples.append(proc.max_rss_kb)

    avg = statistics.fmean(samples)
    sigma = statistics.stdev(samples) if len(samples) > 1 else 0.0
    return "OK", (
        f"mean= {avg:.3f} ms min= {min(samples):.3f} ms max= {max(samples):.3f} ms "
        f"stddev= {sigma:.3f} ms {memory_stats(memory_samples)} ({repeats} measured runs)"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repeats", type=int, default=10)
    parser.add_argument("--warmups", type=int, default=1)
    parser.add_argument("--timeout", type=float, default=30.0)
    args = parser.parse_args()
    if args.repeats < 1 or args.warmups < 0 or not math.isfinite(args.timeout) or args.timeout <= 0:
        raise SystemExit("invalid benchmark arguments")

    solutions = discover(Path(__file__).resolve().parent)
    print(f"Python {YEAR} benchmark report")
    print("=" * 28)
    failures = 0
    for solution in solutions:
        status, detail = bench(solution, args.repeats, args.warmups, args.timeout)
        print(f"[{status}] {solution.name}: {detail}")
        failures += status != "OK"
    print("-" * 28)
    print(f"Solutions discovered: {len(solutions)}")
    print(f"Successful benchmarks: {len(solutions) - failures}")
    print(f"Failures: {failures}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
