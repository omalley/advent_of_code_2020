use std::cell::RefCell;
use std::io;
use std::io::BufRead;
use std::rc::{Rc, Weak};

type Link<T> = Option<Weak<RefCell<T>>>;

#[derive(Debug)]
struct ListNode {
  name: usize,
  prev: Link<ListNode>,
  next: Link<ListNode>,
}

// Implement an efficient data structure for the operations we need.
// * Remove from near front of the list
// * Move from front to back of the list
// * Find where cup N is located currently
// * Insert nodes in the middle of the list
//
#[derive(Debug)]
struct CupGame {
  // index to find where each cup is currently (uses name - 1)
  // this is the owning pointer
  index: Vec<Option<Rc<RefCell<ListNode>>>>,
  // the circular doubly linked list of the cups position in the circle
  circle: Option<Weak<RefCell<ListNode>>>,
}

impl CupGame {
  const TOTAL_CUPS: usize = 1_000_000;
  const CUPS_TO_MOVE: usize = 3;
  const MOVES_TO_MAKE: usize = 10_000_000;

  fn parse(input: &str) -> Self {
    let mut result = CupGame{index: vec![None; Self::TOTAL_CUPS], circle: None};
    for c in input.chars().map(|c| c.to_string().parse::<usize>().unwrap()) {
      let mut node = result.create_node(c);
      result.append_node(&mut node);
    }
    // Add the extra cups to the end of the list
    for c in 1..=Self::TOTAL_CUPS {
      if result.index[c-1].is_none() {
        let mut node = result.create_node(c);
        result.append_node(&mut node);
      }
    }
    result
  }

  // Create a new cup
  fn create_node(&mut self, name: usize) -> Rc<RefCell<ListNode>> {
    let node = Rc::new(RefCell::new(
      ListNode{name, prev: None, next: None}));
    self.index[name-1] = Some(node.clone());
    node
  }

  // Append a node to the end of the circle
  fn append_node(&mut self, node: &mut Rc<RefCell<ListNode>>) {
    let node_ref = node.as_ref();
    match &self.circle {
      None => {
        node_ref.borrow_mut().prev = Some(Rc::downgrade(node));
        node_ref.borrow_mut().next = Some(Rc::downgrade(node));
        self.circle = Some(Rc::downgrade(node));
      },
      Some(weak_head) => {
        let head = weak_head.upgrade().unwrap();
        let tail = head.borrow().prev.as_ref().unwrap().upgrade().unwrap();
        tail.borrow_mut().next = Some(Rc::downgrade(node));
        node_ref.borrow_mut().prev = Some(Rc::downgrade(&tail));
        node_ref.borrow_mut().next = Some(weak_head.clone());
        head.as_ref().borrow_mut().prev = Some(Rc::downgrade(node));
      },
    }
  }

  // Remove a sublist from the circle
  // starts at offset start for length cups
  // Returns the names of the removed cups
  fn remove_range(&mut self, start: usize, length: usize) -> Vec<usize> {
    // Find the element just before the range
    let mut prior = self.circle.as_ref().unwrap().upgrade().unwrap();
    for _ in 0..start-1 {
      let next = prior.borrow().next.as_ref().unwrap().upgrade().unwrap();
      prior = next;
    }
    // Get the first to remove
    let first = prior.borrow().next.as_ref().unwrap().upgrade().unwrap();
    let mut result = vec![first.borrow().name];
    // Advance to the last one to remove
    let mut last = first.clone();
    for _ in 0..length-1 {
      let next = last.borrow().next.as_ref().unwrap().upgrade().unwrap();
      result.push(next.borrow().name);
      last = next;
    }
    let follow = last.borrow().next.as_ref().unwrap().upgrade().unwrap();
    // close up the list around them
    prior.borrow_mut().next = Some(Rc::downgrade(&follow));
    follow.borrow_mut().prev = Some(Rc::downgrade(&prior));
    // close the removed sublist
    first.borrow_mut().prev = Some(Rc::downgrade(&last));
    last.borrow_mut().next = Some(Rc::downgrade(&first));
    result
  }

  // Find where we are going to move the sublist.
  // the current id - 1 wrapping around, but avoids the removed cups
  fn find_destination(&self, current:usize, removed: &Vec<usize>) -> Rc<RefCell<ListNode>> {
    // convert to 0..TOTAL_CUPS - 1
    let mut current = (Self::TOTAL_CUPS + current - 2) % Self::TOTAL_CUPS;
    loop {
      if !removed.contains(&(current + 1)) {
        return self.index[current].clone().unwrap()
      }
      current = (Self::TOTAL_CUPS + current - 1) % Self::TOTAL_CUPS
    }
  }

  // Adds the sublist back into the circle at the given point
  fn splice_after(&self, position: Rc<RefCell<ListNode>>, moved: &Vec<usize>) {
    let sublist = self.index[moved[0]-1].as_ref().unwrap();
    let sublist_end = sublist.borrow().prev.as_ref().unwrap().upgrade().unwrap();
    let after = position.borrow().next.as_ref().unwrap().upgrade().unwrap();
    sublist.borrow_mut().prev = Some(Rc::downgrade(&position));
    sublist_end.borrow_mut().next = Some(Rc::downgrade(&after));
    position.borrow_mut().next = Some(Rc::downgrade(&sublist));
    after.borrow_mut().prev = Some(Rc::downgrade(&sublist_end));
  }

  // Advance current to the right
  fn move_current(&mut self) {
    self.circle = self.circle.as_ref().unwrap().upgrade().unwrap().borrow().next.clone();
  }

  // take a move in the game
  fn next(&mut self) {
    let moving = self.remove_range(1, Self::CUPS_TO_MOVE);
    let destination = self.find_destination(
      self.circle.as_ref().unwrap().upgrade().unwrap().borrow().name, &moving);
    // Add the cups back into the circle
    self.splice_after(destination, &moving);
    // Move current one to the right
    self.move_current();
  }

  // print the cups in order, starting from the current one
  fn print(&self) {
    let mut current = self.circle.as_ref().unwrap().upgrade().unwrap();
    let first = current.borrow().name;
    let mut is_first = true;
    while is_first || current.borrow().name != first {
      print!("{}{}", if is_first {""} else {", "}, current.borrow().name);
      is_first = false;
      let next = current.borrow().next.as_ref().unwrap().upgrade().unwrap();
      current = next;
    }
    println!();
  }

  // Find cup 1 and return the following cup names
  fn get_signature(&self) -> (usize, usize) {
    let one =
      self.index[0].as_ref().unwrap().borrow().next.as_ref().unwrap().upgrade().unwrap();
    let two = one.borrow().next.as_ref().unwrap().upgrade().unwrap();
    let result = (one.borrow().name, two.borrow().name);
    result
  }
}

fn main() {
  let stdin = io::stdin();
  let mut game = CupGame::parse(stdin.lock().lines().next().unwrap().unwrap().trim());
  for _ in 0..CupGame::MOVES_TO_MAKE {
    game.next();
  }
  let pair = game.get_signature();
  println!("signature {} * {} = {}", pair.0, pair.1, pair.0 * pair.1);
}
