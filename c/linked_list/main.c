#include <stdio.h>
#include <string.h>
#include <stdbool.h>

// chp 5: Pointers and Structures

struct tag {
    char lname[20];
    char fname[20];
    int age;
    float rate;
};

struct tag my_struct;
void show_name(struct tag *p);

int main(void) {
    struct tag *st_ptr;
    st_ptr = &my_struct;

    strcpy(my_struct.lname, "Smith");
    strcpy(my_struct.fname, "John");
    printf("Last name: %s\n", my_struct.lname);
    printf("First name: %s\n", my_struct.fname);
    my_struct.age = 25;
    show_name(st_ptr);

    return 0;
}

void show_name(struct tag *p) {
  printf("Last name: %s\n", p->lname);
  printf("First name: %s\n", p->fname);
  printf("Age: %d\n", p->age);
}

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
