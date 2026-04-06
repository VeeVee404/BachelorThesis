use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

const YEAR: &str = "2024";
const RUNTIME_PREFIX: &str = "Runtime:";
const MEMORY_PREFIX: &str = "__MAX_RSS_KB__=";
const TIME_BINARY: &str = "/usr/bin/time";

fn discover(base: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(base)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|v| v.to_str()) else { continue; };
        if name.starts_with("Day") && name.ends_with("_Solution_2024.rs") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn clean(stdout: &str) -> String {
    stdout
        .lines()
        .filter(|line| !line.starts_with(RUNTIME_PREFIX))
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn compile_solution(solution: &Path, out_dir: &Path) -> Result<PathBuf, String> {
    let mut bin = out_dir.join(solution.file_stem().ok_or("missing file stem")?);
    if cfg!(windows) {
        bin.set_extension("exe");
    }
    let out = Command::new("rustc")
        .arg(solution)
        .arg("-O")
        .arg("-o")
        .arg(&bin)
        .output()
        .map_err(|e| format!("failed to invoke rustc: {e}"))?;
    if out.status.success() {
        Ok(bin)
    } else {
        Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
    }
}

struct RunResult {
    stdout: String,
    max_rss_kb: Option<u64>,
}

fn split_stderr_and_memory(stderr: &[u8]) -> (String, Option<u64>) {
    let stderr_text = String::from_utf8_lossy(stderr).into_owned();
    let mut lines: Vec<String> = stderr_text.lines().map(str::to_string).collect();
    if Path::new(TIME_BINARY).exists() {
        if let Some(last_line) = lines.last().map(|line| line.trim().to_string()) {
            if let Some(value) = last_line.strip_prefix(MEMORY_PREFIX) {
                let max_rss_kb = value.parse::<u64>().ok();
                lines.pop();
                return (lines.join("\n").trim().to_string(), max_rss_kb);
            }
        }
    }
    (stderr_text.trim().to_string(), None)
}

fn run_once(bin: &Path, cwd: &Path) -> Result<RunResult, String> {
    let use_time = Path::new(TIME_BINARY).exists();
    let mut command = if use_time {
        let mut command = Command::new(TIME_BINARY);
        command.arg("-f").arg(format!("{MEMORY_PREFIX}%M")).arg(bin);
        command
    } else {
        Command::new(bin)
    };
    let out = command
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("failed to execute benchmark target: {e}"))?;
    let (stderr, max_rss_kb) = split_stderr_and_memory(&out.stderr);
    if out.status.success() {
        Ok(RunResult {
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            max_rss_kb,
        })
    } else {
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}

fn stats(values: &[f64]) -> String {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let stddev = if values.len() > 1 {
        let variance = values
            .iter()
            .map(|v| {
                let d = v - mean;
                d * d
            })
            .sum::<f64>()
            / (values.len() as f64 - 1.0);
        variance.sqrt()
    } else {
        0.0
    };
    format!(
        "mean={mean:.3} ms min={min:.3} ms max={max:.3} ms stddev={stddev:.3} ms"
    )
}

fn memory_stats(values: &[Option<u64>]) -> String {
    let measured: Vec<u64> = values.iter().flatten().copied().collect();
    if measured.is_empty() {
        return "memory=n/a".to_string();
    }
    let mean_kb = measured.iter().sum::<u64>() as f64 / measured.len() as f64;
    let min_kb = measured.iter().copied().min().unwrap_or(0) as f64;
    let max_kb = measured.iter().copied().max().unwrap_or(0) as f64;
    format!(
        "memory_mean={:.3} MiB memory_min={:.3} MiB memory_max={:.3} MiB",
        mean_kb / 1024.0,
        min_kb / 1024.0,
        max_kb / 1024.0
    )
}

fn bench(solution: &Path, repeats: usize, warmups: usize, out_dir: &Path) -> (String, String) {
    let bin = match compile_solution(solution, out_dir) {
        Ok(bin) => bin,
        Err(msg) => return ("COMPILE_FAILED".to_string(), msg),
    };
    let expected = match run_once(&bin, solution.parent().unwrap_or_else(|| Path::new("."))) {
        Ok(run) => clean(&run.stdout),
        Err(msg) => return ("FAILED".to_string(), msg),
    };

    for _ in 0..warmups {
        match run_once(&bin, solution.parent().unwrap_or_else(|| Path::new("."))) {
            Ok(run) if clean(&run.stdout) == expected => {}
            Ok(_) => return ("MISMATCH".to_string(), "warmup output differed from baseline".to_string()),
            Err(msg) => return ("FAILED".to_string(), format!("warmup failed: {msg}")),
        }
    }

    let mut samples = Vec::with_capacity(repeats);
    let mut memory_samples = Vec::with_capacity(repeats);
    for run in 1..=repeats {
        let start = Instant::now();
        match run_once(&bin, solution.parent().unwrap_or_else(|| Path::new("."))) {
            Ok(run_result) if clean(&run_result.stdout) == expected => {
                samples.push(start.elapsed().as_secs_f64() * 1000.0);
                memory_samples.push(run_result.max_rss_kb);
            }
            Ok(_) => return ("MISMATCH".to_string(), format!("run {run} output differed from baseline")),
            Err(msg) => return ("FAILED".to_string(), format!("run {run} failed: {msg}")),
        }
    }

    (
        "OK".to_string(),
        format!(
            "{} {} ({} measured runs)",
            stats(&samples),
            memory_stats(&memory_samples),
            repeats
        ),
    )
}

fn parse_usize(args: &[OsString], flag: &str, default: usize) -> Result<usize, String> {
    for pair in args.windows(2) {
        if pair[0] == flag {
            return pair[1]
                .to_str()
                .ok_or_else(|| format!("invalid value for {flag}"))?
                .parse::<usize>()
                .map_err(|_| format!("invalid integer for {flag}"));
        }
    }
    Ok(default)
}

fn main() {
    let args: Vec<OsString> = env::args_os().collect();
    let repeats = parse_usize(&args, "--repeats", 5).unwrap_or_else(|msg| {
        eprintln!("{msg}");
        std::process::exit(2);
    });
    let warmups = parse_usize(&args, "--warmups", 1).unwrap_or_else(|msg| {
        eprintln!("{msg}");
        std::process::exit(2);
    });
    if repeats == 0 {
        eprintln!("--repeats must be at least 1");
        std::process::exit(2);
    }
    let base = PathBuf::from(file!())
        .canonicalize()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| env::current_dir().expect("could not resolve current directory"));
    let out_dir = env::temp_dir().join("rust_2024_bench_artifacts");
    if let Err(err) = fs::create_dir_all(&out_dir) {
        eprintln!("could not create artifact directory: {err}");
        std::process::exit(1);
    }

    let solutions = discover(&base).unwrap_or_else(|err| {
        eprintln!("could not discover solution files: {err}");
        std::process::exit(1);
    });
    println!("Rust {YEAR} benchmark report");
    println!("==========================");
    let mut failures = 0usize;
    for solution in &solutions {
        let (status, detail) = bench(solution, repeats, warmups, &out_dir);
        println!("[{status}] {}: {detail}", solution.file_name().and_then(|v| v.to_str()).unwrap_or("unknown"));
        failures += usize::from(status != "OK");
    }
    println!("--------------------------");
    println!("Solutions discovered: {}", solutions.len());
    println!("Successful benchmarks: {}", solutions.len().saturating_sub(failures));
    println!("Failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
