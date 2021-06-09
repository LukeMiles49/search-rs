mod list;
use list::List;

pub trait Search {
	fn search<S, T: Clone, P: Problem<S, T>>(problem: &P) -> Option<Vec<T>>;
}

pub trait Problem<State, Transition> {
	fn start_state(&self) -> State;
	
	fn transitions(&self, state: &State) -> Vec<Transition>;
	
	fn apply(&self, state: &State, transition: &Transition) -> State;
	
	fn is_solution(&self, state: &State) -> bool;
}

pub enum BreadthFirstSearch { }

impl Search for BreadthFirstSearch {
	fn search<S, T: Clone, P: Problem<S, T>>(problem: &P) -> Option<Vec<T>> {
		let mut current: Vec<(S, List<T>)> = vec![(problem.start_state(), List::Nil)];
		
		while current.len() > 0 {
			let prev = current;
			
			current = Vec::new();
			
			for (state, transitions) in prev {
				for transition in problem.transitions(&state) {
					let new_state = problem.apply(&state, &transition);
					let new_transitions = List::cons(transition, transitions.clone());
					if problem.is_solution(&new_state) {
						let mut result: Vec<T> = new_transitions.iter().cloned().collect();
						result.reverse();
						return Some(result);
					}
					current.push((new_state, new_transitions));
				}
			}
		}
		
		None
	}
}
