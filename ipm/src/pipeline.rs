use std::collections::HashMap;

use command::Command;
use stats_collector::StatsCollector;

struct Pipeline {
    stats_collector: StatsCollector,
    executing_commands: HashMap<u32, Command>,
    commands_vector: Vec<Command>,
}

enum Stage {
    Decode,
    FetchLeftOperand,
    FetchRightOperand,
    Execute,
    WriteBack,
}

static STAGES_COUNT: u32 = 5;

impl Pipeline {
    fn run_cycle_clock(&self) {

    }

    fn try_insert_command(&self) {

    }

    fn try_shift_command(&self, key: u32) {

    }
}

impl Pipeline {
    pub fn stage_to_key(stage: Stage&) -> u32 {
        match stage {
            Stage::Decode => 0,
            Stage::FetchLeftOperand => 1,
            Stage::FetchRightOperand => 2,
            Stage::Execute => 3,
            Stage::WriteBack => 4,
        }
    }

    pub fn key_to_stage(key: u32) -> Stage {
        match key {
            0 => Stage::Decode,
            1 => Stage::FetchLeftOperand,
            2 => Stage::FetchRightOperand,
            3 => Stage::Execute,
            4 => Stage::WriteBack,
        }
    }

    pub fn new(cmd_count: u32) -> Pipeline {

    }

    pub fn run(&self) {
        
    }
}