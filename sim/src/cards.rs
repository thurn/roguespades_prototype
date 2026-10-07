//! Cards as indices 0..52 (suit * 13 + rank - 2) and hands as 64-bit masks.
//!
//! A card index names a physical slot in the round's deck. Each slot has an effective suit and
//! rank, which synthetic copies and Opening changes can rewrite for the round.

pub const CLUBS: u8 = 0;
pub const DIAMONDS: u8 = 1;
pub const HEARTS: u8 = 2;
pub const SPADES: u8 = 3;
pub const ACE: u8 = 14;
pub const KING: u8 = 13;
pub const QUEEN: u8 = 12;
pub const JACK: u8 = 11;

pub type Mask = u64;

#[inline]
pub fn card(suit: u8, rank: u8) -> u8 {
    suit * 13 + rank - 2
}
#[inline]
pub fn nominal_suit(c: u8) -> u8 {
    c / 13
}
#[inline]
pub fn nominal_rank(c: u8) -> u8 {
    c % 13 + 2
}
#[inline]
pub fn bit(c: u8) -> Mask {
    1u64 << c
}

pub const FULL_DECK: Mask = (1u64 << 52) - 1;

/// Iterates the set bits of a mask, lowest first.
pub struct Bits(pub Mask);
impl Iterator for Bits {
    type Item = u8;
    #[inline]
    fn next(&mut self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }
        let c = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(c)
    }
}

pub fn suit_from_str(s: &str) -> Option<u8> {
    match s {
        "C" => Some(CLUBS),
        "D" => Some(DIAMONDS),
        "H" => Some(HEARTS),
        "S" => Some(SPADES),
        _ => None,
    }
}

pub fn rank_from_str(s: &str) -> Option<u8> {
    match s {
        "A" => Some(14),
        "K" => Some(13),
        "Q" => Some(12),
        "J" => Some(11),
        _ => s.parse::<u8>().ok().filter(|r| (2..=10).contains(r)),
    }
}

pub fn suit_symbol(s: u8) -> &'static str {
    ["♣", "♦", "♥", "♠"][s as usize]
}
pub fn suit_letter(s: u8) -> &'static str {
    ["C", "D", "H", "S"][s as usize]
}
pub fn rank_str(r: u8) -> String {
    match r {
        14 => "A".into(),
        13 => "K".into(),
        12 => "Q".into(),
        11 => "J".into(),
        _ => r.to_string(),
    }
}
pub fn card_str(suit: u8, rank: u8) -> String {
    format!("{}{}", rank_str(rank), suit_symbol(suit))
}

/// The effective identity of every slot for one round.
#[derive(Clone, Copy)]
pub struct Identity {
    pub suit: [u8; 52],
    pub rank: [u8; 52],
    /// Slots by effective suit.
    pub suit_mask: [Mask; 4],
}

impl Identity {
    pub fn standard() -> Self {
        let mut id = Identity {
            suit: [0; 52],
            rank: [0; 52],
            suit_mask: [0; 4],
        };
        for c in 0..52u8 {
            id.suit[c as usize] = nominal_suit(c);
            id.rank[c as usize] = nominal_rank(c);
        }
        id.rebuild();
        id
    }
    pub fn rebuild(&mut self) {
        self.suit_mask = [0; 4];
        for c in 0..52u8 {
            self.suit_mask[self.suit[c as usize] as usize] |= bit(c);
        }
    }
    pub fn set(&mut self, c: u8, suit: u8, rank: u8) {
        let old = self.suit[c as usize];
        self.suit_mask[old as usize] &= !bit(c);
        self.suit[c as usize] = suit;
        self.rank[c as usize] = rank.min(ACE);
        self.suit_mask[suit as usize] |= bit(c);
    }
    /// A sort key: higher is stronger, spades above everything.
    #[inline]
    pub fn power(&self, c: u8) -> u8 {
        self.rank[c as usize]
            + if self.suit[c as usize] == SPADES {
                13
            } else {
                0
            }
    }
}
