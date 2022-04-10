func waysToSplit(nums []int) int {
	count := 0
	partial := make([]int, len(nums))
	partial[0] = nums[0]
	for i := 1; i < len(nums); i++ {
		partial[i] = partial[i-1] + nums[i]
	}
	sum := partial[len(partial)-1]
	sr := 2 * sum / 3
	ir := len(nums) - 1
	for sr < partial[ir-1] {
		ir--
	}
	ilc := ir - 1
	ilf := ilc - 1
	for ir > 1 {
		sc := partial[ir-1] / 2
		sf := 2*partial[ir-1] - sum
		if ir-1 < ilc {
			ilc = ir - 1
		}
		for ilc > 0 && sc < partial[ilc-1] {
			ilc--
		}
		if ilc < ilf && ilc > 0 {
			ilf = ilc
		}
		for ilf > 0 && sf <= partial[ilf-1] {
			ilf--
		}
		if ilc < 1 {
			break
		}
		count += ilc - ilf
		count %= 1000000007
		ir--
	}
	return count
}
