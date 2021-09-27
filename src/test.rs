use log::info;

use crate::{Action, StateMachine, action::ActionFlag};

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
enum States {
    #[default]
    Default,
    State1,
    State2,
}

struct Ctx {
    pub number: u8,
}

#[derive(Debug)]
pub struct ActionA {}

impl Action<States, (), Ctx> for ActionA {
    fn on_register(&mut self) -> Result<(), ()> {
        info!("on_register");
        Ok(())
    }

    fn on_first_run(&mut self, context: &mut Ctx) -> Result<(), ()> {
        info!("on_first_run");
        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &mut Ctx,
    ) -> Result<crate::action::ActionFlag<States>, ()> {
        info!("execute");
        Ok(ActionFlag::SwitchState(States::State2))
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ()> {
        info!("on_finish");
        Ok(())
    }
}

#[derive(Debug)]
pub struct ActionB {}

impl Action<States, (), Ctx> for ActionB {
    fn on_register(&mut self) -> Result<(), ()> {
        info!("on_register");
        Ok(())
    }

    fn on_first_run(&mut self, context: &mut Ctx) -> Result<(), ()> {
        info!("on_first_run");
        Ok(())
    }

    fn execute(
        &mut self,
        delta: &chrono::Duration,
        context: &mut Ctx,
    ) -> Result<crate::action::ActionFlag<States>, ()> {
        info!("execute");
        Ok(ActionFlag::Continue)
    }

    fn on_finish(&mut self, interrupted: bool) -> Result<(), ()> {
        info!("on_finish");
        Ok(())
    }
}

#[test]
fn test_state_machine_execution() {
    let mut machine = StateMachine::<States, (), Ctx>::new();
    machine.add_action(States::State1, ActionA {});
    machine.add_action(States::State2, ActionB {});
    
    let mut ctx = Ctx { number: 0 };

    for _ in 0..10 {
        machine.run(&mut ctx).unwrap();
    }
}
