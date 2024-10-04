/// Convert a u32 array to a u8 array. Useful for converting the range vkey to a B256.
pub fn u32_to_u8(input: [u32; 8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    for (i, &value) in input.iter().enumerate() {
        let bytes = value.to_be_bytes();
        output[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_to_u8() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8];
        let output = u32_to_u8(input);
        assert_eq!(
            output,
            [
                0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7,
                0, 0, 0, 8
            ]
        );
    }
}
