#include <stdio.h>

#include "basics_config.h"
#include "sorting.h"

int main(void) {
	printf("(This is 'basics' version %d.%d)\n", VERSION_MAJOR, VERSION_MINOR);

	printf("Hello, world!\n");
	bubble_sort(NULL, 0);

	return 0;
}
