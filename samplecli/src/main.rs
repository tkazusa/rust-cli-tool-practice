use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            // `-v`オプションが指定されている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}

// モジュール、クレート、要素に対するメタデータ。
// derive マクロ を使って継承
// Opts 構造体を修飾。Clap トレイト、Debug トレイトを実装するコードを生成
#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Taketoshi Kazusa",
    about = "Super awsome sample RPN calculator"
)]
// C 言語スタイルの構造体。
struct Opts {
    // Sets the level of verbosity
    // short は短縮形、long は正式に書くと
    #[clap(short, long)]
    verbose: bool,
    // Formulas written in RPN
    // Option型 値が無いことを示す None と、あることを示す Some(T) を取る。(ここで T は型を指定)
    formula_file: Option<String>,
}

fn main() {
    // コマンドライン引数をパースしている
    let opts = Opts::parse();

    // match 演算子 値が該当するかどうかで分岐
    match &opts.formula_file {
        // opts に formula_file が指定されていれば、その値を撮って出力
        Some(path) => println!("File specified: {}", path),
        // 無ければ、ファイル指定なしを出力
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // ファイルを指定しなかった場合
        println!("No file is specified");
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line)
    }
}