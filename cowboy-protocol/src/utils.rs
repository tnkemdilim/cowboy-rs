#[macro_export]
macro_rules! cmd {
    ($command: expr, $value: expr) => {
        crate::utils::packetize(&crate::utils::write_value($command, $value))
    };
    ($command: expr) => {
        crate::utils::packetize(&$command)
    };
}

#[macro_export]
macro_rules! bounded {
    ($value: expr, $max: expr) => {
        bounded!($value, 0, $max)
    };
    ($value: expr, $min: expr, $max: expr) => {
        if $value > $max {
            return Err(FieldError::InvalidRange {
                start: $min,
                end: $max,
            });
        }
    };
}

/// Write value to command.
pub fn write_value(
    mut cmd: [u8; 9],
    value: u16,
) -> [u8; 9] {
    cmd[7] = (value >> 8 & 0xff) as u8;
    cmd[8] = (value & 0xff) as u8;
    cmd
}

/// Calculate checksum for command using a modified CRC-16-CCITT
/// algorithm gotten directly from the uncompiled Cowboy app.
pub(super) fn checksum(msg: &[u8; 9]) -> [u8; 2] {
    let mut i2 = u16::MAX;
    for &b in msg {
        i2 ^= b as u16;
        for _ in 0..8 {
            let i4 = i2 & 1;
            i2 >>= 1;
            if i4 != 0 {
                i2 ^= 0xA001;
            }
        }
    }
    [i2 as u8, (i2 >> 8) as u8]
}

/// Packetize the command to be ready for transmission.
pub(super) fn packetize(cmd: &[u8; 9]) -> [u8; 11] {
    let checksum = checksum(cmd);

    [
        cmd[0],
        cmd[1],
        cmd[2],
        cmd[3],
        cmd[4],
        cmd[5],
        cmd[6],
        cmd[7],
        cmd[8],
        checksum[0],
        checksum[1],
    ]
}

#[cfg(test)]
pub(crate) mod test {
    #[macro_export]
    macro_rules! chk {
        ($cmd: expr) => {{
            let b: [u8; 11] = $cmd.try_into().unwrap();
            [b[9], b[10]]
        }};
    }

    #[macro_export]
    macro_rules! che {
        ($cmd: expr) => {{
            let a: Result<[u8; 11], _> = $cmd.try_into();
            a.err().unwrap()
        }};
    }

    #[macro_export]
    macro_rules! assert_all {
        ($cases: expr) => {
            for (actual, expected) in $cases {
                assert_eq!(actual, expected);
            }
        };
    }
}
