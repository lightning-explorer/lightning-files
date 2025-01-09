use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watcher_task<F>(dir_path: &Path, on_changes: F) -> Result<()>
where
    F: Fn(),
{
    let (tx, rx) = channel();

    let config = Config::default().with_poll_interval(Duration::from_secs(2));
    let mut watcher = RecommendedWatcher::new(tx, config)?;

    watcher.watch(dir_path, RecursiveMode::NonRecursive)?;

    println!("Watching for changes in {}", dir_path.to_string_lossy());

    for event in rx {
        match event {
            Ok(event) => match event.kind {
                EventKind::Modify(notify::event::ModifyKind::Name(_))
                | EventKind::Create(_)
                | EventKind::Remove(_) => {
                    on_changes();
                }
                _ => {}
            },
            Err(e) => println!("Error watching files: {:?}", e),
        }
    }
    Ok(())
}
