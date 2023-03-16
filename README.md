Every thing here is base on https://github.com/antirez/aocla
Objective : understand what has been written by antirez and why not learn nom and tui

## branching
working branch today : align_try_list
align_try_list <- align <- main

Copy of main before change on align. Objective to be more align with the main version. 
beforepurge <- main 

## Aocla overview

Aocla is a very simple language, more similar to Joy than to FORTH (higher level). It has a total of six datatypes:

* Lists: `[1 2 3 "foo"]`
* Symbols: `mysymbol`, `==` or `$x`
* Integers: `500`
* Booleans: `#t` or `#f`
* Tuples: `(x y z)`
* Strings: `"Hello World!\n"`

## Our first program



    10 20 (x y)

    5 (x) $x $x *

    '(a b c) 

## Working with lists

    []

    [1 2 3] 
    [1 2 3 4] 
    [1 2 3 4 5] 



    [1 2 3] [4 5 6] cat
    [1 2 3 4 5 6]
    
    [1 2 3] first
    1

    [1 2 3] rest
    [2 3]

There is, of course, map:

    aocla> [1 2 3] [dup *] map
    [1 4 9]

    [(l f) // list and function to call with each element.
        $l len (e)  // Get list len in "e"
        0 (j)       // j is our current index
        [$j $e <] [
            $l $j get@  // Get list[j]
            $f upeval   // We want to evaluate in the context of the caller
            $j 1 + (j)  // Go to the next index
        ] while
    ] 'foreach def


## Conditionals


The words `if` and `ifelse` do what you could imagine:

    5 (a)
    5
    [$a 2 >] ["a is > 2" printnl] if
    a is > 2


    10 [dup 0 >] [dup printnl 1 -] while
    10
    9
    8
    7
    6
    5
    4
    3
    2
    1


    (a _ b) $_ $a $b +
    2 4 

## Evaluating lists


    5 [dup dup dup] eval
    5 5 5 5


    [(n l)
        [$n 0 >]
        [$l eval $n 1 - (n)]
        while
    ] 'repeat def


    3 ["Hello!" printnl] repeat
    Hello!
    Hello!
    Hello!

## Eval and local variables

There is a problem with the above implementation of `repeat`, it does
not mix well with local variables. The following program will not have the expected behavior:

    10 (x) 3 [$x printnl] repeat
    Unbound local var: '$x' in eval:0  in unknown:0


the `repeat` procedure using `upeval`:

    [(n l)
        [$n 0 >]
        [$l upeval $n 1 - (n)]
        while
    ] 'repeat def

After the change, it works as expected:

    10 (x) 3 [$x printnl] repeat
    10
    10
    10


    [ (p v) // Procedure, var.
        []                      // Accumulate our program into an empty list
        '$ $v cat swap ->       // Push $<varname> into the stack
        1 swap ->               // Push 1
        '+ swap ->              // Call +
        $v [] -> make-tuple swap -> // Capture back value into <varname>
        [] ->                       // Put all into a nested list
        'upeval swap ->             // Call upeval against the program
        $p def // Create the procedure  // Bind to the specified proc name
    ] 'create-incrementing-proc def

Basically calling `create-incrementing-proc` will end generating
a list like that (you can check the intermediate results by adding
`showstack` calls in your programs):

    [[$x 1 + (x)] upeval]

And finally the list is bound to the specified symbol using `def`.

I believe the Fibonacci implementation written in Aocla, versus the implementation written in other stack-based languages, is quite telling about the jump forward in readability and usability provided by this simple feature:

    [(n)
        [$n 1 <=]
        [
            $n
        ]
        [
            $n 1 - fib
            $n 2 - fib
            +
        ] ifelse
    ] 'fib def

    10 fib
    printnl

