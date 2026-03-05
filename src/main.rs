mod contact;
mod storage;
mod commands;

use clap::{Parser, Subcommand};
use commands::*;
use colored::*;

#[derive(Parser)]
#[command(name = "Contact Book CLI", version = "1.0", author = "Koshi", about = "A simple contact book application")]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    Add{
        #[arg(long)]
        name:String,
        #[arg(long)]
        phone:String,
        #[arg(long)]
        email:Option<String>,
    },
    List,
    Search{
        #[arg(long)]
        name:String,
    },
    Delete{
        #[arg(long)]
        id:u32,
    }
}

fn main() {
    println!("{}", "Welcome to Contact Book CLI!".blue().bold());

    let cli = Cli::parse();

    match cli.command {
        Commands::Add{name, phone, email}=>{
            add_contact(name,phone,email);
        }
        Commands::List =>{
            list_contacts();
        }
        Commands::Search{name}=>{
            search_contacts(name);
        }
        Commands::Delete{id}=>{
            delete_contact(id)
        }

    }
   
}