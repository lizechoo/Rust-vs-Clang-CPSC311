#include <stdio.h>
int main()
{
   // printf() displays the string inside quotation
   // printf("Hello, World!");
   // return 0;
   int x;
   int y; 
   
   x = 7;
   y = 9;
   printf("[1] x is %d \n", x);
   printf("[1] y is %d \n", y);
   
   int *p;
   p = &x;
   printf("[1] p is %d \n", p);
   printf("[1] *p is %d \n", *p);
   
   // modify *p 
   *p = 8;
   //x = 8;
   printf("[2] p is %d \n", p);
   printf("[2] *p is %d \n", *p);
   printf("[2] x is %d \n", x);
   printf("[2] y is %d \n", y);
}