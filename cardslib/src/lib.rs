#[allow(dead_code)]
mod cards{
    // A hand is 5-cards-set
    pub const HAND_SIZE: usize = 5;
    // 4 suits
    pub const SUIT_NUM: u32 = 4;
    // 13 numbers, higher code means higher card
    // 0: 2
    // 1: 3
    // ...
    // 10: Q
    // 11: K
    // 12: A
    pub const NUMBER_NUM: u32 = 13;
    // 52 cards
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

    // [u32; 5] => (Rank, order)
    pub fn calc_hand(hand: &[u32]) -> (Rank, u32) {
        assert!(hand.len() == HAND_SIZE);

        // suit and number backet
        let mut sb = [0; SUIT_NUM as usize];
        let mut nb = [0; NUMBER_NUM as usize];
        // number list (descending order)
        let mut nl_sorted = [0u32; HAND_SIZE];
        for (i, &code) in hand.iter().enumerate() {
            let (s, n) = decode(code);
            sb[s as usize] += 1;
            nb[n as usize] += 1;
            nl_sorted[i] = n;
        }
        nl_sorted.sort_unstable();
        nl_sorted.reverse();

        let flash = find_flash(&sb, &nl_sorted);
        let straight = find_straight(&nl_sorted);

        if let Some(order) = straight {
            if let Some(_) = flash {
                // use straight order
                return (Rank::StraightFlash, order)
            }
        }
        if let Some(order) = find_quads(&nb) {
            return (Rank::Quads, order)
        }
        if let Some(order) = find_fullhouse(&nb) {
            return (Rank::FullHouse, order)
        }
        if let Some(order) = flash {
            return (Rank::Flash, order)
        }
        if let Some(order) = straight {
            return (Rank::Straight, order)
        }

        // TODO: order
        (Rank::High, 0)
    }

    // number list (len <= 5) => single integer (<= 20 bit)
    // num_list[0] is the most significant.
    fn create_order(num_list: &[u32]) -> u32 {
        assert!(num_list.len() <= 5);

        let mut value = 0u32;
        for &num in num_list {
            assert!(num < NUMBER_NUM);
            value <<= 4;
            value |= num;
        }

        value
    }

    fn find_quads(nb: &[i32]) -> Option<u32> {
        assert!(nb.len() == NUMBER_NUM as usize);

        // find idx where nb[idx] == 4 and 1
        let found = nb.iter()
            .position(|&count| count == 4)
            .map(|idx| idx as u32);
        let kicker = nb.iter()
            .position(|&count| count == 1)
            .map(|idx| idx as u32);

        match found {
            None => None,
            Some(num) => Some(create_order(&[num, kicker.unwrap()]))
        }
    }

    fn find_fullhouse(nb: &[i32]) -> Option<u32> {
        assert!(nb.len() == NUMBER_NUM as usize);

        // find idx where nb[idx] == 3 and 2
        let found = nb.iter()
            .position(|&count| count == 3)
            .map(|idx| idx as u32);
        let kicker = nb.iter()
            .position(|&count| count == 2)
            .map(|idx| idx as u32);

        match found {
            None => None,
            Some(num) => Some(create_order(&[num, kicker.unwrap()]))
        }
    }

    fn find_flash(sb: &[i32], nl_sorted: &[u32]) -> Option<u32> {
        assert!(sb.len() == SUIT_NUM as usize);
        assert!(nl_sorted.len() == HAND_SIZE);

        let found = sb.iter().find(|&&count| count == 5);

        found.map(|_| create_order(nl_sorted))
    }

    fn find_straight(nl_sorted: &[u32]) -> Option<u32> {
        assert!(nl_sorted.len() == HAND_SIZE);

        // A, 5, 4, 3, 2
        let special: [u32; 5] = [12, 3, 2, 1, 0];
        if *nl_sorted == special {
            // order is 5 (=3)
            return Some(3)
        }

        // other sequential patterns
        let start = nl_sorted[0];
        let mut ok = true;
        for (i, &num) in nl_sorted.iter().enumerate() {
            let start = start as i32;
            let i: i32 = i as i32;
            let num: i32 = num as i32;
            if num != start - i {
                ok = false;
                break
            }
        }
        if ok {
            // order is the highest number
            return Some(start)
        }

        None
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
    fn calc_hand_quads() {
        let mut hand1 = vec![0u32; 0];
        hand1.push(cards::encode(0, 0));
        hand1.push(cards::encode(1, 0));
        hand1.push(cards::encode(2, 0));
        hand1.push(cards::encode(3, 0));
        hand1.push(cards::encode(0, 1));
        let mut hand2 = vec![0u32; 0];
        hand2.push(cards::encode(0, 12));
        hand2.push(cards::encode(1, 12));
        hand2.push(cards::encode(2, 12));
        hand2.push(cards::encode(3, 12));
        hand2.push(cards::encode(0, 1));
        let mut hand3 = vec![0u32; 0];
        hand3.push(cards::encode(0, 12));
        hand3.push(cards::encode(1, 12));
        hand3.push(cards::encode(2, 12));
        hand3.push(cards::encode(3, 12));
        hand3.push(cards::encode(0, 2));

        let (rank1, order1) = cards::calc_hand(&hand1);
        let (rank2, order2) = cards::calc_hand(&hand2);
        let (rank3, order3) = cards::calc_hand(&hand3);
        assert_eq!(rank1, cards::Rank::Quads);
        assert_eq!(rank2, cards::Rank::Quads);
        assert_eq!(rank3, cards::Rank::Quads);
        assert!(order1 < order2);
        assert!(order2 < order3);
    }

    #[test]
    fn calc_hand_fullhouse() {
        let mut hand1 = vec![0u32; 0];
        hand1.push(cards::encode(0, 0));
        hand1.push(cards::encode(1, 1));
        hand1.push(cards::encode(2, 0));
        hand1.push(cards::encode(3, 1));
        hand1.push(cards::encode(0, 0));
        let mut hand2 = vec![0u32; 0];
        hand2.push(cards::encode(0, 1));
        hand2.push(cards::encode(1, 0));
        hand2.push(cards::encode(2, 1));
        hand2.push(cards::encode(3, 0));
        hand2.push(cards::encode(0, 1));
        let mut hand3 = vec![0u32; 0];
        hand3.push(cards::encode(0, 12));
        hand3.push(cards::encode(1, 12));
        hand3.push(cards::encode(2, 12));
        hand3.push(cards::encode(3, 11));
        hand3.push(cards::encode(0, 11));

        let (rank1, order1) = cards::calc_hand(&hand1);
        let (rank2, order2) = cards::calc_hand(&hand2);
        let (rank3, order3) = cards::calc_hand(&hand3);
        assert_eq!(rank1, cards::Rank::FullHouse);
        assert_eq!(rank2, cards::Rank::FullHouse);
        assert_eq!(rank3, cards::Rank::FullHouse);
        assert!(order1 < order2);
        assert!(order2 < order3);
    }

    #[test]
    fn calc_hand_flash() {
        let mut hand1 = vec![0u32; 0];
        hand1.push(cards::encode(0, 12));
        hand1.push(cards::encode(0, 0));
        hand1.push(cards::encode(0, 3));
        hand1.push(cards::encode(0, 5));
        hand1.push(cards::encode(0, 10));
        let mut hand2 = vec![0u32; 0];
        hand2.push(cards::encode(1, 12));
        hand2.push(cards::encode(1, 1));
        hand2.push(cards::encode(1, 3));
        hand2.push(cards::encode(1, 5));
        hand2.push(cards::encode(1, 10));
        let mut hand3 = vec![0u32; 0];
        hand3.push(cards::encode(2, 12));
        hand3.push(cards::encode(2, 11));
        hand3.push(cards::encode(2, 10));
        hand3.push(cards::encode(2, 9));
        hand3.push(cards::encode(2, 7));

        let (rank1, order1) = cards::calc_hand(&hand1);
        let (rank2, order2) = cards::calc_hand(&hand2);
        let (rank3, order3) = cards::calc_hand(&hand3);
        assert_eq!(rank1, cards::Rank::Flash);
        assert_eq!(rank2, cards::Rank::Flash);
        assert_eq!(rank3, cards::Rank::Flash);
        assert!(order1 < order2);
        assert!(order2 < order3);
    }

    #[test]
    fn calc_hand_straight() {
        let mut hand1 = vec![0u32; 0];
        hand1.push(cards::encode(0, 12));
        hand1.push(cards::encode(1, 3));
        hand1.push(cards::encode(2, 1));
        hand1.push(cards::encode(3, 2));
        hand1.push(cards::encode(0, 0));
        let mut hand2 = vec![0u32; 0];
        hand2.push(cards::encode(0, 7));
        hand2.push(cards::encode(1, 10));
        hand2.push(cards::encode(2, 11));
        hand2.push(cards::encode(3, 9));
        hand2.push(cards::encode(0, 8));
        let mut hand3 = vec![0u32; 0];
        hand3.push(cards::encode(0, 0));
        hand3.push(cards::encode(1, 2));
        hand3.push(cards::encode(2, 4));
        hand3.push(cards::encode(3, 3));
        hand3.push(cards::encode(0, 1));

        let (rank1, order1) = cards::calc_hand(&hand1);
        let (rank2, order2) = cards::calc_hand(&hand2);
        let (rank3, order3) = cards::calc_hand(&hand3);
        assert_eq!(rank1, cards::Rank::Straight);
        assert_eq!(rank2, cards::Rank::Straight);
        assert_eq!(rank3, cards::Rank::Straight);
        assert!(order1 < order2);
        assert!(order2 > order3);
    }

    #[test]
    fn calc_hand_straight_flash() {
        let mut hand1 = vec![0u32; 0];
        hand1.push(cards::encode(0, 12));
        hand1.push(cards::encode(0, 3));
        hand1.push(cards::encode(0, 1));
        hand1.push(cards::encode(0, 2));
        hand1.push(cards::encode(0, 0));
        let mut hand2 = vec![0u32; 0];
        hand2.push(cards::encode(1, 7));
        hand2.push(cards::encode(1, 10));
        hand2.push(cards::encode(1, 11));
        hand2.push(cards::encode(1, 9));
        hand2.push(cards::encode(1, 8));
        let mut hand3 = vec![0u32; 0];
        hand3.push(cards::encode(2, 0));
        hand3.push(cards::encode(2, 2));
        hand3.push(cards::encode(2, 4));
        hand3.push(cards::encode(2, 3));
        hand3.push(cards::encode(2, 1));

        let (rank1, order1) = cards::calc_hand(&hand1);
        let (rank2, order2) = cards::calc_hand(&hand2);
        let (rank3, order3) = cards::calc_hand(&hand3);
        assert_eq!(rank1, cards::Rank::StraightFlash);
        assert_eq!(rank2, cards::Rank::StraightFlash);
        assert_eq!(rank3, cards::Rank::StraightFlash);
        assert!(order1 < order2);
        assert!(order2 > order3);
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

    #[test]
    #[should_panic]
    fn calc_hand_invalid_3() {
        let hand = vec![0u32, 1u32, 2u32, 3u32, 100];
        cards::calc_hand(&hand);
    }
}
