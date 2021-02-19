//Dijkstra är nice
// det här problemet är ganska tidsödande att faktiskt implementera, mycket lättare att bara beskriva algoritmen >:[
// dokumentation gav mig exempelimplementation av priority queue
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

// struct used to store where we are and how much we've paid
#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Case a) is really simple, algorithmically. We simply 
 for each possible starting position, for each step, 
 we take the lowest cost path, then we compare the starting positions.
*/

// case b) is also similarly simple, except we only have to do it once

// case c) is like case b), but we have a limited amount of steps that aren't diagonally downwards.
// This is where we have to actually start implementing Dijkstra

// case d) would just require keeping track of where we've been, by storing each step taken in a data structure of some kind.


//simple struct, essentially a tuple
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra using an adjacency list.
fn dajkstra(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> usize {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // init
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // we look at the next node in the pqueue 
    while let Some(State { cost, position }) = heap.pop() {
        // if we're where we want to be we're done!
        if position == goal { return cost }

        // if we have found a better way we don't want to overwrite it
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through that node

        //we would cheat here in case c)
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // if we do, we add it to the pqueue
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    //should never run
    return 0;
}

fn main() {
    // get standard input stream
    let input = io::stdin();
    // or read single line
    let mut line = String::new();
    
    let mapsize = input.read_line(&mut line).expect("IO Error") as usize;
    //initialize the map using an adjacency list 
    //costs are 1 because i don't have time to implement actual costs
    
    // actually implementing the input here is SUPER HARD 
    // and way out of scope of the original task
    let mut graph = vec![];
    for i in 1..mapsize{
        for j in 1..mapsize{
            let mut adj_list = vec![];
            //the one straight ahead
            if j + 1 < mapsize {adj_list.push(vec![Edge { node: j+1, cost: 1}])}
            //the one diagonally up
            if j + 1 < mapsize && i > 1 {adj_list.push(vec![Edge { node: j+1 - mapsize, cost: 1}])}
            //the one diagonally down
            if j + 1 < mapsize && i < mapsize {adj_list.push(vec![Edge { node: j+1 + mapsize, cost: 1}])}
            graph.push(adj_list);
        }
    }    
}
