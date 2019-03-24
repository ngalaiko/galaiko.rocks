---
title: "Forced wrapper interface"
tags: [
    "go",
]
date: 2019-03-24T10:03:39+01:00
categories: [
    "Blog",
]
---

Let's take this interface as an example:

```go
type Block interface {
    Chain(Block) (string, error)
}
```

I want to make sure that input `Block` is always valid, and the output string is never empty if the error is
`nil`. How can I do that? There are a couple of options.

The first option is to rely on every implementation to validate it, but it means a boilerplate code that is easy
to miss.

Another option is to create another layer and move all of the logic there. Something like:

```go
type Chain interface {
    Add(Block) error
}

type Block interface {
    Hash() string
}
```

It's better, but I don't want to create another entity.

I can also create an implementation that wraps all other implementations and does all the checks:

```go
type check struct {
    i block
}

func Check(i Block) *check {
    return &check{
        i: i,
    }
}

func (c *check) Chain(b Block) (string, error) {
    hash, err := c.i.Chain(b)

    if hash == "" && err == nil {
        fmt.Println("err is nil, and hash is empty, do something here.")
    }

    return hash, err
}
```

There is still the same problem: it's easy to forget to wrap a custom implementation into the check and skip
validation.

Likely, I found a way to enforce it during the build time. To do that, `Block` interface and the `check`
implementation should be slightly changed:

```go
type block interface {
    Chain(Block) (string, error)
}

type Block interface {
    block

    p()
}
```

```go
func Check(i block) Block {
    return &check{
        i: i,
    }
}

type check struct {
    i block
}

func (c *check) Chain(b Block) (string, error) {
    hash, err := c.i.Chain(b)

    if hash == "" && err == nil {
        fmt.Println("err is nil, and hash is empty, do something here.")
    }

    return hash, err
}

func (c *check) p() {}
```

Now, it's impossible to implement `Block` interface, because there is no way to implement a private method.

The only way to do that is to implement all public methods and call `Check` function.

You can find full example code on [GitHub](https://github.com/ngalayko/examples/tree/master/protection)
