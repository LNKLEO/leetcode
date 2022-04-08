impl Solution {
    pub fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut answer : Vec<Vec<i32>> = Vec::new();
        let mut next : Vec<Vec<i32>> = vec![vec![1, grid[start[0] as usize][start[1] as usize], start[0], start[1]]];
        let mut distance = 1;        
        let w = grid.len();
        let h = grid[0].len();
        let mut walked : Vec<Vec<bool>> = vec![vec![false; h]; w];
        walked[start[0] as usize][start[1] as usize] = true;
        while !next.is_empty() {
            let mut record : Vec<i32> = Vec::new();
            record.append(&mut next[0]);
            if distance < record[0] {
                distance = record[0];
                next[0].append(&mut record);
                next.sort();
                record.append(&mut next[0]);
            }
            next.remove(0);
            if record[1] == 0 {
                continue;
            } else if (record[1] > 1) && (pricing[0] <= record[1]) && (record[1] <= pricing[1]) {
                answer.push([record[2], record[3]].to_vec());
                if answer.len() == k as usize {
                    return answer;
                }
            }
            if (record[2] > 0) && (!walked[(record[2] - 1) as usize][record[3] as usize]) {
                walked[(record[2] - 1) as usize][record[3] as usize] = true;
                next.push([distance + 1, grid[(record[2] - 1) as usize][record[3] as usize], record[2] - 1, record[3]].to_vec());
            }
            if (record[3] > 0) && (!walked[record[2] as usize][(record[3] - 1) as usize]) {
                walked[record[2] as usize][(record[3] - 1) as usize] = true;
                next.push([distance + 1, grid[record[2] as usize][(record[3] - 1) as usize], record[2], record[3] - 1].to_vec());
            }
            if (record[3] < (h - 1) as i32) && (!walked[record[2] as usize][(record[3] + 1) as usize]) {
                walked[record[2] as usize][(record[3] + 1) as usize] = true;
                next.push([distance + 1, grid[record[2] as usize][(record[3] + 1) as usize], record[2], record[3] + 1].to_vec());
            }
            if (record[2] < (w - 1) as i32) && (!walked[(record[2] + 1) as usize][record[3] as usize]) {
                walked[(record[2] + 1) as usize][record[3] as usize] = true;
                next.push([distance + 1, grid[(record[2] + 1) as usize][record[3] as usize], record[2] + 1, record[3]].to_vec());
            }
        }
        return answer;
    }
}
