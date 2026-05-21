use wincode::{SchemaWrite, SchemaRead};
use wincode::config::ConfigCore;
use wincode::error::{ReadError, ReadResult, WriteResult};
use wincode::io::{Reader, Writer};
use std::mem::MaybeUninit;
use solana_address::Address;

pub const I_AGENT_ACCOUNT_DISCRIMINATOR: &[u8] = &[4];

#[derive(Clone, Copy)]
pub struct IAgent {
    pub agent_id: Address,
    pub bump: u8,
    pub seeds: [u8; 37],
}

unsafe impl<C: ConfigCore> SchemaWrite<C> for IAgent
where
    Address: SchemaWrite<C, Src = Address>,
    [u8; 37]: SchemaWrite<C, Src = [u8; 37]>,
    u8: SchemaWrite<C, Src = u8>,
{
    type Src = Self;

    fn size_of(src: &Self) -> WriteResult<usize> {
        Ok(1
            + <Address as SchemaWrite<C>>::size_of(&src.agent_id)?
            + <u8 as SchemaWrite<C>>::size_of(&src.bump)?
            + <[u8; 37] as SchemaWrite<C>>::size_of(&src.seeds)?)
    }

    fn write(mut writer: impl Writer, src: &Self) -> WriteResult<()> {
        writer.write(I_AGENT_ACCOUNT_DISCRIMINATOR)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.agent_id)?;
        <u8 as SchemaWrite<C>>::write(writer.by_ref(), &src.bump)?;
        <[u8; 37] as SchemaWrite<C>>::write(writer.by_ref(), &src.seeds)?;
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for IAgent
where
    Address: SchemaRead<'de, C, Dst = Address>,
    [u8; 37]: SchemaRead<'de, C, Dst = [u8; 37]>,
    u8: SchemaRead<'de, C, Dst = u8>,
{
    type Dst = Self;

    fn read(mut reader: impl Reader<'de>, dst: &mut MaybeUninit<Self>) -> ReadResult<()> {
        let disc = reader.take_byte()?;
        if disc != 4 {
            return Err(ReadError::InvalidValue("invalid account discriminator"));
        }
        dst.write(Self {
            agent_id: <Address as SchemaRead<'de, C>>::get(reader.by_ref())?,
            bump: <u8 as SchemaRead<'de, C>>::get(reader.by_ref())?,
            seeds: <[u8; 37] as SchemaRead<'de, C>>::get(reader.by_ref())?,
        });
        Ok(())
    }
}

