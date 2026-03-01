pub mod boundary;
pub mod memory;
pub mod pulse;

use self::memory::Memory;
use self::memory::blackboard::Blackboard;
use self::pulse::Pulse;

pub struct Mass {
	memory: Memory,
	pulses: Vec<Pulse>,
}

impl Mass {
	pub fn construct(memory: Memory, pulses: Vec<Pulse>) -> Self {
		assert!(
			!pulses.is_empty(),
			"a Mass must have at least one Pulse"
		);

		Self { memory, pulses }
	}

	pub fn placeholder() -> Self {
		let memory = Memory {
			blackboard: Blackboard::new(),
		};
		let pulses = vec![Pulse::new()];

		Self::construct(memory, pulses)
	}
}
