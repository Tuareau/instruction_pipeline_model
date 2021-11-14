
static mut COMMANDS_COUNT: u32 = 0;

pub struct Command {
	name: String,
	cmd_type: CommandType,
	left_operand_type: OperandType,
	right_operand_type: OperandType,
	clock_cycles: u32;
}

pub enum CommandType {
	First,
	Second,
}

pub enum OperandType {
	Register,
	Memory,
}

impl Command {
	
	pub fn name(&self) -> String& {
		self.name
	}

	pub fn type(&self) -> CommandType& {
		self.cmd_type
	}

	pub fn left_operand_type(&self) -> OperandType& {
		self.left_operand_type
	}

	pub fn right_operand_type(&self) -> OperandType& {
		self.right_operand_type
	}

	pub fn generate() -> Command {
		
	}
}