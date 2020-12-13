#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

void *heapalloc(size_t size)
{
	printf("Allocating memory: %lu\n", size);
	return malloc(size);
}

void heapfree(void *ptr)
{
	printf("Freeing memory... ");
	free(ptr);
	printf("done.\n");
}

uint64_t rdtsc()
{
	unsigned int lo, hi;
	__asm__ __volatile__("rdtsc"
						 : "=a"(lo), "=d"(hi));
	return ((uint64_t)hi << 32) | lo;
}
