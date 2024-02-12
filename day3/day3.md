# Owning references vs Borrowing References

In day2 we mainly discussed about borrowing references because that concept
is more familiar to what we see in any other programming languages out there.
Since I showed most of the examples in C (a pointer can refer to any address!),
I did not stress on the point that we are using borrowing references.

But we are sure that any value in Rust has a statically determined compile time value type
and a lifetime type.
But in Rust we have ownership model, which means any value is owned by a variable.
If the owning variable goes out of scope(we discussed that with respect to function calls and stacks),
the lifetime of the value ends and it is not legal to access that value later.

## Dangling pointer

Is when we have a reference(borrowing referece) to a value but that value goes out of scope but the
reference still lives. Essentially we have a reference pointing to invalid memory.

Rust guarantees memory safety which means at compile time it makes sure that when the program runs
there will be no references pointing to invalid memory.

## Ownership Rule

Every value should have an owner variable except Arc types in which we have shared ownership.
And all the variables live in stack!

Now read the examples in [day2](../day2/day2.md) it will be clear why functions can only return
borrowed references which are already present in the input parameters. Because if the function
creates a new value it should have an owner which has a shorter lifetime than the function itself,
so you cannot return the borrowed reference to that owner, because once the function is out of scope
so is that owner and that borrowed reference is not valid.
