#include <stdio.h>
#include <stdlib.h>

typedef struct node {
	int val;
	struct node* prev;
	struct node* next;
} node;

const int input[] = {1,2,-3,3,-2,0,4};
enum { N = (sizeof input / sizeof input[0]) };

node circle[N];
node* zero;

void build_circle() {
	for (int i = 0; i < N; i++) {
		if (input[i] == 0) {
			zero = &circle[i];
		}
		circle[i].val = input[i];
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
		for (; d > 0; d--) {
			p = n;
			n = n->next;
		}
		for (; d < 0; d++) {
			n = p;
			p = p->prev;
		}

		p->next = &circle[i];
		circle[i].next = n;
		circle[i].prev = p;
		n->prev = &circle[i];
	}
}

int main(void) {
	build_circle();
	mix_circle();

	node* iter = zero;
	int part_one = 0;

	for(int t = 0; t < 3; t++) {
		for (int i = 0; i < 1000; i++) {
			iter = iter->next;
		}
		part_one += iter->val;
	}

	printf("%i \n", part_one);
	return 0;
}
