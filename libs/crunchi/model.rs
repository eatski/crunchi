pub mod model {
    
    pub enum Block {
        Stop(StopColor),
        Empty,
        Symbol(SymbolType),
        Start,
        Goal
    }

    pub enum SymbolType {
        A,B
    }
    pub enum StopColor {
        White,Black
    }

    pub struct Action {
        pub initiative: Initiative,
        pub action_type: ActionType
    }
    
    pub enum Initiative {
        _1,_2,_3,_4,_5
    }
    
    pub enum ActionType {
        Move(MoveNum,StopColor),
        Warp,
        ObstructPlayer,
        ObstructHero(Hero)
    }
    
    pub enum MoveNum {
        _1,_2,_3
    }
    
    pub enum GoalType {
        Arrival(Hero),
        Matching,
        Scrap(Hero),
        Symbol(SymbolType),
        Seizure,
    }
    
    pub enum Hero {
        Adam,Eve
    }
    
    pub enum Baggage {
        Symbol,
        FailedAction
    }
    
}


pub mod state {
    use super::model::{*};
    use std::collections::HashMap;

    pub struct PlayingState {
        pub blocks: Vec<Block>,
        pub heros: HashMap<Hero,HeroState>
    }
    
    pub enum HeroState {
        Alive {
            baggages: Vec<Baggage>,
            symbols: Vec<SymbolType>
        },
        Arraival {
            symbols: Vec<SymbolType>
        },
        Scrap
    }
    
    pub enum HeroLastState {
        Arraival {
            symbols: Vec<SymbolType>
        },
        Scrap
    }
}



