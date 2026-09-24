use sqlx::{
    mysql::{MySqlTypeInfo, MySqlValueRef},
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    MySql, Type,
};
use std::io::Write;
use crate::Float;

impl Type<MySql> for Float {
    fn type_info() -> MySqlTypeInfo {
        <String as Type<MySql>>::type_info()
    }
}

impl<'q> Encode<'q, MySql> for Float {
    fn encode_by_ref(&self, buf: &mut <MySql as sqlx::Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        let mut stack_buf = [0u8; 32];
        let mut cursor = std::io::Cursor::new(&mut stack_buf[..]);

        if self.scale <= 0 {
            write!(cursor, "{}", self.value)?;
        } else {
            let factor = 10i64.pow(self.scale as u32);
            let int_part = self.value / factor;
            let frac_part = (self.value % factor).abs();

            write!(
                cursor,
                "{}.{:0width$}",
                int_part,
                frac_part,
                width = self.scale as usize
            )?;
        }

        let len = cursor.position() as usize;
        let s = std::str::from_utf8(&stack_buf[..len])?;

        <String as Encode<MySql>>::encode_by_ref(&s.to_string(), buf)
    }
}

impl<'r> Decode<'r, MySql> for Float {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <&str as Decode<MySql>>::decode(value)?;

        Self::from_str(s)
            .ok_or_else(|| format!("Failed to parse Float from MySQL decimal string: {}", s).into())
    }
}
