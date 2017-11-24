#include <stdio.h>
#include <stdlib.h>

int main()
{
   // we try to create a dangling pointer
   
   int a = 3;

   int *p1 = &a;
   int *p2 = &a;

   int *p3 = &*p2;

   printf("[1] a is %d \n", a);
   printf("[1] p1 is %d \n", p1);
   printf("[1] *p1 is %d \n", *p1);
   printf("[1] p2 is %d \n", p2);
   printf("[1] *p2 is %d \n", *p2);
   printf("[1] p3 is %d \n", p3);
   printf("[1] *p3 is %d \n\n", *p3);

   *p1 = 4;

   printf("[2] a is %d \n", a);
   printf("[2] *p1 is %d \n", *p1);
   printf("[2] *p2 is %d \n", *p2);
   printf("[1] *p3 is %d \n\n", *p3);

   *p2 = 6;
   printf("[3] a is %d \n", a);
   printf("[3] *p1 is %d \n", *p1);
   printf("[3] *p2 is %d \n", *p2);
   printf("[1] *p3 is %d \n\n", *p3);

   a = 10;
   printf("[4] a is %d \n", a);
   printf("[4] *p1 is %d \n", *p1);
   printf("[4] *p2 is %d \n", *p2);
   printf("[1] *p3 is %d \n\n", *p3);

}