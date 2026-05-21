use wincode::{SchemaWrite, SchemaRead};
use wincode::config::ConfigCore;
use wincode::error::{ReadError, ReadResult, WriteResult};
use wincode::io::{Reader, Writer};
use std::mem::MaybeUninit;
use solana_address::Address;

pub const AGENT_MARKET_ACCOUNT_DISCRIMINATOR: &[u8] = &[3];

#[derive(Clone, Copy)]
pub struct AgentMarket {
    pub agent_id: Address,
    pub ticker_id: Address,
    pub is_trading: bool,
    pub bump: u8,
}

unsafe impl<C: ConfigCore> SchemaWrite<C> for AgentMarket
where
    Address: SchemaWrite<C, Src = Address>,
    bool: SchemaWrite<C, Src = bool>,
    u8: SchemaWrite<C, Src = u8>,
{
    type Src = Self;

    fn size_of(src: &Self) -> WriteResult<usize> {
        Ok(1
            + <Address as SchemaWrite<C>>::size_of(&src.agent_id)?
            + <Address as SchemaWrite<C>>::size_of(&src.ticker_id)?
            + <bool as SchemaWrite<C>>::size_of(&src.is_trading)?
            + <u8 as SchemaWrite<C>>::size_of(&src.bump)?)
    }

    fn write(mut writer: impl Writer, src: &Self) -> WriteResult<()> {
        writer.write(AGENT_MARKET_ACCOUNT_DISCRIMINATOR)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.agent_id)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.ticker_id)?;
        <bool as SchemaWrite<C>>::write(writer.by_ref(), &src.is_trading)?;
        <u8 as SchemaWrite<C>>::write(writer.by_ref(), &src.bump)?;
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for AgentMarket
where
    Address: SchemaRead<'de, C, Dst = Address>,
    bool: SchemaRead<'de, C, Dst = bool>,
    u8: SchemaRead<'de, C, Dst = u8>,
{
    type Dst = Self;

    fn read(mut reader: impl Reader<'de>, dst: &mut MaybeUninit<Self>) -> ReadResult<()> {
        let disc = reader.take_byte()?;
        if disc != 3 {
            return Err(ReadError::InvalidValue("invalid account discriminator"));
        }
        dst.write(Self {
            agent_id: <Address as SchemaRead<'de, C>>::get(reader.by_ref())?,
            ticker_id: <Address as SchemaRead<'de, C>>::get(reader.by_ref())?,
            is_trading: <bool as SchemaRead<'de, C>>::get(reader.by_ref())?,
            bump: <u8 as SchemaRead<'de, C>>::get(reader.by_ref())?,
        });
        Ok(())
    }
}

