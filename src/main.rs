use clap::{command, Parser, Subcommand};

mod db;
mod model;
mod remote;
mod server;

#[derive(Parser)]
#[command(author, version, about)]
struct CmdLet{
    #[command(subcommand)]
    task: Task
}

#[derive(Subcommand)]
enum Task{
    Insert,
    Pull{
        src:Option<String>
    },
    Send,
    Serve{ 
        #[arg(default_value_t = 3000)]
        port: u16
    }
}


#[tokio::main]
async fn main() {
    let args = CmdLet::parse();
    match args.task{
        Task::Insert => todo!(),
        Task::Pull { src } => {
            let connection = db::open_db();
            let src = src.unwrap_or("https://contests.sdutacm.cn/contests.json".to_string());
            let data = remote::pull_contest_data(&src).await.unwrap();
            for item in data{
                db::insert_db(&connection, &item).unwrap();
            }
        },
        Task::Send => todo!(),
        Task::Serve { port } => server::serve(port).await,
    }
}
