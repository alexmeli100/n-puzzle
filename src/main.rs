use std::collections::{HashMap, VecDeque, BinaryHeap,};
use std::collections::hash_map::{Entry};
use std::fmt;
use std::cmp::Ordering;
use std::time::{Instant};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Item(i32, State);

impl Ord for Item {
  fn cmp (&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0).reverse()
  }
}

impl PartialOrd for Item {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct State {
  board: Vec<i32>,
}

impl State {
  fn manhattan_dis(&self) -> i32 {
    self.board.iter()
      .filter(|x| **x != 0)
      .fold(0, |acc, x| acc + self.target_dis(*x))
  }

  fn get_pos(&self, num: i32) -> Option<(usize, usize)> {
    if let Some(pos) = self.board.iter().position(|&x| x == num) {
      return Some((pos % 3, pos / 3));
    }
    
    None  
  }

  fn target_dis(&self, num: i32) -> i32 {
    match self.get_pos(num) {
      Some((x, y)) => {
        let target_y = (num - 1) / 3;
        let target_x = (num - 1) % 3;

        (x as i32 - target_x).abs() + (y as i32 - target_y).abs()
      },

      _ => panic!("Invalid number in board")
    }
  }

  fn neighbors(&self) -> Vec<State> {
    let zero_pos = self.get_pos(0).unwrap();

    ['U', 'D', 'L', 'R'].iter()
      .filter_map(|dir| self.move_dir(*dir, zero_pos))
      .collect::<Vec<State>>()
      
  }

  fn move_dir(&self, dir: char, zero_pos: (usize, usize)) -> Option<State> {
    let (x0, y0) = zero_pos;
    let mut b = self.board.clone();

    match dir {
      'U' if y0 > 0 => {
        b.swap(y0 * 3 + x0, (y0 - 1) * 3 + x0);
      },
      'D' if y0 < 2 => {
        b.swap(y0 * 3 + x0, (y0 + 1) * 3 + x0);
      },
      'L' if x0 > 0 => {
        b.swap(y0 * 3 + x0, y0 * 3 + (x0 - 1));
      },
      'R' if x0 < 2 => {
        b.swap(y0 * 3 + x0, y0 * 3 + (x0 + 1));
      }

      _ => return None
    };

    let new_state = State { board: b};

    Some(new_state)
  }
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for y in 0..3 {
      write!(f, "|")?;
      for x in 0..3 {
        let num = self.board[y * 3 + x];
        write!(f, "{}|", num)?;
      }
      writeln!(f, "")?;
    }
    Ok(())
  }
}

fn find_path() -> Option<VecDeque<State>> {
  let state = State { board: [1, 2, 5, 3, 4, 0, 6, 7, 8].to_vec() };
  let mut frontier = BinaryHeap::new();
  let mut previous = HashMap::new();
  let mut path_cost = HashMap::new();

  frontier.push(Item(state.manhattan_dis(), state.clone()));
  previous.insert(state.clone(), None);
  path_cost.insert(state.clone(), 0); 

  while let Some(Item(_, s)) = frontier.pop() {
    let h_val = s.manhattan_dis();

    if h_val == 0 {
      return Some(get_path(previous, s.clone()));
    }

    for n in s.neighbors().iter() {
      let new_cost = path_cost.get(&s).unwrap() + 1;
      let h = n.manhattan_dis();

      match path_cost.entry(n.clone()) {
        Entry::Vacant(e) => {
          previous.insert(n.clone(), Some(s.clone()));
          e.insert(new_cost);
        }

        Entry::Occupied(mut e) => {
          if *e.get() > new_cost {
            previous.insert(n.clone(), Some(s.clone()));
            e.insert(new_cost);
          } else {
            continue;
          }
        }
      }

      frontier.push(Item(h + new_cost, n.clone()));
    }
  }

  None
}

fn get_path(previous: HashMap<State, Option<State>>, goal: State) -> VecDeque<State> {
  let mut path = VecDeque::new();
  let mut g = goal;
  path.push_front(g.clone());

  while let Some(p) = previous.get(&g) {
    match p {
      Some(s) => { 
        path.push_front(s.clone());
        g = s.clone();
      },
      None => break
    }
  }
  path
}

fn main() {
  let now = Instant::now();
  if let Some(path) = find_path() {
    for p in path.iter() {
      println!("{}", p);
    }
  }

  println!("{:?}", Instant::now().duration_since(now)); 
}
