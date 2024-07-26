# <p align="center">GameMan</p>
> The GameBoy emulator that does not play around™

# What can the emulator actually run?
I am at a *very* early stage, this emulator will **not** run *any* game.  
Here's a list of things I need to implement

- [ ] CPU
    - [x] Barebones CPU for testing graphics
    - [ ] All opcodes implemented
    - [ ] All interrupts implemented
- [x] GPU
    - [x] Barebones and graphicless implementation
    - [x] Background rendering
    - [x] Window rendering
    - [x] Sprite rendering
- [ ] Input Handling
- [ ] GameBoy Color support
- [ ] SPU (Sound), I have yet to research this

Here's a list of roms and games that work on the emulator
- [dmg-acid2](https://github.com/mattcurrie/dmg-acid2)

# What makes this emulator different?
- It's **embeddable**, you can embed this into any UI framework, gtk, iced etc. This is not actually an emulator per se, it's the "backend", the "frontend" is up to you to implement, altough I will probably make one myself. As of now, you can try `cargo run --example debugger -- rom.gb` to get a primitive debugger, and `cargo run --example screen -- rom.gb` to see the actual gameboy screen.
- It will *probably* be **embedded** as well in the future, meaning you will be able to run this on a microcontroller
- Focused on **clean code**, this emulator's focus is neither speed or accuracy, if I delivered on this front is up to you, feel free to open an issue or a pull request if you think something can be improved
