use std::io::{stdout, Write};

use anyhow::{Context, Result};
use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

pub fn mop_stdout() -> Result<()> {
    stdout()
        .queue(cursor::Hide)
        .context("Failure to hide cursor")?;
    stdout()
        .queue(Clear(ClearType::All))
        .context("Failure to clear output")?;
    stdout()
        .queue(cursor::MoveTo(0, 0))
        .context("Failure to move cursor to (0, 0)")?;
    stdout()
        .queue(cursor::Show)
        .context("Failure to show cursor")?;

    stdout().flush().context("Failure to flush output buffer")?;

    Ok(())
}

pub fn mop_and_purge_stdout() -> Result<()> {
    stdout()
        .queue(cursor::Hide)
        .context("Failure to hide cursor")?;
    stdout()
        .queue(Clear(ClearType::Purge))
        .context("Failure to clear and purge output")?;
    stdout()
        .queue(cursor::MoveTo(0, 0))
        .context("Failure to move cursor to (0, 0)")?;
    stdout()
        .queue(cursor::Show)
        .context("Failure to show cursor")?;

    stdout().flush().context("Failure to flush output buffer")?;

    Ok(())
}
