mod p1_switches;


pub trait StateMachine {
    type State;
    type Transition;
    fn next_state(state:&Self::State,t:&Self::Transition)->Self::State;
    fn human_name()->String{
        "Unnamed state Machine".into()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum User {
    Alice,
    Bob,
    Charlie
    
}