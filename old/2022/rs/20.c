#include <stdio.h>
#include <stdlib.h>

typedef struct node {
	int value;
	struct node* left;
	struct node* right;
} node;

int BUF[6000];
int LEN = 0;

void read_file()
{
	FILE* fp = fopen("../input/20", "r");
	if (fp == NULL) return;

	char* line = NULL;
	size_t len = 0;
	ssize_t read;

	while ((read = getline(&line, &len, fp)) != -1)
	{
		BUF[LEN++] = atoi(line);
	}

	fclose(fp);
}

node* create_circle() 
{
	node* start = (node *) malloc(sizeof (node));

	start->value = BUF[0];
	start->right = (node*) malloc(sizeof (node));

	node* previous = start;
	node* current = start->right;

	for (int i = 1; i < LEN - 1; ++i) 
	{
		current->value = BUF[i];
		current->left = previous; 

		current->left->value = BUF[i - 1];
		current->right = (node*) malloc(sizeof(node));

		previous = current;
		current = current->right;
	}

	current->value = BUF[LEN - 1];
	current->left = previous;
	current->right = start;
	start->left = current;

	return start;
}

int main(void) 
{
	read_file();
	node* current = create_circle();

	int mod = LEN - 1;

	for (int i = 0; i < 8; ++i)
	{
		while (current->value != BUF[i])
			current = current->right;

		node* src = current;
		src->left->right = src->right;
		src->right->left = src->left;

		node* prv = src->left;
		node* nxt = src->right;

		int move = ((BUF[i] % mod) + mod) % mod;
		printf("MOVE: %i - BUF: %i \n", move, BUF[i]);

		for (int i = 0; i < move; i++)
		{
			prv = prv->right;
			nxt = nxt->right;
		}
		printf("L: %i - R: %i\n", prv->value, nxt->value);

		prv->right = src;
		src->left = prv;
		nxt->left = src;
		src->right = nxt;
	}

	while (current->value != 0)
		current = current->right;

	//printf("L: %i - R: %i \n", current->left->value, current->right->value);
	int part_one = 0;

	for (int i = 0; i < 3; ++i)
	{
		for (int j = 0; j < 1000; ++j)
			current = current->right;
		part_one += current->value;
	}

	printf("%i \n", part_one);


	return 0;
}
