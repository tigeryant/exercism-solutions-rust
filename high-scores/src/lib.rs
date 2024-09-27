#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        // todo!("Construct a HighScores struct, given the scores: {scores:?}")
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        // todo!("Return all the scores as a slice")
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // todo!("Return the latest (last) score")
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // todo!("Return the highest score")
        self.scores().iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // todo!("Return 3 highest scores")
        // sort the scores, take the top three, return a vec owned by the caller
        let mut res_vec = self.scores.to_vec();
        res_vec.sort_unstable_by(|a, b| b.cmp(a));
        res_vec.truncate(3);
        res_vec
    }
}
