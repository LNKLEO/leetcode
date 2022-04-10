impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut dishes = satisfaction.clone();
        dishes.sort_by(|a,b| b.cmp(a));
        let mut answer = 0;
        let mut partial = 0;
        for dish in dishes {
            partial += dish;
            if partial < 0 {
                return answer;
            }
            answer += partial;
        }
        return answer;
    }
}
