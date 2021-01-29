# Turning Unison into chicken scheme! What a concept.

Here's how you do it:

```
$ cargo run --release -- pack-all-chicken ~/.unison/v1/terms some_name.space ./outfile.scm
```

That will pack up a whole namespace into outfile.scm.

You can then run the file with
```
$ csi -s outfile.scm
```
or compile it to native code with
```
$ csc outfile.scm
```

## Next steps

- [ ] allow you to specify a single term (program) to run
- [ ] run all the tests in all libraries (not just the runtime tests)
- [ ] benchmark against current unison runtime(?)
- [ ] provide a cli flag for "run tests"
- [x] try it with spock to generate js? (update: I tried it, spock seems *super* lots of glaring bugs that prevented me from running things 😢)