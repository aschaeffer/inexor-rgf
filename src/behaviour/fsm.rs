use log::trace;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveInstance;
use crate::BehaviourReactiveInstanceContainer;
use crate::BehaviourState;
use crate::BehaviourTransitionError;
use crate::BehaviourTransitions;
use crate::BehaviourValidator;

pub trait BehaviourFsm<T: ReactiveInstance>: BehaviourReactiveInstanceContainer<T> + Send + Sync {
    /// Returns the current state of the behaviour.
    fn ty(&self) -> &BehaviourTypeId;

    /// Returns the current state of the behaviour.
    fn get_state(&self) -> BehaviourState;

    /// Returns the current state of the behaviour.
    fn set_state(&self, state: BehaviourState);

    /// Returns the validator.
    fn get_validator(&self) -> &dyn BehaviourValidator<T>;

    /// Returns the validator.
    fn get_transitions(&self) -> &dyn BehaviourTransitions<T>;

    /// Executes a behaviour transition.
    fn transition(&self, target_state: BehaviourState) -> Result<(), BehaviourTransitionError> {
        trace!("transition {:?} -> {:?}", self.get_state(), target_state);
        match self.get_state() {
            BehaviourState::Created => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => self
                    .get_validator()
                    .validate()
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourInvalid),
                BehaviourState::Ready => self.transition(BehaviourState::Valid).and_then(|_| {
                    self.get_transitions()
                        .init()
                        .map(|_| self.set_state(target_state))
                        .map_err(BehaviourTransitionError::BehaviourInitializationFailed)
                }),
                BehaviourState::Connected => self.transition(BehaviourState::Ready).and_then(|_| {
                    self.get_transitions()
                        .connect()
                        .map(|_| self.get_reactive_instance().add_behaviour(self.ty().clone()))
                        .map(|_| self.set_state(target_state))
                        .map_err(BehaviourTransitionError::BehaviourConnectFailed)
                }),
            },
            BehaviourState::Valid => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => self
                    .get_transitions()
                    .init()
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourInitializationFailed),
                BehaviourState::Connected => self.transition(BehaviourState::Ready).and_then(|_| {
                    self.get_transitions()
                        .connect()
                        .map(|_| self.get_reactive_instance().add_behaviour(self.ty().clone()))
                        .map(|_| self.set_state(target_state))
                        .map_err(BehaviourTransitionError::BehaviourConnectFailed)
                }),
            },
            BehaviourState::Ready => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Connected => self
                    .get_transitions()
                    .connect()
                    .map(|_| self.get_reactive_instance().add_behaviour(self.ty().clone()))
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourConnectFailed),
            },
            BehaviourState::Connected => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => self
                    .get_transitions()
                    .disconnect()
                    .map(|_| self.get_reactive_instance().remove_behaviour(self.ty()))
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourDisconnectFailed),
                BehaviourState::Connected => Err(BehaviourTransitionError::InvalidTransition),
            },
        }
    }
}

#[macro_export]
macro_rules! behaviour_fsm {
    ($fsm: ident, $validator: ty, $transitions: ty, $reactive_instance: ty) => {
        pub struct $fsm {
            pub reactive_instance: std::sync::Arc<$reactive_instance>,
            pub ty: inexor_rgf_core_model::BehaviourTypeId,
            pub state: std::sync::RwLock<BehaviourState>,
            pub validator: $validator,
            pub transitions: $transitions,
        }

        impl $fsm {
            pub fn new(
                reactive_instance: std::sync::Arc<$reactive_instance>,
                ty: inexor_rgf_core_model::BehaviourTypeId,
                validator: $validator,
                transitions: $transitions,
            ) -> Self {
                $fsm {
                    reactive_instance,
                    ty,
                    state: std::sync::RwLock::new(BehaviourState::Created),
                    validator,
                    transitions,
                }
            }
        }

        impl BehaviourFsm<$reactive_instance> for $fsm {
            fn ty(&self) -> &inexor_rgf_core_model::BehaviourTypeId {
                &self.ty
            }

            fn get_state(&self) -> BehaviourState {
                let reader = self.state.read().unwrap();
                (*reader).clone()
            }

            fn set_state(&self, state: BehaviourState) {
                let mut writer = self.state.write().unwrap();
                *writer = state;
            }

            fn get_validator(&self) -> &dyn BehaviourValidator<$reactive_instance> {
                &self.validator
            }

            fn get_transitions(&self) -> &dyn BehaviourTransitions<$reactive_instance> {
                &self.transitions
            }
        }

        impl BehaviourReactiveInstanceContainer<$reactive_instance> for $fsm {
            fn get_reactive_instance(&self) -> &std::sync::Arc<$reactive_instance> {
                &self.reactive_instance
            }

            fn get(&self, property_name: &str) -> Option<serde_json::Value> {
                self.reactive_instance.get(property_name)
            }

            fn set(&self, property_name: &str, value: serde_json::Value) {
                self.reactive_instance.set(property_name, value);
            }
        }
    };
}
