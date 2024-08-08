# Time Complexity

Time complexity is the time required to execute an instruction.

Time complexity can also be described by estimating the number of elementary steps performed by an algorithm to complete its execution.
         
Asymptotic notations are used to find the time complexity.

Asymptotic notations are the mathematical way to represent Time complexity.          

 1) Big oh [O]:-

* It is the maximum time taken by an algorithm to consider or execute all the values.
* If we consider two functions g(n) and f(n), then it is used to define if the set of functions is going to grow slower than or at the same rate with respect to the expression.
* If f(n) and g(n) are the functions defined on a positive integer number 'n' , then

    *f(n) = O(g(n))  [n = number of inputs]*

* If n is big-oh of g of n if and only if positive constants c and n exist , such that 
 
    *f(n) <= Cg(n) <= n*

* It means that for large amount of data , f(n) will grow no more that a constant g(n).

Hence, g provides upper bound.


2) Omega [ Ω ]:-
                                                                            
* It is used to find the best case of an algorithm's time complexity.

* If we consider two functions g(n) and f(n), then it is used to define if the set of functions is going to grow faster than or at the same rate with respect to the expression.

* This also elaborates on the minimum amount of time required by an algorithm considering all input values.

    *f(n)=ΩCg(n)    [n=number of input's]*

That means , at larger valuesof n , the lower bound of f(n) and g(n) , there exists positive constants c and n0 ,such that

*0<=Cg(n)<=f(n)     for all n >=n0*

Hence , f provieds lower bound.

3) Theta [Θ]:-

* It is used to find the average case of an algorithm's time complexity.

* If we consider two functions g(n) and f(n), then it is used to define if the set of functions is going to lie in both 'O' and 'Omega'.

* It is writen as ,

    *ΘCg(n)=f(n)* 

There exits positive constants c1 , c2 and n0 such that,

*0<=C1g(n)<=f(n)<=C2g(n)    for all n>=n0*
