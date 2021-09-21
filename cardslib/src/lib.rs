#[allow(dead_code)]
mod cards{
    pub const SUIT_NUM: u32 = 4;
    pub const NUMBER_NUM: u32 = 13;
    pub const CARDS_NUM: u32 = SUIT_NUM * NUMBER_NUM;

    #[derive(Debug, PartialEq, Eq)]
    pub enum Rank {
        High,
        OnePair,
        TwoPair,
        Trips,
        Straight,
        Flash,
        FullHouse,
        Quads,
        StraightFlash,
    }

    // (suit, number) => u32
    pub fn encode(suit: u32, number: u32) -> u32 {
        assert!(suit < SUIT_NUM && number < NUMBER_NUM);

        suit * NUMBER_NUM + number
    }

    // u32 => (suit, number)
    pub fn decode(code: u32) -> (u32, u32) {
        assert!(code < CARDS_NUM);

        (code / NUMBER_NUM, code % NUMBER_NUM)
    }

    // [u32; 5] => (Rank, kicker)
    pub fn calc_hand(hand: &[u32]) -> (Rank, u32) {
        assert!(hand.len() == 5);

        // suit and number backet
        let mut sb = [0; SUIT_NUM as usize];
        let mut nb = [0; NUMBER_NUM as usize];
        for code in hand {
            let (s, n) = decode(*code);
            sb[s as usize] += 1;
            nb[n as usize] += 1;
        }

        if let Ok(kicker) = find_quads(&nb) {
            return (Rank::Quads, kicker)
        }

        // TODO: kicker
        (Rank::High, 0)
    }

    // number list (len <= 5) => single integer (<= 20 bit)
    // num_list[0] is the most significant.
    fn create_kicker(num_list: &[u32]) -> u32 {
        assert!(num_list.len() <= 5);

        let mut value = 0u32;
        for num in num_list {
            assert!(*num < NUMBER_NUM);
            value <<= 4;
            value |= *num;
        }

        value
    }

    fn find_quads(nb: &[i32]) -> Result<u32, ()> {
        assert!(nb.len() == NUMBER_NUM as usize);

        let mut found = -1;
        let mut kicker = -1;
        for i in (0..nb.len()).rev() {
            if nb[i] == 4 {
                assert!(found == -1);
                found = i as i32;
            }
            if nb[i] == 1 {
                assert!(kicker == -1);
                kicker = i as i32;
            }
        }

        match found {
            -1 => Err(()),
            _ => Ok(create_kicker(&[found as u32, kicker as u32]))
        }
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
        let hand = vec![0u32; 4];
        cards::calc_hand(&hand);
    }

    #[test]
    #[should_panic]
    fn calc_hand_invalid_2() {
        let hand = vec![0u32; 6];
        cards::calc_hand(&hand);
    }
}
