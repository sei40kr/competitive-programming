package main

// trapping-rain-water.go
// author: Seong Yong-ju ( @sei40kr )

import "fmt"

func trap(height []int) int {
	len := len(height)
	if len == 0 {
		return 0
	}

	dp := make([]int, len)
	dp[0] = height[0]
	dp[len-1] = height[len-1]

	for i := 1; i < len-1; i++ {
		x := height[i]
		if x < dp[i-1] {
			x = dp[i-1]
		}

		dp[i] = x
	}
	for i := len - 2; 1 <= i; i-- {
		x := height[i]
		if x < dp[i+1] {
			x = dp[i+1]
		}

		if x < dp[i] {
			dp[i] = x
		}
	}

	ans := 0
	for i, x := range dp {
		ans += x - height[i]
	}

	return ans
}

func main() {
	fmt.Printf("%d", trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1}))
}
