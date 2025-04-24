use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(short = 'd', long, default_value_t = 0)]
    days: u64,

    #[arg(short = 'H', long, default_value_t = 0)]
    hours: u64,

    #[arg(short = 'm', long, default_value_t = 0)]
    minutes: u64,

    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    let total_seconds = args.days * 86400 + args.hours * 3600 + args.minutes * 60;
    let max_age = Duration::from_secs(total_seconds);
    let now = SystemTime::now();
    let mut deleted_count = 0;

    let entries: Vec<_> = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    for entry in entries {
        let path = entry.path().to_path_buf();

        if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(age) = now.duration_since(modified) {
                    if age > max_age {
                        if args.dry_run {
                            println!("[dry-run] Would delete: {}", path.display());
                        } else {
                            if metadata.is_dir() {
                                
                                let is_empty = match fs::read_dir(&path) {
                                    Ok(read_dir) => read_dir.count() == 0,
                                    Err(_) => false,
                                };

                                if is_empty {
                                    if let Err(e) = fs::remove_dir(&path) {
                                        eprintln!(" Failed to delete directory {}: {}", path.display(), e);
                                    } else {
                                        println!("Deleted directory: {}", path.display());
                                        deleted_count += 1;
                                    }
                                }
                            } else {
                                if let Err(e) = fs::remove_file(&path) {
                                    eprintln!(" Failed to delete file {}: {}", path.display(), e);
                                } else {
                                    println!("Deleted file: {}", path.display());
                                    deleted_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if args.dry_run {
        println!(" Dry-run complete.");
    } else {
        if deleted_count > 0 {
            println!(" Cleanup done. {} items deleted.", deleted_count);
        } else {
            println!(" No items to delete. ");
        }
    }
}
