use std::ops::{Add,Sub};
use std::fmt;

#[derive(Clone)]
pub struct Graph {
    rows: Vec<Row>,
    count: usize,
}

static c: Option<Piece> = None;

impl Graph {
    #[allow(dead_code)]
    pub fn new() -> Graph {
        Graph { rows: Vec::new(), count
   :0 }
    }
    
    #[inline]
    pub fn pieces(&self) -> usize {
        self.count
    }
    
    pub fn triangle_from_size(size: usize) -> Graph {
        let mut g = Graph { rows: Vec::new(), count: 0};
        for i in 1..(size+1) {
            g.rows.push(vec![None;i]);
        }
        g
    }
    
    pub fn add_piece(&mut self, pos: Point, value: usize) {
        let Point(row,column) = pos;
        self.rows[row as usize][column as usize] = Some(Piece(value));
        self.count += 1;
    }
    
    pub fn in_bounds(&self,pos: Point) -> bool {
        let Point(x,y) = pos;
        if y < 0 || x < 0 {
            return false;
        }
        if y > x {
            return false;
        }
        return (x as usize) < self.rows.len();
    }
    
    pub unsafe fn contains_piece_unchecked(&self, pos: Point) -> Option<Piece> {
        let Point(row, col) = pos;
        self.rows.get_unchecked(row as usize).get_unchecked(col as usize).clone()
    }
    
    
    pub fn contains_piece(&self, pos: Point) -> &Option<Piece> {
        let Point(row,column) = pos;
        self.rows.get(row as usize)
            .and_then(|r| r.get(column as usize))
            .unwrap_or(&c)
    }
    
    #[allow(dead_code)]
    pub fn take(&mut self, pos: Point) -> Option<Piece> {
        if self.in_bounds(pos) {
            unsafe { self.rows
                .get_unchecked_mut(pos.0 as usize)
                .get_unchecked_mut(pos.1 as usize).take() 
            }
        } else {
            None
        }
    }
    
    fn take_unchecked(&mut self, pos: Point) -> Option<Piece> {
        unsafe { self.rows
            .get_unchecked_mut(pos.0 as usize)
            .get_unchecked_mut(pos.1 as usize).take() 
        }
    }
    
    // Provides a functional interface for interacting with count.
    // Creates a new graph with the count moved and removed accordingly.
    pub fn move_piece(&self,old_pos: Point, new_pos:Point) -> Option<Graph> {        
        if !self.in_bounds(new_pos) {
            return None;
        }
        if let Some(_) = unsafe {self.contains_piece_unchecked(new_pos)} {
            return None;
        }
        if let &Some(_) = self.contains_piece(old_pos) {
            let diff = new_pos - old_pos;
            match diff {
                Point(x@2,y@0) | Point(x@-2,y@0) |
                Point(x@0,y@2) | Point(x@0,y@-2) | 
                Point(x@2,y@2) | Point(x@-2,y@-2) => {
                    let sign_x = get_sign(x);
                    let sign_y = get_sign(y);
                    let p = Point(sign_x,sign_y) + old_pos;
                    if let None = unsafe {self.contains_piece_unchecked(p)} {
                        return None;
                    }
                    // Delete the piece that was just jumped over.
                    let mut temp = self.clone();
                    temp.take_unchecked(p);
                    let piece = temp.take_unchecked(old_pos).unwrap();
                    temp.add_piece(new_pos,piece.0);
                    temp.count -= 2;
                    Some(temp)
                },
                _ => {
                    None
                }
            }
        } else {
            None
        }
    }
    
    // Returns false if fails.
    #[allow(dead_code)]
    pub fn move_piece_mut(&mut self, old_pos: Point, new_pos: Point) -> bool {
        let Point(x_1,y_1) = old_pos;
        let Point(x_2,y_2) = new_pos;
        if !self.in_bounds(new_pos) {
            return false;
        }
        if let &Some(_) = self.contains_piece(new_pos) {
            return false;
        }
        if let &Some(_) = self.contains_piece(old_pos) {
            let diff = Point(x_2 - x_1, y_2 - y_1);
            match diff {
                Point(x@2,y@0) | Point(x@-2,y@0)
                | Point(x@0,y@2) | Point(x@0,y@-2) 
                | Point(x@2,y@2) | Point(x@-2,y@-2) => {
                    let sign_x = get_sign(x);
                    let sign_y = get_sign(y);
                    let p = Point(sign_x,sign_y) + old_pos;
                    if let &None = self.contains_piece(p) {
                        return false;
                    }
                    // Delete the piece that was just jumped over.
                    self.take(p);
                    let piece = self.take(old_pos).unwrap();
                    self.add_piece(new_pos,piece.0);
                    self.count
                -= 1;
                    true
                },
                _ => {
                    false
                }
            }
        } else {
            false
        }
    }
}

fn get_sign (i: isize) -> isize {
    match i {
        0 => 0,
        n if n > 0 => 1,
        _ => -1
    }
}

impl PartialEq for Graph {
    fn eq(&self, rhs: &Graph) -> bool {
        if self.rows.len() != rhs.rows.len() {
            return false;
        }
        
        if self.count != rhs.count {
            return false;
        }
        
        for (row1,row2) in self.rows.iter().zip(rhs.rows.iter()) {
            if row1.len() != row2.len() {
                return false;
            }
            for (col1, col2) in row1.iter().zip(row2.iter()) {
                if col1.is_some() && col2.is_none() {
                    return false;
                }
                if col1.is_none() && col2.is_some() {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Graph {}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        for row in &self.rows {
            for _ in 0..(self.rows.len()-i) {
                try!(write!(f, "  "));
            }
            i += 1;
            for col in row {
                if let &Some(Piece(id)) = col {
                    try!(write!(f,"[{: >#2}] ",id));
                } else {
                    try!(write!(f,"[  ] "));
                }
            }
            try!(writeln!(f,""));
        }
        writeln!(f,"")
    }
}

type Row = Vec<Option<Piece>>;

#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Piece(pub usize);

#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Point(pub isize,pub isize);

impl Add for Point {
    type Output = Point;
    
    fn add(self, other:Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Point;
    
    fn sub(self, other:Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}