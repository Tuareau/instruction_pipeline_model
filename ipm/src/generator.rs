
use rand::Rng;
use command::OperandType;
use command::CommandType;

struct Generator {}

impl Generator {

    fn generate_register_adressing_probability() -> f64 {
        let mut prob = 0.0;
        let P = rand::thread_rng().gen_range(1..=3);
        match P {
            1 => prob = 0.9,
            2 => prob = 0.8,
            3 => prob = 0.6,
        }
        prob
    }

    fn generate_first_command_type_probability() -> f64 {
        let mut prob = 0.0;
        let P = rand::thread_rng().gen_range(1..=3);
        match P {
            1 => prob = 0.9,
            2 => prob = 0.7,
            3 => prob = 0.5,
        }
        prob
    }

    fn generate_memory_access_clc(op_type: OperandType&) -> u32 {
        match op_type {
            OperandType::Register => 1,
            OperandType::Memory => {
                let mut clc = 1;
                let prob = Generator::generate_register_adressing_probability();
                if prob >= 0.6 {
                    clc = 10;
                }
                if prob >= 0.8 {
                    clc = 5;
                }
                if prob >= 0.9 {
                    clc = 2;
                }
                clc
            }
        }
    }

    fn generate_execution_clc(cmd_type: CommandType&) -> u32 {
        match cmd_type {
            CommandType::First => 1,
            CommandType::Second => {
                let mut clc = 1;
                let prob = Generator::generate_first_command_type_probability();
                if prob >= 0.6 {
                    clc = 10;
                }
                if prob >= 0.8 {
                    clc = 5;
                }
                if prob >= 0.9 {
                    clc = 2;
                }
                clc
            }
        }
    }

}

impl Generator {

    pub fn generate_command_type() -> CommandType {
        let fst_cmd_prob = Generator::generate_first_command_type_probability();
        let prob = rand::thread_rng().gen_range(0..=100);
        if (prob >= 100 - fst_cmd_prob * 100) {
            CommandType::First
        }
        else {
            CommandType::Second
        }
    }

    pub fn generate_operand_type() -> OperandType {
        let reg_prob = Generator::generate_register_adressing_probability();
        let prob = rand::thread_rng().gen_range(0..=100);
        if (prob >= 100 - reg_prob * 100) {
            OperandType::Register
        }
        else {
            OperandType::Memory
        }
    }

    pub fn generate_clock_cycles(cmd: Command&, stage: Pipeline::Stage) -> u32 {
        let mut clc = match stage {
            Pipeline::Stage::Decode => clc = 1,
            Pipeline::Stage::FetchLeftOperand => 
                Generator::generate_memory_access_clc(cmd.left_operand_type()),
            Pipeline::Stage::FetchRightOperand =>
                Generator::generate_memory_access_clc(cmd.right_operand_type()),
            Pipeline::Stage::Execute =>
                Generator::generate_execution_clc(cmd.type()),
            Pipeline::Stage::WriteBack =>
                Generator::generate_memory_access_clc(cmd.right_type_operand()),
        }
        clc
    }

}