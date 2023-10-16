#include <stdio.h>
#include <stdlib.h>

typedef struct node {
	int value;
	struct node* left;
	struct node* right;
} node;

int INPUT[] = {3,8,9,5,4,7,6,1,2};
int LEN = 9;

node* create_circle() {
	node* head = (node*) malloc(sizeof(node));

	head->value = INPUT[0];
	head->right = (node*) malloc(sizeof(node));

	node* prev = head;
	node* iter = head->right;

	for (size_t i = 1; i < LEN; i++) {
		iter->value = INPUT[i];
		iter->left = prev;

		iter->right = (node*) malloc(sizeof(node));

		prev = iter;
		iter = iter->right;
	}

	prev->right = head;
	head->left = prev;

	return head;
}

void part_one(node* head) {
	node* iter = head;
	node* store[3] = {NULL, NULL, NULL};

	for (int i = 0; i < 100; ++i) {
		node* next = iter->right;

		for (int j = 0; j < 3; j++) {
			store[j] = next;
			next = next->right;
		}

		iter->right = next;
		next->left = iter;

		int searching = iter->value - 1;
		node* found = NULL;

		while (found == NULL) {
			if (searching < 0)
				searching = 9;

			for (int j = 0;  j < LEN - 3; j++) {
				if (next->value == searching)
					found = next;
				if (found != NULL)
					break;
				next = next->right;
			}

			searching--;
			next = next->right;
		}

		found->right = store[0];
		store[0]->left = found;

		next->left = store[2];
		store[2]->right = next;	

		iter = iter->right;
	}

	while (iter->value != 1) {
		iter = iter->right;
	}
	iter = iter->right;

	printf("Part one: ");
	for (int i = 1; i < LEN; i++) {
		printf("%d", iter->value);
		iter = iter->right;
	}
	printf("\n");
}

void part_two(node* head) {
	// TODO: finish part two
}

int main(void) {
	node* head = create_circle();

	part_one(head);
	part_two(head);
	return 0;
}
