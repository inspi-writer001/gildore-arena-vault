use wincode::{SchemaWrite, SchemaRead};
use wincode::config::ConfigCore;
use wincode::error::{ReadError, ReadResult, WriteResult};
use wincode::io::{Reader, Writer};
use std::mem::MaybeUninit;
use solana_address::Address;
use quasar_lang::client::{DynVec};

pub const GLOBAL_STATE_ACCOUNT_DISCRIMINATOR: &[u8] = &[2];

#[derive(Clone)]
pub struct GlobalState {
    pub fee_destination: Address,
    pub fee_bps: u16,
    pub max_fee: u64,
    pub bump: u8,
    pub admin: DynVec<Address, u16>,
}

unsafe impl<C: ConfigCore> SchemaWrite<C> for GlobalState
where
    Address: SchemaWrite<C, Src = Address>,
    u16: SchemaWrite<C, Src = u16>,
    u64: SchemaWrite<C, Src = u64>,
    u8: SchemaWrite<C, Src = u8>,
{
    type Src = Self;

    fn size_of(src: &Self) -> WriteResult<usize> {
        Ok(1
            + <Address as SchemaWrite<C>>::size_of(&src.fee_destination)?
            + <u16 as SchemaWrite<C>>::size_of(&src.fee_bps)?
            + <u64 as SchemaWrite<C>>::size_of(&src.max_fee)?
            + <u8 as SchemaWrite<C>>::size_of(&src.bump)?
            + 2
            + {
                let mut s = 0usize;
                for item in src.admin.iter() {
                    s += <Address as SchemaWrite<C>>::size_of(item)?;
                }
                s
            })
    }

    fn write(mut writer: impl Writer, src: &Self) -> WriteResult<()> {
        writer.write(GLOBAL_STATE_ACCOUNT_DISCRIMINATOR)?;
        <Address as SchemaWrite<C>>::write(writer.by_ref(), &src.fee_destination)?;
        <u16 as SchemaWrite<C>>::write(writer.by_ref(), &src.fee_bps)?;
        <u64 as SchemaWrite<C>>::write(writer.by_ref(), &src.max_fee)?;
        <u8 as SchemaWrite<C>>::write(writer.by_ref(), &src.bump)?;
        writer.write(&(src.admin.len() as u64).to_le_bytes()[..2])?;
        for item in src.admin.iter() {
            <Address as SchemaWrite<C>>::write(writer.by_ref(), item)?;
        }
        Ok(())
    }
}

unsafe impl<'de, C: ConfigCore> SchemaRead<'de, C> for GlobalState
where
    Address: SchemaRead<'de, C, Dst = Address>,
    u16: SchemaRead<'de, C, Dst = u16>,
    u64: SchemaRead<'de, C, Dst = u64>,
    u8: SchemaRead<'de, C, Dst = u8>,
{
    type Dst = Self;

    fn read(mut reader: impl Reader<'de>, dst: &mut MaybeUninit<Self>) -> ReadResult<()> {
        let disc = reader.take_byte()?;
        if disc != 2 {
            return Err(ReadError::InvalidValue("invalid account discriminator"));
        }
        let fee_destination = <Address as SchemaRead<'de, C>>::get(reader.by_ref())?;
        let fee_bps = <u16 as SchemaRead<'de, C>>::get(reader.by_ref())?;
        let max_fee = <u64 as SchemaRead<'de, C>>::get(reader.by_ref())?;
        let bump = <u8 as SchemaRead<'de, C>>::get(reader.by_ref())?;
        let admin_len = {
            let mut buf = [0u8; 8];
            let pfx_bytes = reader.take_scoped(2)?;
            buf[..2].copy_from_slice(pfx_bytes);
            u64::from_le_bytes(buf) as usize
        };
        let admin: DynVec<Address, u16> = {
            let mut items = Vec::with_capacity(admin_len);
            for _ in 0..admin_len {
                items.push(<Address as SchemaRead<'de, C>>::get(reader.by_ref())?);
            }
            items.into()
        };
        dst.write(Self {
            fee_destination,
            fee_bps,
            max_fee,
            bump,
            admin,
        });
        Ok(())
    }
}

