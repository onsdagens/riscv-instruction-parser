#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    SP = 13,
    LR = 14,
    PC = 15,
}

impl TryFrom<u8> for Register {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            9 => Ok(Register::R9),
            10 => Ok(Register::R10),
            11 => Ok(Register::R11),
            12 => Ok(Register::R12),
            13 => Ok(Register::SP),
            14 => Ok(Register::LR),
            15 => Ok(Register::PC),
            _ => Err("Not a valid register."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_to_Register() {
        assert_eq!(0.try_into(), Ok(Register::R0));
        assert_eq!(1.try_into(), Ok(Register::R1));
        assert_eq!(2.try_into(), Ok(Register::R2));
        assert_eq!(3.try_into(), Ok(Register::R3));
        assert_eq!(4.try_into(), Ok(Register::R4));
        assert_eq!(5.try_into(), Ok(Register::R5));
        assert_eq!(6.try_into(), Ok(Register::R6));
        assert_eq!(7.try_into(), Ok(Register::R7));
        assert_eq!(8.try_into(), Ok(Register::R8));
        assert_eq!(9.try_into(), Ok(Register::R9));
        assert_eq!(10.try_into(), Ok(Register::R10));
        assert_eq!(11.try_into(), Ok(Register::R11));
        assert_eq!(12.try_into(), Ok(Register::R12));
        assert_eq!(13.try_into(), Ok(Register::SP));
        assert_eq!(14.try_into(), Ok(Register::LR));
        assert_eq!(15.try_into(), Ok(Register::PC));
        assert_eq!(
            16.try_into(),
            Err::<Register, &str>("Not a valid register.")
        )
    }
}