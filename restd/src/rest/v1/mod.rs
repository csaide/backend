use std::panic::RefUnwindSafe;

use gotham::pipeline::chain::PipelineHandleChain;
use gotham::router::builder::*;
use gotham::state::State;

fn endpoint(state: State) -> (State, &'static str) {
    (state, "hello")
}

pub fn router<C, P>(route: &mut impl DrawRoutes<C, P>)
where
    C: PipelineHandleChain<P> + Copy + Send + Sync + 'static,
    P: RefUnwindSafe + Send + Sync + 'static,
{
    route.get("/").to(endpoint);
}
