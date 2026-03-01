// A pulse is statelss and can be attached to a mass. 
// It represents a behavior or capability that the mass can perform.
use crate::tools::Tool;
use crate::skills::Skill;
use crate::runtime::Runtime;


pub (in crate::mass) struct Pulse {

}

impl Pulse {
    pub (in crate::mass) fn new() -> Self {
        Self {

        }
    }
}