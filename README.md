here is my everyday report of learning rust

Learning basic programming Syntax of the Rust.

Learning goal is to make stack web-application using Rust

using lyb like actix, actix-web, Dioxus

## what we know about Rust

Rust use borow and owning

evey varable has its lifetime inside a {}


imp:
  str:
    str is an immutable string slice, also known as a string literal or string reference.
    It's a view into a sequence of UTF-8 bytes stored elsewhere, such as in the data segment of the program's binary or on the heap.
    It cannot be mutated directly because it's immutable.
    str is commonly used for string literals and string slices.
    Example: let s: &str = "hello";

  String:
    String is a growable, heap-allocated string.
    It's mutable and can be modified or extended after creation.
    String owns its data, which is stored on the heap, and it has the ability to grow or shrink dynamically as needed.
    String is commonly used when you need to manipulate or modify strings at runtime.
    Example: let s: String = String::from("hello");



				STACK - HEAP JARGON
    --------------------------------------------------------------------

Rust in string case


let s1:String = String::from("abckaldfja")

stack                heap

creates              index  char
  ptr      ------->   [0]--->[a]
to The string[0]      [1]--->[b]
		      [2]--->[c]	



Everything is stored in heap except immutable constants. 
Stack can store imut arr.

Heap consists of string and vecots


as the value of string can grow or shrink dynamic memory 
allocation is required, done by heap. Not can directly
do it in stack.


OWNERSHIP IN RUST

let s1:string = String::from("hello")
     
      points
stack 		heap

(s1)
len=5
ptr,   ------->	h s1[0]
cap		e
		l
		l
		o

let s2 = s1;

s1 gets replaced by s2 and gets removed from stack as heap is faithfull to have only one owner
the stack data gets copied. Like numbers

whearas the String or Vector which stores data in the heap pointed by the mem in stack, 

no two thing can point towards at the same data in heap


	
when fn and variables gets stored in the stack they point towards values in the heap (if string or Vector, remember only unknown memory values vars gets stored in heap)

var passed in the funct, the variable data ownership gets passed down to func


	String variable = String::from("hello");

	
	take_ownership(str: variable);
	printf("{}", variable )		<--------------------------------error as the str has moved on. now the function (take_ownership is pointing towards the mem in the heap)

		
----> defined function	function take_owner(str:string) {.....}



		stack 			heap
			

		(s1)--------------->  HELLO		error as the same mem stack cant point towards same mem location the heap. 
				^-----|
				|
	      (my_func)---------^



GARBAGE COLLECTION IS WHEN THE THE VARIABLE GOES OUT SCOPE AND GETS COLLECTED IN THE GARBAGE COLLECTOR WHICH GETS EMPTIED
