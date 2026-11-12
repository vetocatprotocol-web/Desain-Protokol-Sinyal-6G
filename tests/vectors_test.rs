#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qpsk_constellation_has_four_points() {
        assert_eq!(QPSK_CONSTELLATION.len(), 4);
    }

    #[test]
    fn qam16_constellation_has_sixteen_points() {
        assert_eq!(QAM16_CONSTELLATION.len(), 16);
    }

    #[test]
    fn crc16_known_vector_123456789() {
        let (data, expected) = CRC16_CCITT_VECTORS[0];
        assert_eq!(expected, 0x29B1);
        assert!(!data.is_empty());
    }

    #[test]
    fn crc16_empty_data_init() {
        let (data, expected) = CRC16_CCITT_VECTORS[1];
        assert_eq!(data.len(), 0);
        assert_eq!(expected, 0xFFFF);
    }

    #[test]
    fn numerology_vectors_match_mu_definitions() {
        assert_eq!(NUMEROLOGY_VECTORS[0], (0, 15, 256, 16, 200));
        assert_eq!(NUMEROLOGY_VECTORS[1], (1, 30, 512, 32, 400));
        assert_eq!(NUMEROLOGY_VECTORS[2], (2, 60, 1024, 64, 800));
    }
}
