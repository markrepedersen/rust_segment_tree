use std::io;
use std::io::BufRead;

pub mod data_structures {
    impl Add for (usize, usize, usize) {
	fn add(self, other: (usize, usize, usize)) -> (usize, usize, usize) {
	    (self.0 + other.0, self.1 + other.1, self.2 + other.2);
	}
    }
    
    pub struct seg_tree {
	data: Vec<(usize, usize, usize)>,
	lazy: Vec<(usize, usize, usize)>,
	n: usize,
	root: usize,
    }

    impl seg_tree {
	const left_child_index: usize = 2*index;
	const right_child_index: usize = left_child_index + 1;
	
	pub fn new(n: usize) -> seg_tree {
	    let arr: Vec<(usize, usize, usize)> = vec![(0,0,0); n];
	    let arr_lazy: Vec<(usize, usize, usize)> = vec![(0,0,0); n];
	    
	    return seg_tree {
		data: arr,
		lazy: arr_lazy,
		n: n,
		root: 1,
	    }
	}

	pub fn push(self, i: usize) {
	    self.data[]
	}

	fn update(self, val: (usize, usize, usize), x: usize, y: usize, index: usize, left: usize, right: usize) {
	    let mid: usize = (right - left)/2 + 1;
	    if x <= left && right <= y {
		self.data[index] += val;
		self.lazy[index] += val;
	    } else if x < r && l < y {
		self.push(index);
		self.update(val, x, y, left_child_index, left, mid);
		self.update(val, x, y, right_child_index, mid, right);
		self.data[index] = self.data[left_child_index] + self.data[right_child_index];
	    }
	}

	pub fn update(self, val: (usize, usize, usize), x: usize, y: usize) {
	    update(val, x, y, self.root, 0, self.n);
	}
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buf = String::new();

    reader.read_line(&mut buf);
    let T: usize = buf.parse().unwrap();
      
    reader.read_line(&mut buf);
    let mut arr: Vec<usize> = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (N, M) = (arr[0], arr[1]);

    for _i in 0..M {
	reader.read_line(&mut buf);
	arr = buf.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
	let (p, l, r) = (arr[0], arr[1], arr[2]);
    }
}
