#include <stdio.h>
#include <stdlib.h>

const int input[] = {1,2,-3,3,-2,0,4};

typedef struct node {
	long long val;
	struct node* prev;
	struct node* next;
} node;

node circle[7]; // change to input len
node* zero;

void build_circle(long long key) {
	for (int i = 0; i < N; i++) {
		if (input[i] == 0) {
			zero = &circle[i];
		}
		circle[i].val = input[i] * key;
		circle[i].next = &circle[(i + 1) % N];
		circle[i].prev = &circle[(i - 1 + N) % N];
	}
}

void mix_circle() {
	for (int i = 0; i < N; i++) {
		node* p = circle[i].prev;
		node* n = circle[i].next;

		p->next = n;
		n->prev = p;

		int d = circle[i].val % (N - 1);
		for (; d > 0; --d) {
			p = n;
			n = n->next;
		}
		for (; d < 0; ++d) {
			n = p;
			p = p->prev;
		}

		p->next = &circle[i];
		circle[i].next = n;
		circle[i].prev = p;
		n->prev = &circle[i];
	}
}

long long solve(long long key) {
	build_circle(key);
	int t = (key > 10LL) ? 10 : 1;

	for (int i = 0; i < t; i++) {
		mix_circle();
	}

	node* iter = zero;
	long long sum = 0;

	for (int t = 0; t < 3; ++t) {
		for (int i = 0; i < 1000; ++i) {
			iter = iter->next;
		}
		sum += iter->val;
	}

	return sum;
}

int main(void) {
	printf("Part one: %lld\n", solve(1L));
	printf("Part two: %lld\n", solve(811589153LL));
	return 0;
}
