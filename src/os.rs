use std::fs::File;
use std::io::Write;

pub fn trigger_sleep() -> std::io::Result<()> {
    let mut mem_sleep = File::create("/sys/power/mem_sleep")?;
    mem_sleep.write_all(b"deep")?;

    let mut state = File::create("/sys/power/state")?;
    state.write_all(b"mem")?;

    Ok(())
}
