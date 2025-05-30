# C Pointers

`int k`    - a variable
`&k`       - the address of k
`int *ptr` - a pointer variable
`*ptr`     - the value of what ptr points to

A variable is declared by giving it a type and a name (e.g. int k;)

A pointer variable is declared by giving it a type and a name (e.g. int *ptr) where the asterisk tells the compiler that the variable named ptr is a pointer variable and the type tells the compiler what type the pointer is to point to (integer in this case).

Once a variable is declared, we can get its address by preceding its name with the unary & operator, as in &k.

We can "dereference" a pointer, i.e. refer to the value of that which it points to, by using the unary '*' operator as in *ptr.

An "lvalue" of a variable is the value of its address, i.e. where it is stored in memory. The "rvalue" of a variable is the value stored in that variable (at that address).

---

```
struct tag *st_ptr;
st_ptr = &my_struct;

(*st_ptr).field = 5;
// same as
st_ptr->field = 5;
```
