
// today we are maintaining global mutable state

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref RINGS_OF_POWER: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(ring: &str) -> Result<(), &'static str> 
{
    let mut db = RINGS_OF_POWER.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(ring.to_string());
    Ok(())
}

fn main() -> Result<(), &'static str> {
    insert("Narya")?;
    insert("Nenya")?;
    insert("Vilya")?;
    {
        let db = RINGS_OF_POWER.lock().map_err(|_| "Failed to acquire MutexGuard")?;

        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("The One")?;
    Ok(())
}