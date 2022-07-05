package main

// reverse-nodes-in-k-group.go
// author: Seong Yong-ju ( @sei40kr )

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseKGroup(head *ListNode, k int) *ListNode {
	if head == nil || k < 1 {
		return nil
	}

	stack := make([]*ListNode, 0, k+1)
	node := head

	for node != nil && len(stack) < k+1 {
		stack = append(stack, node)
		node = node.Next
	}

	if len(stack) < k {
		// Do nothing
		return head
	}

	for i := k - 1; i >= 1; i-- {
		stack[i].Next = stack[i-1]
	}

	if k+1 <= len(stack) {
		stack[0].Next = reverseKGroup(stack[k], k)
	} else {
		stack[0].Next = nil
	}

	return stack[k-1]
}

func main() {
}
