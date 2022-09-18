Autowalk is a windows-only program to enable autowalking in games that do not implement it written in Rust. It has been inspired from [Squad](https://joinsquad.com/) autowalk. 

## Table of contents
- [Table of contents](#table-of-contents)
- [Usage](#usage)
- [Settings](#settings)
- [Contributing](#contributing)
- [Copyright and license](#copyright-and-license)

## Usage

- Run autowalk.exe, ignore windows smartscreen to make it work. The code is here for you to review it is legit and it is not a malware.
- Double-tap the ALT key to start sending 'W' presses to the game, a single-tap will stop the autowalk. If you wish to change the interval and the trigger key check [Settings](#settings)

## Settings 
Use autowalk.exe -h to see the settings and set the parameters.
```
autowalk 0.1.0

USAGE:
    autowalk.exe [OPTIONS]

OPTIONS:
    -a, --autowalk-trigger-key <AUTOWALK_TRIGGER_KEY>
            Key to enable the autowalk check list in
            https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes, and convert
            hex to int base10 e.g. (0x12 = 18 = ALT key) [env: AUTOWALK_TRIGGER_KEY=] [default: 18]

    -d, --debug
            [env: DEBUG=]

    -h, --help
            Print help information

    -t, --trigger-interval-millis <TRIGGER_INTERVAL_MILLIS>
            Interval in milliseconds to trigger the autowalk between two key presses [env:
            TRIGGER_INTERVAL_MILLIS=] [default: 200]

    -V, --version
    
```

## Contributing 

Have a bug or a feature request? Issues and PRs are welcome! 😀

## Copyright and license
Code released under the MIT License.

Enjoy 🎉