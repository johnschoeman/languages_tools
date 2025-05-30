#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// chp 10: Pointers to Functions

#define MAX_BUF 256

long arr[10] = { 3, 7, 9, 2, 5, 1, 8, 4, 6, 0 };
char arr2[5][20] = {  "Mickey Mouse",
                      "Donald Duck",
                      "Minnie Mouse",
                      "Goofy",
                      "Ted Jensen" };

void bubble(void *p,
            int width,
            int N,
            int(*fptr)(void *, void *));
int compare_string(void *m, void *n);
int compare_long(void *m, void *n);

int main(void)
{
    int i;
    putchar('\n');

    for (i = 0; i < 10; i++) {
        printf("%ld ", arr[i]);
    }
    putchar('\n');
    for (i = 0; i < 5; i++) {
        printf("%s\n", arr2[i]);
    }

    bubble(arr, 8, 10, compare_long);
    bubble(arr2, 20, 5, compare_string);

    putchar('\n');

    for (i = 0; i < 10; i++) {
        printf("%ld ", arr[i]);
    }
    putchar('\n');
    for (i = 0; i < 5; i++) {
        printf("%s\n", arr2[i]);
    }

    return 0;
}

void bubble(
    void *p,
    int width,
    int N,
    int(*fptr)(void *, void *)
    )
{
    int i, j, k;
    unsigned char buf[MAX_BUF];
    unsigned char *bp = p;

    for (i = N-1; i >= 0; i--) {
        for (j = 1; j <= i; j++) {
          k = fptr((void *)(bp + width*(j-1)), (void *)(bp + j*width));
          if (k > 0) {
             memcpy(buf, bp + width*(j-1), width);
             memcpy(bp + width*(j-1), bp + j*width , width);
             memcpy(bp + j*width, buf, width);
            }
        }
    }
}

int compare_string(void *m, void *n) {
    char *m1 = m;
    char *n1 = n;
    return (strcmp(m1,n1));
}

int compare_long(void *m, void *n) {
  long *m1, *n1;
  m1 = (long *)m;
  n1 = (long *)n;
  return (*m1 > *n1);
}

// chp 9: Pointers and Dynamic Allocation of Memory

// // Method 1: Using malloc
// #define COLS 5
//
// typedef int RowArray[COLS];
// RowArray *rptr;
//
// int main(void) {
//     int nrows = 10;
//     int row, col;
//
//     rptr = malloc(nrows * COLS * sizeof(int));
//
//     for (row = 0; row < nrows; row++) {
//         for (col = 0; col < COLS; col++) {
//             rptr[row][col] = 17;
//         }
//     }
//
//     return 0;
// }

// Method 2

// int main(void) {
//     int nrows = 5;
//     int ncols = 10;
//     int row;
//     int **rowptr;
//     rowptr = malloc(nrows * sizeof(int *));
//     if (rowptr == NULL) {
//         puts("\nFailure to allocate room for row pointers.\n");
//         exit(0);
//     }
//
//     printf("\n\n\nIndex Pointer(hex)    Pointer(dec)   Diff.(dec)");
//
//     for (row = 0; row < nrows; row++) {
//          rowptr[row] = malloc(ncols * sizeof(int));
//          if (rowptr[row] == NULL) {
//            printf("\nFailure to allocate for row[%d]\n", row);
//            exit(0);
//          }
//          printf("\n%d     %p  %d", row, rowptr[row], *rowptr[row]);
//          if (row > 0) {
//              printf("            %d", (int)(rowptr[row] - rowptr[row - 1]));
//          }
//     }
//
//     return 0;
// }

// chp 8: Pointers to Arrays

// int (*p1d)[10]; // p1d is a pointer to an array of 10 integers
// int *p1d[10];   // p1d is an array of 10 pointers to integers

// chp 7: More on Multi-Dimensional Arrays

// #define ROWS 5
// #define COLS 10
//
// void set_value(int m_array[][COLS])
// {
//     int row, col;
//     for (row = 0; row < ROWS; row++)
//     {
//         for (col = 0; col < COLS; col++)
//         {
//             m_array[row][col] = 1;
//         }
//     }
// }
//
// int main(void) {
//     int multi[ROWS][COLS];
//
//     set_value(multi);
//
//     return 0;
// }

// chp 6: Some more on Strings, and Arrays of Strings

// #define ROWS 5
// #define COLS 10
//
// int multi[ROWS][COLS];
//
// int main(void) {
//   int row, col;
//
//   for (row = 0; row < ROWS; row++) {
//     for (col = 0; col < COLS; col++) {
//       multi[row][col] = row * col;
//     }
//   }
//
//   for (row = 0; row < ROWS; row++) {
//     for (col = 0; col < COLS; col++) {
//       printf("\n%d %d:", row, col);
//       printf("%d ", multi[row][col]);
//       printf("%d ", *(*(multi + row) + col));
//     }
//   }
//
//   return 0;
// }

// chp 5: Pointers and Structures

// struct tag {
//     char lname[20];
//     char fname[20];
//     int age;
//     float rate;
// };
//
// struct tag my_struct;
// void show_name(struct tag *p);
//
// int main(void) {
//     struct tag *st_ptr;
//     st_ptr = &my_struct;
//
//     strcpy(my_struct.lname, "Smith");
//     strcpy(my_struct.fname, "John");
//     printf("Last name: %s\n", my_struct.lname);
//     printf("First name: %s\n", my_struct.fname);
//     my_struct.age = 25;
//     show_name(st_ptr);
//
//     return 0;
// }
//
// void show_name(struct tag *p) {
//   printf("Last name: %s\n", p->lname);
//   printf("First name: %s\n", p->fname);
//   printf("Age: %d\n", p->age);
// }

// chp 4 : More on Strings

// int my_strlen(const char str[]) {
//     int len = 0;
//     while (str[len] != '\0') {
//       len++;
//     }
//     return len;
// }
//
// char *my_strcat(char dest[], const char source[]) {
//     int dest_len = my_strlen(dest);
//     int i = 0;
//     while (source[i] != '\0') {
//       dest[dest_len + i] = source[i];
//       i++;
//     }
//     dest[dest_len + i] = '\0';
//     return dest;
// }
//
// char *my_strchr(const char str[], int c) {
//     while (*str != '\0') {
//       if (*str == c) {
//         return (char *)str;
//       }
//       str++;
//     }
//     return NULL;
// }
//
// char *my_strcpy(char dest[], const char source[]) {
//     int i = 0;
//     while (source[i] != '\0') {
//       dest[i] = source[i];
//       i++;
//     }
//     dest[i] = '\0';
//     return dest;
// }
//
// int main() {
//     char a[80] = "a";
//     char b[80] = "bbbb";
//     char c[80] = "cccc";
//
//     a[1] = 'x';
//     2[a] = 'y';
//
//     puts(a);
//
//     printf("Length of string: %d\n", my_strlen(a));
//     printf("Length of string: %zu\n", strlen(a));
//
//     printf("\n");
//     puts(strcat(b, a));
//     puts(my_strcat(c, a));
//
//     printf("\n");
//     // puts(strchr(a, 1));
//     char *ptr;
//     ptr = strchr(a, 'x');
//     printf("strchr(a, 1) = %s\n", ptr);
//
//     char *ptrB;
//     ptrB = my_strchr(a, 'x');
//     printf("my_strchr(a, 1) = %s\n", ptrB);
//
//     return 0;
// }

// chp 3 : Pointers and Strings

// char *my_strcpy(char *destination, const char *source) {
//     char *p = destination;
//     while (*source != '\0') {
//         *p++ = *source++;
//     }
//     *p = '\0';
//
//     return destination;
// }
//
// int main() {
//     char strA[80] = "A string to be used for demo purposes";
//     char strB[80];
//
//     my_strcpy(strB, strA);
//     puts(strA);
//     puts(strB);
//
//     return 0;
// }

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
