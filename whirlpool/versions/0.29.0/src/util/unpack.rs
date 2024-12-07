use anchor_lang::prelude::*;
use bincode;
use default_boxed::DefaultBoxed;

use crate::state::{TickArray, Tick, TICK_ARRAY_SIZE_USIZE, NUM_REWARDS};

#[derive(Debug, PartialEq, Clone, Copy, DefaultBoxed)]
pub struct UnpackedTickArray {
  pub start_tick_index: i32,
  pub ticks: [UnpackedTick; TICK_ARRAY_SIZE_USIZE],
  pub whirlpool: Pubkey,
}

#[derive(borsh::BorshDeserialize, Debug, PartialEq, Default, Clone, Copy)]
pub struct UnpackedTick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; NUM_REWARDS],
}

pub fn unpack_tick_array(loader: &AccountLoader<TickArray>) -> std::result::Result<Box<UnpackedTickArray>, Error> {
  try_deserialize_tick_array(
    loader.to_account_info().owner,
    &mut loader.to_account_info().data.borrow().as_ref(),
  )
}

fn try_deserialize_tick_array(owner: &Pubkey, buf: &mut &[u8]) -> std::result::Result<Box<UnpackedTickArray>, Error> {
  if !owner.eq(&crate::ID) {
    return Err(anchor_lang::error::ErrorCode::ConstraintOwner.into());
  }

  let mut cursor = 0;

  let discriminator: u64 = bincode::deserialize(&buf[cursor..cursor+8]).unwrap();
  cursor += 8;

  // DISCRIMINATOR: 45 61 bd be 6e 07 42 bb
  if discriminator != 0xbb42076ebebd6145 /* LE u64 */ {
    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into());
  }

  let mut tickarray = UnpackedTickArray::default_boxed();

  tickarray.start_tick_index = bincode::deserialize(&buf[cursor..cursor+4]).unwrap();
  cursor += 4;

  for i in 0..TICK_ARRAY_SIZE_USIZE {
    tickarray.ticks[i] = UnpackedTick::deserialize(&mut &buf[cursor..cursor+Tick::LEN]).unwrap();
    cursor += Tick::LEN;
  }

  tickarray.whirlpool = Pubkey::deserialize(&mut &buf[cursor..cursor+32]).unwrap();

  Ok(tickarray)
}
