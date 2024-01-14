const INITIAL_NUM_SEGMENTS: usize = 10000;

pub struct UmSegments {
    pub segments: Vec<Vec<u32>>,
    pub free_list: Vec<usize>,
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
    pub fn get(&mut self, index: usize) -> &mut Vec<u32> {
        &mut self.segments[index]
    }

    /// Replaces the segment at the given index with the given segment.
    ///
    /// # Panics
    ///    Panics if the index is out of bounds.
    pub fn replace(&mut self, index: usize, segment: Vec<u32>) -> Vec<u32> {
        self.segments.push(segment);
        self.segments.swap_remove(index)
    }
}