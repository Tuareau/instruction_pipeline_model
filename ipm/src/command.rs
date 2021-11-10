pub struct Command {
	type: CommandType,
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