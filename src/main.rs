use clap::{command, Parser, Subcommand};

mod db;
mod model;
mod remote;
mod server;

#[derive(Parser)]
#[command(author, version, about)]
struct CmdLet {
    #[arg(default_value_t = 3000)]
    port: u16,
    #[command(subcommand)]
    task: Task,
}

#[derive(Subcommand)]
enum Task {
    Insert,
    Pull { src: Option<String> },
    Send,
    Serve,
}

#[tokio::main]
async fn main() {
    let args = CmdLet::parse();
    match &args.task {
        Task::Insert => todo!(),
        Task::Pull { src } => {
            if src.is_some() {
                todo!()
            }
            let res = reqwest::get(format!("http://127.0.0.1:{}/cmdlet/pull", args.port))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{}", res);
        }
        Task::Send => todo!(),
        Task::Serve => server::serve(args.port).await,
    }
}
