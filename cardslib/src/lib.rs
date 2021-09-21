mod cards{
    pub const SUIT_NUM: u32 = 4;
    pub const NUMBER_NUM: u32 = 13;
    pub const CARDS_NUM: u32 = SUIT_NUM * NUMBER_NUM;

    pub fn encode(suit: u32, number: u32) -> u32 {
        assert!(suit < SUIT_NUM && number < NUMBER_NUM);
        suit * NUMBER_NUM + number
    }
    pub fn decode(code: u32) -> (u32, u32) {
        assert!(code < CARDS_NUM);
        (code / NUMBER_NUM, code % NUMBER_NUM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let mut expected = 0u32;
        for s in 0..4 {
            for n in 0..13 {
                let enc = cards::encode(s, n);
                assert_eq!(enc, expected);
                expected += 1;
            }
        }
        assert_eq!(expected, cards::CARDS_NUM);
    }

    #[test]
    fn decode() {
        let mut code = 0u32;
        for s in 0..4 {
            for n in 0..13 {
                let (suit, number) = cards::decode(code);
                assert_eq!(suit, s);
                assert_eq!(number, n);
                code += 1;
            }
        }
    }

    #[test]
    #[should_panic]
    fn invalid_encode1() {
        cards::encode(0, 13);
    }

    #[test]
    #[should_panic]
    fn invalid_encode2() {
        cards::encode(0, 13);
    }
}
