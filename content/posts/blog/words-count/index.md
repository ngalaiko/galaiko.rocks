---
title: "Words Count"
tags: [
    "algorithm",
    "go",
]
categories: [
    "Blog",
]
date: 2019-05-11T11:01:45+02:00
---

Last week I participated in a small competiotion at work. The goal was to calculate the
top 10 the most common words in a file and print them with number of occurences. For
testing we used export from HackerNews comments from 2012 until today, and that file
was 4GB.

I will describe what optimizations I used to solve the problem from the most to the least
obvious.

Solution was measured on MacBook Pro 2018 with 2,2 GHz Intel Core i7 CPU and 32 GB RAM.

## Generic solution

The most simple way to solve this is to iterate over the file, save words to a map with
number of occurences, then iterate over the map and print top 10 most common words.

```go
func countWords(fileData []byte) {
    words := map[string]uint{}

    wordBuf := make([]byte, 64)
    wordPos := 0
    for _, c := range fileData {
            switch {
            case c >= 'A' && c <= 'Z':
                    c += 32
                    fallthrough
            case c >= 'a' && c <= 'z':
                    if wordPos == maxLen {
                            continue
                    }
                    wordBuf[wordPos] = c
                    wordPos++
            default:
                    if wordPos == 0 {
                        continue
                    }

                    words[string(wordBuf[:wordPos])]++
                    wordPos = 0
            }
            words[string(wordBuf[:wordPos])]++
    }
}
```

This will make a map with number of occurences of each English word in the file.

It takes ~1min to make this map.

After that keys needs to be sorted:
```go
freq := make([]*string, 2<<25)
for word, count := range words {
    wCopy := word
    freq[count] = &wCopy
}

done := 0
for i := uint64(len(freq) - 1); i > 0 && done < 10; i-- {
    if freq[i] == nil {
        continue
    }
    fmt.Printf("%s: %d\n", *freq[i], i)
    done++
}
```

This is a terrible way of sorting anything in real life, but it words fast, and that's
exactly what we need in out case. It took ~113ms

## Reading file concurrently

The next step is to assume that it could go faster if file is processed in batches
concurrently. That is true, but it will also create [data races](https://galaiko.rocks/posts/blog/go-data-races/).
To solve it, I will use this implementation of a map with atomic write.

Another way to go from here is to save results from each thread to it's own map and
then merge maps.

```go
func fromFile(filepath string, batchSize int64, tk *count.Stream) error {
        file, err := os.Open(filepath)
        if err != nil {
                return fmt.Errorf("failed to read `%s`: %s", filepath, err)
        }
        defer file.Close()

        info, err := file.Stat()
        if err != nil {
                return fmt.Errorf("failed to stat `%s`: %s", filepath, err)
        }

        all := info.Size() / batchSize
        wg := &errgroup.Group{}
        for i := int64(0); i < all; i++ {
                i := i
                // NOTE: read concurrently and process in batch
                wg.Go(func() error {

                        buff := make([]byte, batchSize)

                        off, err := file.ReadAt(buff, batchSize*i)
                        switch err {
                        case nil:
                        case io.EOF:
                                return nil
                        default:
                                return err
                        }

                        processBatch(buff[:off], tk)

                        return nil
                })
        }

        return wg.Wait()
}
```

I was using `2<<19-1` as a `batchSize`. This is based on test runs and nothins else.

`tk` here contains an instance of [github.com/cornelk/hashmap](https://github.com/cornelk/hashmap/)

## Law of Large numbers

The last optimization, and the biggest one I've made is not completely about programming.

There is this theorem in probablility theory that says:

>  the average of the results obtained from a large number of trials should be close
to the expected value, and will tend to become closer as more trials are performed

That means that if you take a large file with English text and count top 10 words in there,
the top will always look almost the same.

Next step here is to google [most common words in English](https://en.wikipedia.org/wiki/Most_common_words_in_English)
and use this list to decrease number of words we count.

By doing that, amount of writes to the map will dramaticaly decrease, what will allow
to spend less time waiting for write access to the map.

After implementing this, processing time is ~25s

You can find full code on [Github](https://github.com/ngalaiko/words)
