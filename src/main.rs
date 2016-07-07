mod graph;

use graph::*;

const MOVES: [Point;6] = [Point(2,0), Point(-2,0), Point(0,2), Point(0,-2), Point(-2,-2), Point(2,2)];

fn main ()  {
    let dim = 7;
    println!("Hello World");
    let mut g = Graph::triangle_from_size(dim);
    let mut id = 1;
    for i in 1..(dim as isize) {
        for j in 0..i+1 {
            if i == 2 && j == 1 {
                // continue;
            }
            g.add_piece(Point(i,j), id);
            id += 1;
        }
    }
    
    let mut target = Graph::triangle_from_size(dim);
    target.add_piece(Point(0,0),1);
    let ans = solve_triangle(g,&target,dim as isize);
    for step in ans.unwrap().iter().rev() {
        println!("{}", step);
    }
}

fn solve_triangle(current: Graph, target: &Graph,dim: isize) -> Option<Vec<Graph>> {
    
    /*
    if current == *target {
        return Some(vec![current]);
    }
    */
    
    
    if current.pieces() <= 1 {
        return Some(vec![current]);
        // return None;
    }
    
    for i in 0..dim {
        for j in 0..i+1 {
            if let &None = current.contains_piece(Point(i,j)) {
                continue;
            }
            for m in &MOVES {
                let p = Point(i,j);
                if let Some(g) = current.move_piece(p, p + *m) {
                    let solution = solve_triangle(g,target,dim);
                    if let Some(mut v) = solution {
                        v.push(current);
                        return Some(v);
                    }
                }
            }   
        }
    }
    None
}