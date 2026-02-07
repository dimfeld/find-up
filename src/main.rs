use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_else(|| OsString::from("find-up"));

    let mut use_std = false;
    let mut names: Vec<OsString> = Vec::new();

    for arg in args {
        if arg == "--std" {
            use_std = true;
        } else {
            names.push(arg);
        }
    }

    if use_std {
        names = std_names();
    }

    if names.is_empty() {
        eprintln!(
            "Usage: {} [--std] <name> [name ...]\n       {} --std",
            program.to_string_lossy(),
            program.to_string_lossy()
        );
        process::exit(2);
    }

    let start = match env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("find-up: failed to get current directory: {e}");
            process::exit(2);
        }
    };

    match find_up_closest(&start, &names) {
        Some(found) => {
            let out = found
                .canonicalize()
                .unwrap_or_else(|_| absolutize_fallback(&found));
            println!("{}", out.display());
            process::exit(0);
        }
        None => process::exit(1),
    }
}

/// Find the *closest* ancestor directory (including `start`) that contains
/// any of `names`. If several names exist at that same closest level, the
/// first name in `names` order is returned.
fn find_up_closest(start: &Path, names: &[OsString]) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();

    loop {
        for name in names {
            let candidate = dir.join(name);
            if candidate.exists() {
                return Some(candidate);
            }
        }

        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return None,
        }
    }
}

fn absolutize_fallback(p: &Path) -> PathBuf {
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(p)
    }
}

/// A "standard" set of markers commonly used to identify project roots across ecosystems.
/// Includes VCS roots (.git, .jj) and language/build-tool markers.
fn std_names() -> Vec<OsString> {
    // Tie-breaker is the order below (first match at closest level wins).
    // Put VCS first because many people treat it as the most universal "root".
    [
        // VCS
        ".git",
        ".jj",
        // Node / JS / TS
        "package.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "bun.lockb",
        "bun.lock",
        "package-lock.json",
        "tsconfig.json",
        "deno.json",
        "deno.jsonc",
        // Rust
        "Cargo.toml",
        // Go
        "go.mod",
        "go.work",
        // Python
        "pyproject.toml",
        "poetry.lock",
        "Pipfile",
        "requirements.txt",
        "setup.py",
        "setup.cfg",
        // Java / Kotlin / Scala
        "pom.xml",
        "build.gradle",
        "build.gradle.kts",
        "settings.gradle",
        "settings.gradle.kts",
        // .NET
        ".sln",
        // PHP
        "composer.json",
        // Ruby
        "Gemfile",
        // Elixir
        "mix.exs",
        // Haskell
        "stack.yaml",
        "cabal.project",
        // C / C++
        "CMakeLists.txt",
        "Makefile",
        // Swift
        "Package.swift",
        // OCaml
        "dune-project",
        "opam",
        // Zig
        "build.zig",
        // Lua
        "rockspec",
        // Nix
        "flake.nix",
        "default.nix",
    ]
    .iter()
    .map(OsString::from)
    .collect()
}
