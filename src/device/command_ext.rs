use crate::command::*;

use super::*;

pub trait CommandExtSync: SyncDevice {
    fn execute_command<C, R, E>(&mut self, cmd: &C) -> Result<R, E>
    where
        C: SyncCommand<Ret = R, Err = E>,
    {
        cmd.execute(self)
    }

    fn execute_commands<C, E>(&mut self, cmd: &[C]) -> Result<(), E>
    where
        C: SyncCommand<Err = E>,
    {
        for c in cmd {
            c.execute(self)?;
        }
        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub trait CommandExtAsync: AsyncDevice {
    async fn execute_command_async<C, R, E>(&mut self, cmd: &C) -> Result<R, E>
    where
        C: AsyncCommand<Ret = R, Err = E>,
    {
        cmd.execute_async(self).await
    }

    async fn execute_commands_async<C, E>(&mut self, cmd: &[C]) -> Result<(), E>
    where
        C: AsyncCommand<Err = E>,
    {
        for c in cmd {
            c.execute_async(self).await?;
        }
        Ok(())
    }
}

impl<D: SyncDevice> CommandExtSync for D {}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl<D: AsyncDevice> CommandExtAsync for D {}
