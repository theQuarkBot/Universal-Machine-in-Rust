use std::ops::Index;



const INITIAL_NUM_SEGMENTS: usize = 100;

pub struct UmSegments {
    segments: Vec<Vec<u32>>,
    free_list: Vec<usize>,
}

impl Default for UmSegments {
    fn default() -> Self {
        UmSegments {
            segments: Vec::with_capacity(INITIAL_NUM_SEGMENTS),
            free_list: Vec::with_capacity(INITIAL_NUM_SEGMENTS),
        }
    }
}

impl UmSegments {
    pub fn new_with_program(program: Vec<u32>) -> UmSegments {
        let mut um_segments = UmSegments::default();
        um_segments.segments.push(program);
        um_segments
    }

    /// Allocates a new segment of the given size and returns the index of the
    /// segment.
    /// 
    /// The allocated segment is initialized to all zeros.
    pub fn alloc(&mut self, size: usize) -> usize {
        let new_segment = vec![0; size];
        if let Some(index) = self.free_list.pop() {
            self.segments[index] = new_segment;
            index
        } else {
            self.segments.push(new_segment);
            self.segments.len() - 1
        }
    }

    /// Frees the segment at the given index.
    pub fn free(&mut self, index: usize) {
        self.segments[index].clear();
        self.free_list.push(index);
    }

    /// Returns a reference to the segment at the given index.
    /// 
    /// # Panics
    ///     Panics if the index is out of bounds.
    pub fn get(&self, index: usize) -> &Vec<u32> {
        &self.segments[index]
    }
}