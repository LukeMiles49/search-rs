use search::{Problem, Search, BreadthFirstSearch};
use std::cmp::min;

#[derive(Debug, Copy, Clone)]
struct JugsProblem<const N: usize> {
	jugs: [usize; N],
	start: [usize; N],
	goal: usize,
}

#[derive(Debug, Copy, Clone)]
struct JugState<const N: usize> {
	liquid: [usize; N],
}

#[derive(Debug, Copy, Clone)]
struct Pour {
	from: usize,
	to: usize,
}

impl<const N: usize> Problem<JugState<N>, Pour> for JugsProblem<N> {
	fn start_state(&self) -> JugState<N> {
		JugState { liquid: self.start }
	}
	
	fn transitions(&self, state: &JugState<N>) -> Vec<Pour> {
		let mut transitions = Vec::new();
		for from in 0..N {
			for to in 0..N {
				if from != to {
					transitions.push(Pour { from, to });
				}
			}
		}
		transitions
	}
	
	fn apply(&self, state: &JugState<N>, transition: &Pour) -> JugState<N> {
		let Pour { from, to } = *transition;
		let amount = min(state.liquid[from], self.jugs[to] - state.liquid[to]);
		let mut state = state.clone();
		state.liquid[from] -= amount;
		state.liquid[to] += amount;
		state
	}
	
	fn is_solution(&self, state: &JugState<N>) -> bool {
		state.liquid.iter().any(|&l| l == self.goal)
	}
}

#[test]
fn jugs_game() {
	let problem = JugsProblem {
		jugs: [12, 8, 5],
		start: [12, 0, 0],
		goal: 6,
	};
	
	println!("{:?}", BreadthFirstSearch::search(&problem));
}
