use super::StateMachine;


pub struct LightSwitch;

impl StateMachine for LightSwitch{
    type State = bool;
    type Transition = ();

    fn next_state(mut starting_state: &bool, t: &()) -> bool {
		if starting_state==&true{
            starting_state=&false;
            starting_state.to_owned()
        }else{
            starting_state=&true;
            starting_state.to_owned()  
        }
	}
}

pub struct WeirdSwitches;

#[derive(PartialEq, Eq, Debug)]
pub struct TwoSwitches{
    first_switch:bool,
    second_switch:bool
}


pub enum Toggle{
    FirstSwitch,
    SecondSwitch
}

impl StateMachine for WeirdSwitches{
    type State = TwoSwitches;
    type Transition = Toggle;

    fn next_state(starting_state:&Self::State,t:&Self::Transition)->Self::State {
       
       match *t{
        Toggle::FirstSwitch=>{
            if starting_state.first_switch{
                TwoSwitches{
                    first_switch:false,
                    second_switch:false
                }
            }else{
                TwoSwitches{
                    first_switch:starting_state.first_switch,
                    second_switch:starting_state.second_switch
                }
            }
        },
        Toggle::SecondSwitch=>{
             TwoSwitches {
                    first_switch: starting_state.first_switch,
                    second_switch: !starting_state.second_switch,
                }
        }
        
       }
       
        
    }
}

