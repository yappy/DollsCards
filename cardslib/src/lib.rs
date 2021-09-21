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

    pub fn calc_hand(hand: &[u32]) {
        assert!(hand.len() == 5);

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
    #[should_panic]
    fn encode_invalid_1() {
        cards::encode(0, 13);
    }

    #[test]
    #[should_panic]
    fn encode_invalid_2() {
        cards::encode(0, 13);
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
    fn decode_invalid() {
        cards::decode(cards::CARDS_NUM);
    }

    #[test]
    #[should_panic]
    fn calc_hand_invalid_1() {
        let hand = vec![0u32, 1u32, 2u32, 3u32];
        cards::calc_hand(&hand);
    }

    #[test]
    #[should_panic]
    fn calc_hand_invalid_2() {
        let hand = vec![0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        cards::calc_hand(&hand);
    }
}
