func findPeakGrid(mat [][]int) []int {
	ascC := make([][]bool, 0, len(mat))
	ascR := make([][]bool, 0, len(mat[0]))
	for i := 0; i < len(mat); i++ {
		ascC = append(ascC, []bool{})
		ascR = append(ascR, []bool{})
		for j := 0; j < len(mat[i]); j++ {
			if i < len(mat)-1 {
				ascC[i] = append(ascC[i], mat[i+1][j] > mat[i][j])
			}
			if j < len(mat[i])-1 {
				ascR[i] = append(ascR[i], mat[i][j+1] > mat[i][j])
			}
			if (i == 0 || ascC[i-1][j]) && (j == 0 || ascR[i][j-1]) &&
				!((i < len(mat)-1) && ascC[i][j]) && !((j < len(mat[i])-1) && ascR[i][j]) {
				return []int{i, j}
			}
		}
	}
	return []int{}
}
