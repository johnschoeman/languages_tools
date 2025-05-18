#include <stdio.h>

// chp 3 : Pointers and Strings

char *my_strcpy(char *destination, const char *source) {
    char *p = destination;
    while (*source != '\0') {
        *p++ = *source++;
    }
    *p = '\0';

    return destination;
}

int main() {
    char strA[80] = "A string to be used for demo purposes";
    char strB[80];

    my_strcpy(strB, strA);
    puts(strA);
    puts(strB);

    return 0;
}

// chp 2 : Pointer types and Arrays

// int main() {
//     int my_array[] = {1, 2, 3, 4, 5};
//     int *ptr;
//
//     int i;
//     // ptr = &my_array[0];
//     ptr = my_array;
//
//     printf("\n\n");
//
//     for (i = 0; i < 6; i++) {
//       printf("my_array[%d] = %d   ", i, my_array[i]);
//       // printf("ptr + %d = %d\n", i, *(ptr + i));
//       printf("ptr + %d = %d\n", i, *(ptr++));
//       // printf("ptr + %d = %d\n", i, *(++ptr));
//     }
//
//     return 0;
// }

// chp 1 : What is a pointer?

// int main() {
//     j = 1;
//     k = 2;
//     ptr = &k;
//     printf("\n");
//     printf("j has the value %d and is stored at %p\n", j, (void *)&j);
//     printf("k has the value %d and is stored at %p\n", k, (void *)&k);
//     printf("ptr has the value %p and is stored at %p\n", ptr, (void *)&ptr);
//     printf("The value of the integer pointed to by ptr is %d\n", *ptr);
//     return 0;
// }
