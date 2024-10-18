# OS-Checker
Ever had a teacher give you a 40 000 lines long text file that should verify the ouptut of your programming assigment for a certain input?

Os-Checker stands for output stream checker and is meant to act like the `assert-line` feature of the bats testing framework, but easier to use.

### Usage
See program usage:
```bash
cargo -- run -h
```
Example:
```bash
cargo run -- /path/to/my_program [my_program_args ...] /path/to/my_program_expected_output.txt [OPTIONS]
```
![](https://i.imgur.com/IMVbk0u.png)  
`--no-space-format` option
![](https://i.imgur.com/IB03IKR.png)
![](https://i.imgur.com/1WrirKf.png)  
`--no-line-order` option
![](https://i.imgur.com/doOw8cP.png)
![](https://i.imgur.com/EZAWrzQ.png)
