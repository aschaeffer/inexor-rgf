use crate::TypedReactivePropertyImpl;
use reactive_graph_reactive_model_api::ReactiveInstance;
use serde_json::json;

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> FnOnce<(f64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    type Output = ();

    extern "rust-call" fn call_once(mut self, args: (f64,)) -> Self::Output {
        self.call_mut(args)
    }
}

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> FnMut<(f64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call_mut(&mut self, args: (f64,)) -> Self::Output {
        self.call(args)
    }
}

#[rustversion::nightly]
impl<IdType, ReactiveInstanceType> Fn<(f64,)> for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    extern "rust-call" fn call(&self, args: (f64,)) -> Self::Output {
        self.reactive_instance.set(&self.property_name, json!(args.0));
    }
}

impl<IdType, ReactiveInstanceType> TypedReactivePropertyImpl<IdType, ReactiveInstanceType, f64>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    pub fn op<F>(&self, f: F)
    where
        F: Fn(f64) -> f64,
    {
        if let Some(v) = self.reactive_instance.as_f64(&self.property_name) {
            self.reactive_instance.set(&self.property_name, json!(f(v)));
        }
    }
}
