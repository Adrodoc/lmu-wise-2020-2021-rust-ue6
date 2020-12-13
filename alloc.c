#include <stdint.h>
#include <stdlib.h>

void *heapalloc(size_t size)
{
	return malloc(size);
}

void heapfree(void *ptr)
{
	free(ptr);
}

uint64_t rdtsc()
{
	unsigned int lo, hi;
	__asm__ __volatile__("rdtsc"
						 : "=a"(lo), "=d"(hi));
	return ((uint64_t)hi << 32) | lo;
}
