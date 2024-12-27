#![allow(dead_code)]

pub(crate) mod charset;
pub(crate) mod collation;
mod config;
mod decode_value;
mod encode_value;
mod executor;
mod executor_buffer;
mod mysql_error;
mod record;
mod records;
mod transaction_manager;
mod ty;
mod tys;

use crate::database::{Database, DatabaseTy};
pub use config::Config;
use core::marker::PhantomData;
pub use decode_value::DecodeValue;
pub use encode_value::EncodeValue;
pub use executor::Executor;
pub use executor_buffer::ExecutorBuffer;
pub use mysql_error::MysqlError;
pub use record::Record;
pub use records::Records;
pub use transaction_manager::TransactionManager;
pub use ty::Ty;

/// MySQL
#[derive(Debug)]
pub struct Mysql<E>(PhantomData<fn() -> E>);

impl<E> Database for Mysql<E>
where
  E: From<crate::Error>,
{
  const TY: DatabaseTy = DatabaseTy::Mysql;

  type DecodeValue<'exec> = DecodeValue<'exec>;
  type EncodeValue<'buffer, 'tmp>
    = EncodeValue<'buffer, 'tmp>
  where
    'buffer: 'tmp;
  type Error = E;
  type Record<'exec> = Record<'exec, E>;
  type Records<'exec> = Records<'exec, E>;
  type Ty = Ty;
}

impl<E> Default for Mysql<E> {
  #[inline]
  fn default() -> Self {
    Self(PhantomData)
  }
}

// TODOOOOOOOOOOOOOOOOOOOOOOOOOOOOO
// Unificar os comandos clear e table_names
// Tentar remover a duplicacao das implemetnatcoes de tipos
// Tentar remover duplicacao do dbMigration
// FromRecord esta duplicado
// VErificar a implementacao do chono
