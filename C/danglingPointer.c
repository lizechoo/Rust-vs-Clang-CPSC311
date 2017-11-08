#include <stdio.h>
#include <stdlib.h>

int main()
{
   // we try to create a dangling pointer
   
   int a[3] = {1, 2, 3};
   
   int *p = malloc(sizeof(int)*3);
   
   p = &a[0];
   
   printf("[1] a is %d %d %d\n", a[0], a[1], a[2]);
   printf("[1] *p is %d %d %d \n", *p, *(p+1), *(p+2));

   free(p);  // free memory for p
   p = NULL;   // comment this out will make the program runs (based on luck?)
   
   p[1] = 4; //invalid access through dangling pointer!
   
   printf("[2] a is %d %d %d\n", a[0], a[1], a[2]);
   //printf("[2] *p is %d %d %d \n", *p, *(p+1), *(p+2));
}