use clap::{Subcommand, Parser};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command
}
#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作る
    New,
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSVからインポート
    Import,
    /// レポートを出力
    Report,
}

fn main() {
    let _args = App::parse();
}
