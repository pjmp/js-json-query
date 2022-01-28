use std::{
    fs::{self, File},
    io::{self, stdout, BufRead, Read, Write},
    path::PathBuf,
};

use argh::FromArgs;
use atty::{is, Stream};
use v8::{Context, ContextScope, HandleScope, Isolate, Script, V8};

#[derive(Debug, FromArgs)]
#[doc = "A tool for processing JSON inputs with real JavaScript, no dsl"]
pub(crate) struct App {
    /// path to json file
    #[argh(option, short = 'p', from_str_fn(parse_path))]
    pub(crate) path: Option<PathBuf>,

    /// code to process the json input
    #[argh(short = 's', positional)]
    pub(crate) script: Option<String>,

    /// get version information
    #[argh(switch, short = 'v')]
    pub(crate) version: bool,

    /// js files to include
    #[argh(option, short = 'i', from_str_fn(parse_include_paths))]
    pub(crate) includes: Option<Vec<File>>,
}

fn parse_include_paths(paths: &str) -> Result<Vec<File>, String> {
    let mut paths = paths.split(',').collect::<Vec<&str>>();

    paths.dedup();

    let mut rect: Vec<File> = vec![];

    for path in paths {
        let file = File::open(path);

        if let Ok(path) = file {
            rect.push(path)
        } else {
            return Err("No such file or directory".to_string());
        }
    }

    Ok(rect)
}

fn parse_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);

    if path.is_file() {
        Ok(path)
    } else {
        Err("No such file or directory".to_string())
    }
}

impl App {
    pub(crate) fn new() -> Self {
        argh::from_env::<Self>()
    }

    pub(crate) fn json(&self) -> String {
        if is(Stream::Stdin) & self.path.is_some() {
            let path = self.path.as_ref().unwrap();

            return fs::read_to_string(path).expect("Unable to read file at self.path");
        }

        let stdin = io::stdin();

        stdin.lock().lines().fold(String::new(), |mut acc, line| {
            if let Ok(line) = line {
                acc.push_str(&line);

                acc
            } else {
                acc
            }
        })
    }

    pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
        let app = Self::new();

        if app.version {
            println!("v{} (V8 {})", env!("CARGO_PKG_VERSION"), V8::get_version());
            return Ok(());
        }

        if app.path.is_none() & is(Stream::Stdin) {
            return Err("pass either `--path` or pipe json".into());
        }

        let platform = v8::new_default_platform(0, false).make_shared();
        V8::initialize_platform(platform);
        V8::initialize();

        let isolate = &mut Isolate::new(Default::default());
        let scope = &mut HandleScope::new(isolate);
        let context = Context::new(scope);
        let scope = &mut ContextScope::new(scope, context);

        let user_script = if let Some(ref script) = app.script {
            let includes = if let Some(ref includes) = app.includes {
                includes.iter().fold(String::new(), |mut init, mut file| {
                    let _ = file.read_to_string(&mut init);
                    init
                })
            } else {
                "".to_string()
            };

            format!(
                r#"
                {includes}
                const out = eval("{script}");
                // printing js object just prints `[object Object]`
                // so need to stringify it.
                if (typeof out !== "string") {{
                    JSON.stringify(out, null, 2);
                }} else {{
                    out;
                }}
            "#
            )
        } else {
            "JSON.stringify(it, null, 2)".to_string()
        };

        let input = app.json();
        let input = format!("globalThis.it = {input}; {user_script}");

        let code = v8::String::new(scope, &input).ok_or("v8::String returned no value")?;
        let script =
            Script::compile(scope, code, None).ok_or("Script::compile returned no value")?;
        let result = script.run(scope).ok_or("Local::run returned no value")?;
        let result = result
            .to_string(scope)
            .ok_or("Local::to_string returned no value")?;

        // prevent broken pipe error
        writeln!(&mut stdout(), "{}", result.to_rust_string_lossy(scope))?;

        Ok(())
    }
}
