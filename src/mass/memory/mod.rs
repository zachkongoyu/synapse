pub mod blackboard;

use self::blackboard::Blackboard;

pub (in crate::mass) struct Memory {
    pub (in crate::mass) blackboard: Blackboard,
}