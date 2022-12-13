use std::{
    fmt::Write,
    fs,
    io::{self},
    path::Path,
    str::FromStr,
};

use regex::Regex;

fn read_line(request: &str) -> String {
    if request.len() > 0 {
        print!("{}", request);
        std::io::Write::flush(&mut io::stdout()).unwrap();
    }
    let mut res = String::new();
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");

    res.trim().to_owned()
}

fn read_valid<T>(request: &str, default_value: T) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
    T: std::fmt::Display,
    T: Clone,
{
    let numbers =
        std::iter::repeat_with(|| read_line(&format!("{} ({}): ", request, default_value.clone())))
            .map(|value_str| {
                if value_str.is_empty() {
                    Ok(default_value.clone())
                } else {
                    value_str.parse()
                }
            })
            .filter(Result::is_ok)
            .map(|x| x.unwrap());

    numbers.take(1).next().unwrap()
}

fn main() {
    let year: u32 = read_valid("Enter year", 2022);
    let day: u32 = read_valid("Enter day number", 1);
    let tasks: u32 = read_valid("Enter task number", 2);

    generate(year, day, tasks)
}

fn generate(year: u32, day: u32, tasks: u32) {
    generate_bench(year, day, tasks);

    let global_library_path = Path::new("src/lib.rs");
    let day_lib_path = format!("src/y{}d{:02}.rs", year % 2000, day);
    let day_lib_path = Path::new(&day_lib_path);

    if !global_library_path.exists() {
        println!("lib.rs not found");
        return;
    }

    for task in 1..=tasks {
        let id = format!("y{}d{:02}t{}", year % 2000, day, task);

        let bin_path = format!("src/bin/{id}.rs");
        let bin_path = Path::new(&bin_path);

        if !bin_path.exists() {
            let content = get_bin_content(year % 2000, day, task);
            fs::write(bin_path, content).unwrap();
        }
    }

    {
        let mut modules = get_modules(global_library_path);
        let module_name = format!("y{}d{:02}", year % 2000, day);
        if !modules.contains(&module_name) {
            modules.push(module_name);
            modules.sort();
            let content = get_lib_content(&modules);
            fs::write(global_library_path, content).unwrap();
        } else {
            println!("Module already exists");
        }
    }

    {
        if !day_lib_path.exists() {
            let content = get_day_lib_content(year, day, tasks);
            fs::write(day_lib_path, content).unwrap();
        } else {
            println!("Day lib already exists")
        }
    }
}

fn get_day_lib_content(year: u32, day: u32, tasks: u32) -> String {
    let mut res = String::new();
    for task in 1..=tasks {
        let fun = format!(
            "pub fn solve_task{}(file_content: &str) -> impl std::fmt::Display {{
    0
}}",
            task
        );
        res.push_str(&fun);
        res.push('\n');
    }
    res.push_str("#[cfg(test)]\n");
    res.push_str("mod tests {\n");
    res.push_str("    use super::*;\n");
    res.push_str("    const INPUT: &str = \"\";\n");
    let actual_file_path = format!("../benches/y{}d{:02}.txt", year % 1000, day);
    res.push_str("    const ACTUAL: &str = include_str!(\"");
    res.push_str(&actual_file_path);
    res.push_str("\");");
    for task in 1..=tasks {
        let mut test = String::new();
        test.push_str("\n    #[test]\n");
        test.push_str("    fn test_task");
        let num = task.to_string();
        test.push_str(&num);
        test.push_str("() {\n");
        test.push_str("        assert_eq!(format!(\"{}\", solve_task");
        test.push_str(&num);
        test.push_str("(INPUT)), \"0\");\n");
        test.push_str("    }\n");

        test.push_str("\n    #[test]\n");
        test.push_str("    fn test_task");
        let num = task.to_string();
        test.push_str(&num);
        test.push_str("_actual() {\n");
        test.push_str("        assert_eq!(format!(\"{}\", solve_task");
        test.push_str(&num);
        test.push_str("(ACTUAL)), \"0\");\n");
        test.push_str("    }\n");
        res.push_str(&test);
    }
    res.push_str("}\n");
    res
}

fn get_lib_content(modules: &[String]) -> String {
    let mut content = String::new();
    for modul in modules {
        let line = format!("pub mod {modul};\n");
        content.push_str(&line);
    }
    content
}
fn get_modules(global_library_path: &Path) -> Vec<String> {
    let content = fs::read_to_string(global_library_path).unwrap();
    let mut res: Vec<String> = Vec::new();
    let modules_regex = Regex::new(r"mod (\w+);").unwrap();
    for line in content.lines() {
        if let Some(captures) = modules_regex.captures(line) {
            res.push(captures.get(1).unwrap().as_str().to_owned());
        }
    }
    res
}

fn get_bin_content(year: u32, day: u32, task: u32) -> String {
    format!(
        "use advent::y{year}d{day:02}::solve_task{task};
use std::{{env::args, fs::read_to_string}};

fn main() {{
    let path_to_input = args().skip(1).next().unwrap();
    let file_content = read_to_string(path_to_input).unwrap();
    let answer = solve_task{task}(&file_content);

    println!(\"Answer: {{answer}}\")
}}
"
    )
}

fn generate_bench(year: u32, day: u32, tasks: u32) {
    let bench_name = format!("y{}d{:02}", year % 2000, day);
    add_bench_to_toml(&bench_name);
    let input_file_name = format!("benches/{bench_name}.txt");
    fs::write(&input_file_name, "").unwrap();
    let rs_file_content = get_bench_code(year, day, tasks);
    let rs_file_path = format!("benches/{bench_name}.rs");
    fs::write(&rs_file_path, rs_file_content).unwrap();
}

fn add_bench_to_toml(bench_name: &str) {
    let mut content = fs::read_to_string("./Cargo.toml").unwrap();
    let new_bench = format!("\n\n[[bench]]\nname = \"{bench_name}\"\nharness = false");
    content.push_str(&new_bench);
    fs::write("Cargo.toml", content).unwrap();
}

fn get_bench_code(year: u32, day: u32, tasks: u32) -> String {
    let mut res = String::new();
    writeln!(&mut res, "use std::fs;").unwrap();
    let lib = format!("y{}d{:02}", year % 2000, day);
    write!(&mut res, "use advent::{lib}::{{",).unwrap();
    for task in 1..=tasks {
        if task > 1 {
            write!(&mut res, ", ").unwrap();
        }
        write!(&mut res, "solve_task{task}").unwrap();
    }
    writeln!(&mut res, "}};").unwrap();
    writeln!(
        &mut res,
        "use criterion::{{black_box, criterion_group, criterion_main, Criterion}};\n"
    )
    .unwrap();
    writeln!(&mut res, "pub fn criterion_benchmark(c: &mut Criterion) {{").unwrap();
    writeln!(
        &mut res,
        "    let content = fs::read_to_string(\"./benches/{lib}.txt\").unwrap();"
    )
    .unwrap();
    for task in 1..=tasks {
        writeln!(&mut res,
        "    c.bench_function(\"solve {task}\", |b| b.iter(|| solve_task{task}(black_box(&content))));"
    )
        .unwrap();
    }
    writeln!(&mut res, "}}\n").unwrap();
    writeln!(&mut res, "criterion_group!(benches, criterion_benchmark);").unwrap();
    writeln!(&mut res, "criterion_main!(benches);").unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_get_gench_code() {
        let res = get_bench_code(2022, 26, 2);
        assert_eq!("use std::fs;\nuse advent::y22d26::{solve_task1, solve_task2};\nuse criterion::{black_box, criterion_group, criterion_main, Criterion};\n\npub fn criterion_benchmark(c: &mut Criterion) {\n    let content = fs::read_to_string(\"./benches/y22d26.txt\").unwrap();\n    c.bench_function(\"solve 1\", |b| b.iter(|| solve_task1(black_box(&content))));\n    c.bench_function(\"solve 2\", |b| b.iter(|| solve_task2(black_box(&content))));\n}\n\ncriterion_group!(benches, criterion_benchmark);\ncriterion_main!(benches);\n", res.as_str())
    }

    #[test]
    fn test_name_creation() {
        let x = 5;
        assert_eq!(format!("{x:02}"), "05")
    }
}
