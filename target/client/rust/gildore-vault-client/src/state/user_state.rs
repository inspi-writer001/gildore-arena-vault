use wincode::{SchemaWrite, SchemaRead};
use wincode::config::ConfigCore;
use wincode::error::{ReadError, ReadResult, WriteResult};
use wincode::io::{Reader, Writer};
use std::mem::MaybeUninit;
use solana_address::Address;

pub const USER_STATE_ACCOUNT_DISCRIMINATOR: &[u8] = &[1];

#[derive(Clone, Copy)]
pub struct UserState {
    pub user_address: Address,
    pub agent_id: Address,
    pub ticker_id: Address,
    pub is_initialized: PodBool,
    pub modified_time: u64,
    pub created_time: u64,
    pub amount: u64,
    pub bump: u8,
}

unsafe impl<C: ConfigCore> SchemaWrite<C> for UserState
where
    Address: SchemaWrite<C, Src = Address>,
    PodBool: SchemaWrite<C, Src = PodBool>,
    u64: SchemaWrite<C, Src = u64>,
    u8: SchemaWrite<C, Src = u8>,
{
    type Src = Self;

    fn size_of(src: &Self) -> WriteResult<usize> {
        Ok(1
            + <Address as SchemaWrite<C>>::size_of(&src.user_address)?
            + <Address as SchemaWrite<C>>::size_of(&src.agent_id)?
            + <Address as SchemaWrite<C>>::size_of(&src.ticker_id)?
            + <PodBool as SchemaWrite<C>>::size_of(&src.is_initialized)?
            + <u64 as SchemaWrite<C>>::size_of(&src.modified_time)?
            + <u64 as SchemaWrite<C>>::size_of(&src.created_time)?
            + <u64 as SchemaWrite<C>>::size_of(&src.amount)?
            + <u8 as SchemaWrite<C>>::size_of(&src.bump)?)
    }

    fn write(mut writer: impl Writer, src: &Self) -> WriteResult<()> {
        writer.write(USER_STATE_ACCOUNT_DISCRIMINATOR)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.user_address)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.agent_id)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.ticker_id)?;
        <PodBool as SchemaWrite<C>>::write(writer.by_ref(), &src.is_initialized)?;
        <u64 as SchemaWrite<C>>::write(writer.by_ref(), &src.modified_time)?;
        <u64 as SchemaWrite<C>>::write(writer.by_ref(), &src.created_time)?;
        <u64 as SchemaWrite<C>>::write(writer.by_ref(), &src.amount)?;
        <u8 as SchemaWrite<C>>::write(writer.by_ref(), &src.bump)?;
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for UserState
where
    Address: SchemaRead<'de, C, Dst = Address>,
    PodBool: SchemaRead<'de, C, Dst = PodBool>,
    u64: SchemaRead<'de, C, Dst = u64>,
    u8: SchemaRead<'de, C, Dst = u8>,
{
    type Dst = Self;

    fn read(mut reader: impl Reader<'de>, dst: &mut MaybeUninit<Self>) -> ReadResult<()> {
        let disc = reader.take_byte()?;
        if disc != 1 {
            return Err(ReadError::InvalidValue("invalid account discriminator"));
        }
        dst.write(Self {
            user_address: <Address as SchemaRead<'de, C>>::get(reader.by_ref())?,
            agent_id: <Address as SchemaRead<'de, C>>::get(reader.by_ref())?,
            ticker_id: <Address as SchemaRead<'de, C>>::get(reader.by_ref())?,
            is_initialized: <PodBool as SchemaRead<'de, C>>::get(reader.by_ref())?,
            modified_time: <u64 as SchemaRead<'de, C>>::get(reader.by_ref())?,
            created_time: <u64 as SchemaRead<'de, C>>::get(reader.by_ref())?,
            amount: <u64 as SchemaRead<'de, C>>::get(reader.by_ref())?,
            bump: <u8 as SchemaRead<'de, C>>::get(reader.by_ref())?,
        });
        Ok(())
    }
}

