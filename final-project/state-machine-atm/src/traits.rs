use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::atm::{Key, MyEnum};


/// A state machine - Generic over the transition type
pub trait StateMachine {
    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Calculate the resulting state when this state undergoes the given transition
    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State;
}



// Simple helper to do some hashing.
pub(crate) fn hash<T: MyEnum>(t: &[T]) -> u64 {
    let mut res: u64 = 0;
    for i in t.iter() {
        match i.get_value() {
            1 => res = res * 10 + 1,
            2 => res = res * 10 + 2,
            3 => res = res * 10 + 3,
            4 => res = res * 10 + 4,
            _ => return 0
        }
    }
    res
}

// Test for hash function 
#[test]
fn test_hash_enum_vec() {
    enum KeyTest{
        One,
        Two,
        Three,
        Four
    }
    impl MyEnum for KeyTest {
        fn get_value(&self) -> u32 {
            match self {
                KeyTest::One => 1,
                KeyTest::Two => 2,
                KeyTest::Three => 3,
                KeyTest::Four => 4,
            }
        }
    }
    let input: Vec<KeyTest> = vec![KeyTest::One, KeyTest::Two, KeyTest::Three, KeyTest::Four];

    let hash1 = hash(&input);
    let hash2 = hash(&input);

    assert_eq!(hash1, hash2);
}
